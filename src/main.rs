use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Opts {
    // Set the target window handle
    #[clap(short, long)]
    target: String,
}

fn main() {
    let _opts = Opts::parse();
}
