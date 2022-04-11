mod configuration;

use crate::configuration::get_settings;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Opts {
    word: String,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    let settings = get_settings()?;
    println!("{:?}", opts);
    println!("{:?}", settings);
    Ok(())
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Opts::command().debug_assert()
}
