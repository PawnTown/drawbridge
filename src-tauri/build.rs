fn main() -> Result<(), Box<dyn std::error::Error>> {
  tauri_build::build();
  tonic_build::compile_protos("./protos/pub_api.proto")?;
  Ok(())
}
