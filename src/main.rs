use std::error::Error;

mod cli;
mod packer;
use packer::{StickerPack};

fn main() -> Result<(), Box<dyn Error>> {
  let matches = cli::app().get_matches();

  if let Some(generate_matches) = matches.subcommand_matches(cli::generate::NAME) {
    let paths = generate_matches.values_of(cli::generate::ARG_DIRECTORY).ok_or("no directory provided")?;
    paths.for_each(|pack_path| {
      match StickerPack::generate(pack_path) {
        Ok(result) => println!("pack {} generated successfully", result.name),
        Err(error) => eprintln!("{}", error),
      }
    });
  } else if let Some(extract_matches) = matches.subcommand_matches(cli::extract::NAME) {
    let json_path = extract_matches.value_of(cli::extract::ARG_PACK_FILE).ok_or("no pack json file provided")?;
    match StickerPack::extract(json_path) {
      Ok(_) => println!("pack extracted successfully"),
      Err(error) => eprintln!("{}", error),
    }
  }

  Ok(())
}
