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

    #[arg(short)]
    m: bool,

    #[arg(short)]
    w: bool,
}

struct WcResult {
    file: String,
    bytes: Option<usize>,
    lines: Option<usize>,
    words: Option<usize>,
    chars: Option<usize>,
}

impl WcResult {
    fn new(file: String) -> WcResult {
        WcResult {
            file,
            bytes: None,
            lines: None,
            words: None,
            chars: None,
        }
    }
}

impl fmt::Display for WcResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = [self.bytes, self.lines, self.chars, self.words]
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

    if args.l {
        let number_of_lines = string_content.split("\n").count();
        wc_result.lines = Some(number_of_lines);
    }
    if args.w {
        let number_of_lines = string_content.split("\n").count();
        wc_result.lines = Some(number_of_lines);
    }
    if args.m {
        let number_of_words = string_content.split_whitespace().count();
        wc_result.words = Some(number_of_words);
    }
    if args.m {
        let number_of_chars = string_content.chars().count();
        wc_result.chars = Some(number_of_chars);
    }

    println!("{}", wc_result);
}
