extern crate clap;

use clap::{App, AppSettings};

pub fn app<'a, 'b>() -> App<'a, 'b> {
  App::new("WSTicK packer")
    .version("0.1")
    .author("William Artero <https://artero.dev>")
    .about("WSTicK pack tool to create and maintain sticker packs")
    .setting(AppSettings::ArgRequiredElseHelp)
    .subcommand(generate::command())
    .subcommand(extract::command())
}

pub mod generate {
  use clap::{App, SubCommand, Arg};

  pub static NAME: &str = "generate";
  pub static ARG_DIRECTORY: &str = "directory";

  pub fn command<'a, 'b>() -> App<'a,'b> {
    SubCommand::with_name(self::NAME)
      .about("generate a WSTicK-compatible sticker pack json")
      .arg(
        Arg::with_name(self::ARG_DIRECTORY)
          .multiple(true)
          .required(true)
          .help("sticker pack directories")
      )
  }
}

pub mod extract {
  use clap::{App, SubCommand, Arg};

  pub static NAME: &str = "extract";
  pub static ARG_PACK_FILE: &str = "pack-file";

  pub fn command<'a, 'b>() -> App<'a,'b> {
    SubCommand::with_name(self::NAME)
      .about("extracts images from a WSTicK-compatible sticker pack json")
      .arg(
        Arg::with_name(self::ARG_PACK_FILE)
          .required(true)
          .help("sticker pack json file")
      )
  }
}
