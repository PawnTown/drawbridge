fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("./proto/cecpub.proto").unwrap();
  tauri_build::build();
  Ok(())
}
