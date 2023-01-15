use std::io::Write;
use std::process::Command;

pub fn create_temp_script(cmds: String, name: &str) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut tmp = std::env::temp_dir();
    tmp.push(format!("drawbridge_{}", name));

    let mut file = std::fs::File::create(&tmp)?;
    let cmds = cmds.replace("\n", "\r\n");
    let mut v: Vec<u16> = cmds.encode_utf16().collect();
    file.write_all(to_le(&mut v))?;
    file.sync_all()?;

    return Ok(tmp);
}

fn to_le(v: &mut [u16]) -> &[u8] {
    for b in v.iter_mut() {
        *b = b.to_le()
    }
    unsafe { v.align_to().1 }
}

pub fn run_temp_script(cmds: String, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tmp = create_temp_script(cmds, name)?;
    let tmp_fn = tmp.to_str().unwrap_or("");
    let res = Command::new("cmd")
        .args(&["/C", &tmp_fn])
        .status();
    //std::fs::remove_file(tmp)?;
    let _ = res?;
    Ok(())
}

pub fn create_exe_file(output: String, exec_path: String, id: String) -> Result<(), Box<dyn std::error::Error>> {
    return run_temp_script(
        format!(
            "
@if (@X)==(@Y) @end /* JScript comment
@echo off
setlocal

del %~n0.exe /q /s >nul 2>nul

for /f \"tokens=* delims=\" %%v in ('dir /b /s /a:-d  /o:-n \"%SystemRoot%\\Microsoft.NET\\Framework\\*jsc.exe\"') do (
    set \"jsc=%%v\"
)

if not exist \"%~n0.exe\" (
    \"%jsc%\" /nologo /out:\"%~n0.exe\" \"%~dpsfnx0\"
)

%~n0.exe  \"%jsc%\" %*
del /q /f %~n0.exe 1>nul 2>nul 
endlocal & exit /b %errorlevel%
*/

//https://github.com/npocmaka/batch.scripts/blob/master/hybrids/.net/bat2exe.bat
import System;
import System;
import System.IO;
import System.Diagnostics;

var binName=\"{}\";
var compilerLoc=arguments[1];

var temp=Path.GetTempPath();
var dt=(new Date()).getTime();
var tempJS=temp+\"\\2exe\"+dt+\".js\";

var toCompile=\"\r\n\
import System;\r\n\
import  System.Diagnostics;\r\n\
var pr=System.Diagnostics.Process.Start('cmd.exe','/c {} connect {}');\r\n\
pr.WaitForExit();\r\n\
\";

File.WriteAllText(tempJS, toCompile);
var pr=System.Diagnostics.Process.Start(compilerLoc,'/nologo /out:\"'+binName+'\" \"'+tempJS+'\"');
pr.WaitForExit();
File.Delete(tempJS);
        ", output, exec_path, id),
        "mk_shortcut.bat",
    );
}

pub fn create_shortcut(output: String, file_to_run: String, exec_path: String, id: String) -> Result<(), Box<dyn std::error::Error>> {
    return run_temp_script(
        format!(
            "
Set oWS = WScript.CreateObject(\"WScript.Shell\")
Set oLink = oWS.CreateShortcut(\"{}\")
oLink.TargetPath = \"{}\"
oLink.Arguments = \"connect {}\"
oLink.IconLocation = \"{}, 0\"
oLink.Save()
        ", output, file_to_run, id, exec_path),
        "mk_shortcut.vbs",
    );
}