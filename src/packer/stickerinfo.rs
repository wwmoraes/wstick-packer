extern crate serde_derive;

use serde_derive::{Serialize, Deserialize};
use base64;
use anyhow::anyhow;

use crate::packer::types::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct StickerInfo {
  pub image_data: String,
  pub emojis: Vec<String>,
  #[serde(skip)]
  checksum: Option<md5::Digest>,
}

impl Default for StickerInfo {
  fn default() -> StickerInfo {
    StickerInfo {
      image_data: "".to_owned(),
      emojis: vec!(),
      checksum: None,
    }
  }
}

impl StickerInfo {
  fn new(data: String) -> StickerInfo {
    StickerInfo {
      checksum: Some(md5::compute(&data)),
      image_data: data,
      ..StickerInfo::default()
    }
  }

  pub fn checksum(&mut self) -> Result<md5::Digest> {
    if let None = self.checksum {
      self.checksum = Some(md5::compute(&self.image_data));
    }
    self.checksum.ok_or(anyhow!("unable to get checksum for sticker"))
  }

  pub fn load_from_data(data: &[u8]) -> Result<StickerInfo> {
    Ok(StickerInfo::new(base64::encode(data)))
  }

  pub fn save<P>(&mut self, path: P) -> Result<()> where P: AsRef<std::path::Path> {
    let path_ref: &std::path::Path = path.as_ref();

    let filepath = path_ref.with_file_name(format!("{:?}.webp", self.checksum()?));

    #[cfg(debug_assertions)]
    println!("saving {:?}", filepath);

    let image_data = base64::decode(&self.image_data)?;
    std::fs::write(filepath, image_data).or(Err(anyhow!("unable to extract sticker")))
  }
}
