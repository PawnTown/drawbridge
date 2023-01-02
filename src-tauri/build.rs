fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::compile_protos("./proto/proto.proto").unwrap();
  tauri_build::build();
  Ok(())
}
