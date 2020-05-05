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
  }

  Ok(())
}
