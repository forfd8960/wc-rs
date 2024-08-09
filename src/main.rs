use clap::Parser;
use wc_rs::{Options, OptionsHandler};

fn main() -> anyhow::Result<()> {
    let opts = Options::parse();
    println!("{:?}", opts);

    let mut opt_handler = OptionsHandler::new(opts);
    opt_handler.handle_options()?;
    Ok(())
}
