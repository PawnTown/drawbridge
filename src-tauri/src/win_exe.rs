use std::io::Write;
use std::path::Path;
use std::process::Command;
use regex::Regex;
use std::fs;
use std::fs::metadata;

mod asset;

static NET_FRAMEWORK_PATH: &str = "C:\\Windows\\Microsoft.NET\\Framework";
static CS_COMPILER: &str = "C:\\Windows\\Microsoft.NET\\Framework\\v4.0.30319\\csc.exe";

fn get_cs_compiler_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let data = storage::load("settings".to_string()).unwrap();
    let settings: Value = serde_json::from_str(&data).unwrap();

    let mut custom_cs_compiler_path = settings["customCsCompilerPath"].as_str();
    match custom_cs_compiler_path {
        Some(path) => {
            let path = Path::new(path);
            if path.exists() {
                return Ok(path.to_path_buf());
            }
        },
        None => {
            // Try to find the compiler
            let re = Regex::new(r"\\v[\d\.]+$").unwrap();
            let paths = fs::read_dir(NET_FRAMEWORK_PATH).unwrap().map(|r| r.unwrap()).collect();
            paths.sort_by_key(|dir| dir.path());
            for path in paths {
                let path = path.unwrap().path();

                let md = metadata(path).unwrap();
                if !md.is_dir() {
                    continue;
                }

                let path_str = path.to_str().unwrap();
                if re.is_match(path_str) {
                    let subpaths = fs::read_dir(path_str).unwrap();
                    for subpath in subpaths {
                        let subpath = subpath.unwrap().path();
                        let subpath_str = subpath.to_str().unwrap();
                        if subpath_str.ends_with("csc.exe") {
                            return Ok(subpath);
                        }
                    }
                }
            }
        }
    }
}

fn create_cs_file(name: String, exec_path: String, id: String) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let re = Regex::new(r"([\\])").unwrap();
    let escaped_exec_path = re.replace_all(&exec_path, r"\$1");

    let code = format!("
using System;
using System.Diagnostics;

class Program
{{
    static Process process = new Process();

    static void Main(string[] args)
    {{
        process.EnableRaisingEvents = true;
        process.OutputDataReceived += new System.Diagnostics.DataReceivedEventHandler(process_OutputDataReceived);
        process.ErrorDataReceived += new System.Diagnostics.DataReceivedEventHandler(process_ErrorDataReceived);
        process.Exited += new System.EventHandler(process_Exited);

        process.StartInfo.FileName = \"cmd.exe\";
        process.StartInfo.Arguments = \"/c \\\"{}\\\" connect {}\";
        process.StartInfo.UseShellExecute = false;
        process.StartInfo.RedirectStandardError = true;
        process.StartInfo.RedirectStandardOutput = true;

        process.Start();
        process.BeginErrorReadLine();
        process.BeginOutputReadLine();

        process.WaitForExit();
    }}

    static void process_Exited(object sender, EventArgs e)
    {{
        Console.WriteLine(string.Format(\"process exited with code {{0}}\\n\", process.ExitCode.ToString()));
    }}

    static void process_ErrorDataReceived(object sender, DataReceivedEventArgs e)
    {{
        Console.WriteLine(e.Data);
    }}

    static void process_OutputDataReceived(object sender, DataReceivedEventArgs e)
    {{
        Console.WriteLine(e.Data);
    }}
}}
    ", escaped_exec_path, id);

    let mut tmp = std::env::temp_dir();
    tmp.push(format!("drawbridge_{}", name));

    let mut file = std::fs::File::create(&tmp)?;
    let code = code.replace("\n", "\r\n");
    file.write_all(code.as_bytes())?;
    file.sync_all()?;

    return Ok(tmp);
}

fn get_icon() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let mut tmp = std::env::temp_dir();
    tmp.push("drawbridge_icon.ico");

    let icon_data = asset::Asset::get("icon.ico").unwrap();

    let mut file = std::fs::File::create(&tmp)?;
    file.write_all(&icon_data.data)?;
    file.sync_all()?;

    return Ok(tmp);
}

pub fn compile_cs_file(exec_path: String, id: String, output: String) -> Result<(), Box<dyn std::error::Error>> {
    let outpath = Path::new(&output);
    let output_filename = outpath.file_name();

    let mut path_anc = outpath.ancestors();
    path_anc.next();
    let output_path = path_anc.next().unwrap().to_str().unwrap();

    let icon = get_icon().unwrap();
    let icon_path = icon.to_str().unwrap();

    let out = format!("-out:{}", output_filename.unwrap().to_str().unwrap());
    let icon = format!("-win32icon:{}", icon_path);
    println!("{}", output_path);
    println!("{}", out);

    let tmp = create_cs_file("shortcut".to_owned(), exec_path, id)?;
    let tmp  = tmp.to_str().unwrap_or("");
    let _res = Command::new(get_cs_compiler_path().unwrap())
        .args(&[&out, &icon, tmp])
        .current_dir(&output_path)
        .status();

    std::fs::remove_file(icon_path)?;
    std::fs::remove_file(tmp)?;

    Ok(())
}