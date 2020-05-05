// extern crate serde_derive;
extern crate serde;
// extern crate serde_json;

use std::path::Path;
use serde_derive::{Serialize, Deserialize};
use std::fs::read_to_string;
use serde_json;
use md5;
use anyhow::anyhow;

use crate::packer::stickerinfo::StickerInfo;
use crate::packer::types::Result;

type StickersCollection = std::collections::HashMap<md5::Digest, StickerInfo>;

static PACK_FILE: &str = "pack.json";

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct StickerPack {
  pub name: String,
  pub identifier: String,
  pub publisher: String,
  pub tray_image: String,
  pub stickers: Vec<StickerInfo>,
  ios_app_store_link: String,
  android_play_store_link: String,
  // publisher_email: String,
  // publisher_website: String,
  // privacy_policy_website: String,
  // license_agreement_website: String,
}

impl Default for StickerPack {
  fn default() -> StickerPack {
      StickerPack {
        name: "unnamed".to_owned(),
        identifier: "unidentified".to_owned(),
        publisher: "unknown".to_owned(),
        tray_image: "".to_owned(),
        stickers: vec!(),
        ios_app_store_link: "https://itunes.apple.com/app/wstick/id1442273161?mt=8".to_owned(),
        android_play_store_link: "https://play.google.com/store/apps/details?id=com.wstick.hk".to_owned(),
      }
  }
}

impl StickerPack {
  fn load<P>(path: P) -> Result<StickerPack> where P: AsRef<Path> {
    let path_ref: &std::path::Path = path.as_ref();

    #[cfg(debug_assertions)]
    println!("loading {:?}", path_ref);

    // TODO load directly the given json, otherwise load a default json name
    let json_path: std::path::PathBuf;

    // load either a path + default filename or the provided filename
    if path_ref.is_dir() {
      json_path = path_ref.join(PACK_FILE);
    } else {
      json_path = path_ref.to_path_buf();
    }

    #[cfg(debug_assertions)]
    println!("json path considered: {:?}", json_path);

    // check if the pack file exists
    if ! json_path.is_file() {
      Err(anyhow!("invalid path {:?} provided", path_ref))?
    }

    // read the pack.json contents and unmarshal it to a StickerPack object
    let base_contents = read_to_string(json_path)?;
    let result = serde_json::from_str::<StickerPack>(base_contents.as_str())?;
    Ok(result)
  }

  fn save<P>(&self, path: P) -> Result<()> where P: AsRef<Path> {
    let base_directory = path.as_ref();

    if ! base_directory.is_dir() {
      let base_path_str = base_directory.to_str().ok_or(anyhow!("unable to get pack directory path"))?;
      Err(anyhow!("{} is not a directory, ignoring", base_path_str))?
    }

    // marshall the pack data
    let marshalled_data = serde_json::to_string_pretty(&self)?;

    // write the generated.json file with pack data
    let generated_file = base_directory.join("generated.json");

    let result = std::fs::write(generated_file, marshalled_data)?;
    Ok(result)
  }

  pub fn generate<P>(path: P) -> Result<StickerPack> where P: AsRef<Path> {
    let base_directory: &std::path::Path = path.as_ref();

    let mut pack_data = StickerPack::load(base_directory)?;

    // set tray image if it exists
    let tray_image_path = base_directory.join("tray_image.webp");
    if tray_image_path.is_file() {
      let tray_image_data = read_to_string(tray_image_path)?;
      pack_data.tray_image = base64::encode(tray_image_data);
    }

    // list the pack directory contents to marshal the images
    let pack_dir_contents = base_directory.read_dir()?;

    // creates a hash map that'll prevent duplicated stickers
    let mut stickers_collection: StickersCollection = std::collections::HashMap::new();

    // parse each entry on the pack directory
    // TODO thread all the things!
    for pack_dir_entry in pack_dir_contents {
      let entry: std::fs::DirEntry = pack_dir_entry?;

      // only continue if the entry is a file
      let entry_type: std::fs::FileType = entry.file_type()?;
      if ! entry_type.is_file() {
        continue;
      }

      // continue if the entry isn't a webp image
      let entry_path = entry.path();
      match entry_path.extension().ok_or(anyhow!("unable to get file extension for {:?}", entry_path)) {
        Ok(entry_extension) => {
          if ! entry_extension.eq("webp") {
            continue;
          }
        },
        Err(error) => {
          eprintln!("{}", error);
          continue;
        }
      }

      // get the file contents
      let entry_contents = std::fs::read(entry_path)?;

      // generate an md5 hash to use as a map key
      let sticker_hash = md5::compute(entry_contents.clone());

      // check if an entry already exists, and if not, create and insert
      stickers_collection.entry(sticker_hash).or_insert(StickerInfo::load_from_data(entry_contents)?);
    }

    // collect the hashmap values into the pack vector
    // TODO collect the hashmap values by moving instead of cloning
    pack_data.stickers = stickers_collection.values().cloned().collect::<Vec<StickerInfo>>();

    pack_data.save(base_directory)?;

    Ok(pack_data)
  }

  pub fn extract<P>(path: P) -> Result<()> where P: AsRef<Path> {
    let path_ref: &std::path::Path = path.as_ref();

    let pack_data = StickerPack::load(path_ref)?;

    for sticker in pack_data.stickers {
      let image_basename = md5::compute(&sticker.image_data);
      let sticker_path = path_ref.with_file_name(format!("{:?}.webp", image_basename));
      println!("extracting {:?}", sticker_path);
      let image_data = base64::decode(&sticker.image_data);
      match image_data {
        Ok(data) => std::fs::write(sticker_path, data)?,
        Err(error) => eprintln!("{}", error),
      }
    }

    Ok(())
  }
}
