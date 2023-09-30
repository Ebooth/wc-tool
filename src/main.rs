use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "cwc")]
#[command(author = "Ebooth <pauldejendev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "A copy of unix command line tool wc ", long_about = None)]
struct Args {
    file: String,

    #[arg(short)]
    c: bool,
}

fn main() {
    let args = Args::parse();
    let file = &args.file;
    let content = fs::read(file).expect(format!("{} not found ", file).as_str());
    if args.c {
        println!("{} {}", content.len(), file);
    }
}
