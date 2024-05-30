use clap::Parser;
use rcli::{Opts, Subcommand};


fn main() -> anyhow::Result<()>{
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => 
            rcli::process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}

