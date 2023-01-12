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