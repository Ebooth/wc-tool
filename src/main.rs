use clap::Parser;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
#[derive(Parser, Debug)]
#[command(name = "cwc")]
#[command(author = "Ebooth <pauldejeandev@gmail.com>")]
#[command(version = "1.0")]
#[command(about = "A copy of unix command line tool wc ", long_about = None)]

struct Args {
    files: Vec<String>,

    /// The number of bytes in each input file is written to the standard output.
    #[arg(short = 'c', group = "byte_count")]
    bytes: bool,

    /// The number of lines in each input file is written to the standard output.
    #[arg(short = 'l')]
    lines: bool,

    /// The number of characters in each input file is written to the standard output.  
    /// If the current locale does not support multibyte characters, this is equivalent to the -c option.
    #[arg(short = 'm', group = "byte_count")]
    chars: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short = 'w')]
    words: bool,
}

struct WcResult {
    file: String,
    bytes: Option<usize>,
    lines: Option<usize>,
    words: Option<usize>,
    chars: Option<usize>,
}

impl WcResult {
    fn new() -> WcResult {
        WcResult {
            file: String::new(),
            bytes: None,
            lines: None,
            words: None,
            chars: None,
        }
    }

    fn add(&mut self, other: &Self) {
        if let Some(b) = other.bytes {
            self.bytes = Some(b + self.bytes.unwrap_or(0));
        }
        if let Some(c) = other.chars {
            self.chars = Some(c + self.chars.unwrap_or(0));
        }
        if let Some(w) = other.words {
            self.words = Some(w + self.words.unwrap_or(0));
        }
        if let Some(l) = other.lines {
            self.lines = Some(l + self.lines.unwrap_or(0));
        }
    }
}

impl fmt::Display for WcResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fields = [self.lines, self.words, self.bytes, self.chars]
            .iter()
            .filter_map(|&x| x)
            .map(|x| format!("{x:8}"))
            .collect::<String>();
        write!(f, "{} {}", fields, self.file)
    }
}

fn main() {
    let mut args = Args::parse();
    let files = &args.files;

    if [args.bytes, args.chars, args.lines, args.words]
        .iter()
        .all(|&x| !x)
    {
        args.bytes = true;
        args.lines = true;
        args.words = true;
    }

    if files.is_empty() {
        let content = read_from_stdin();
        let wc_result = process_content(content, &args);
        if let Some(v) = wc_result {
            println!("{}", v);
        }
        return;
    }

    if files.len() == 1 {
        process_file(&files[0], &args);
        return;
    }

    let mut total_result = WcResult::new();
    total_result.file = "total".to_owned();

    for file in files {
        let result = process_file(file, &args);

        if let Some(v) = result {
            total_result.add(&v);
        }
    }
    println!("{}", total_result);
}

fn read_from_stdin() -> Vec<u8> {
    let mut content: Vec<u8> = vec![];
    let stdin = io::stdin();

    let mut reader = stdin.lock();
    let mut buffer = reader.fill_buf().unwrap();
    while !buffer.is_empty() {
        content.extend_from_slice(buffer);
        let length = buffer.len();
        reader.consume(length);
        buffer = reader.fill_buf().unwrap();
    }

    return content;
}

fn process_file(file: &str, args: &Args) -> Option<WcResult> {
    let content = match fs::read(file) {
        Ok(val) => val,
        Err(_) => {
            println!("{}: open: No such file or directory", file);
            return None;
        }
    };

    let wc_result = process_content(content, args);
    if let Some(mut v) = wc_result {
        v.file = file.to_owned();
        println!("{}", v);
        return Some(v);
    }
    return None;
}

fn process_content(content: Vec<u8>, args: &Args) -> Option<WcResult> {
    let mut wc_result = WcResult::new();
    if args.bytes {
        wc_result.bytes = Some(content.len());
    };

    let string_content = match String::from_utf8(content) {
        Ok(val) => val,
        Err(error) => {
            println!("{error}");
            return None;
        }
    };

    if args.lines {
        let number_of_lines = string_content.matches("\n").count();
        wc_result.lines = Some(number_of_lines);
    }
    if args.words {
        let number_of_words = string_content.split_whitespace().count();
        wc_result.words = Some(number_of_words);
    }
    if args.chars {
        let number_of_chars = string_content.chars().count();
        wc_result.chars = Some(number_of_chars);
    }

    return Some(wc_result);
}
