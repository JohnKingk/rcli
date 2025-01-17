use std::path::Path;

use clap::Parser;

#[derive(Debug,Parser)]
#[command(name="rcli", version, author, about, long_about=None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd: Subcommand
}

#[derive(Debug, Parser)]
pub enum Subcommand {
    #[command(name="csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts{
    #[arg(short, long, value_parser= verify_input_file )]
    pub input: String,

    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}



fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.into())
    }else {
        Err("File does not exit")
    }
}