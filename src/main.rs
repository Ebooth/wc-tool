use clap::Parser;
use std::fmt;
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

    #[arg(short)]
    l: bool,
}

struct WcResult {
    file: String,
    bytes: Option<usize>,
    lines: Option<usize>,
}

impl WcResult {
    fn new(file: String) -> WcResult {
        WcResult {
            file,
            bytes: None,
            lines: None,
        }
    }
}

impl fmt::Display for WcResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = [self.bytes, self.lines]
            .iter()
            .filter_map(|&x| x)
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        write!(f, "{} {}", fields.join(" "), self.file)
    }
}

fn main() {
    let args = Args::parse();
    let file = &args.file;

    let mut wc_result = WcResult::new(file.to_owned());

    let content = fs::read(file).expect(format!("{} not found ", file).as_str());
    if args.c {
        wc_result.bytes = Some(content.len());
    };
    let string_content = String::from_utf8(content).unwrap();
    let lines = string_content.split("\n").collect::<Vec<_>>();
    if args.l {
        wc_result.lines = Some(lines.len());
    }

    println!("{}", wc_result);
}
