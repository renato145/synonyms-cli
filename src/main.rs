use clap::Parser;

#[derive(Parser, Debug)]
#[clap(about, version)]
struct Opts {
    word: String,
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}

#[test]
fn verify_app() {
    use clap::IntoApp;
    Opts::command().debug_assert()
}
