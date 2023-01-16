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
    std::fs::remove_file(tmp)?;
    let _ = res?;
    Ok(())
}

pub fn create_bat_shortcut(exec_path: String, id: String, output: String) -> Result<(), Box<dyn std::error::Error>> {
    use directories::ProjectDirs;
    use fs::File;
    use std::fs;
    
    let mut bat_file;
    if let Some(proj_dirs) = ProjectDirs::from("im", "khad",  "drawbridge") {
        bat_file = proj_dirs.data_local_dir().to_path_buf();
        bat_file.push(format!("engine_{}.bat", id));

        let file = File::create(bat_file.clone());
        if let Err(_err) = file.unwrap().write_all(format!("\"{}\" connect {}", exec_path, id).as_bytes()) {
            std::process::exit(1);
        }
    } else {
        std::process::exit(1);
    }

    run_temp_script(
        format!(
            "
Set oWS = WScript.CreateObject(\"WScript.Shell\")
Set oLink = oWS.CreateShortcut(\"{}\")
oLink.TargetPath = \"{}\"
oLink.Arguments = \"connect {}\"
oLink.IconLocation = \"{}, 0\"
oLink.Save()
        ", output, bat_file.to_string_lossy(), id, exec_path),
        "mk_shortcut.vbs",
    ).unwrap();
    return Ok(());
}