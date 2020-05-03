use std::error::Error;
use std::env;

mod packer;
use packer::{StickerPack};

fn main() -> Result<(), Box<dyn Error>> {
  // get arguments skipping the caller
  let args: Vec<String> = env::args().skip(1).collect();

  // we need at least one folder to try generating the sticker pack!
  if args.len() == 0 {
    Err("pass at least one folder")?
  }

  // parse each folder passed
  args.into_iter().map(StickerPack::generate).for_each(|pack| {
    match pack {
      Ok(result) => println!("pack {} generated successfully", result.name),
      Err(error) => eprintln!("{}", error),
    }
  });

  Ok(())
}
