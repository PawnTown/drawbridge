use rust_embed::RustEmbed;
#[derive(RustEmbed)]
#[folder = "public/win/"]
pub struct Asset;