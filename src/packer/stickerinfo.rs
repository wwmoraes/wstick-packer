extern crate serde_derive;

use serde_derive::{Serialize, Deserialize};
use base64;

use crate::packer::types::Result;

#[derive(Serialize, Deserialize, Clone)]
pub struct StickerInfo {
  pub image_data: String,
  pub emojis: Vec<String>,
}

impl Default for StickerInfo {
  fn default() -> StickerInfo {
    StickerInfo {
      image_data: "".to_owned(),
      emojis: vec!(),
    }
  }
}

impl StickerInfo {
  pub fn load_from_data(data: Vec<u8>) -> Result<StickerInfo> {
    Ok(StickerInfo {
      image_data: base64::encode(data),
      ..StickerInfo::default()
    })
  }
}
