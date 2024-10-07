use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error};
use std::path::Path;
use regex::Regex;
use syntect::easy::HighlightFile;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

fn bark_line(line: &str) -> String {
    format!("{}... woof!\n", line)
}

fn whine_error(message: &str) -> String {
    format!("{}... whine! (Error)\n", message)
}

fn fetch_and_grep_file(filename: &str, pattern: Option<&Regex>) -> Result<(), Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    println!("Fetching {}... arf!\n", filename);
    for line in reader.lines() {
        let line_content = line?;
        if let Some(ref pat) = pattern {
            if pat.is_match(&line_content) {
                println!("{}", bark_line(&line_content));
            }
        } else {
            println!("{}", bark_line(&line_content));
        }
    }

    Ok(())
}

fn fetch_and_highlight_file(filename: &str) -> Result<(), Error> {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let theme = &ts.themes["base16-ocean.dark"];
    
    let mut highlighter = HighlightFile::new(filename, &ps, theme).map_err(|_| {
        io::Error::new(io::ErrorKind::Other, "File can't be highlighted (unsupported or not found)")
    })?;

    let reader = highlighter.reader;

    println!("Fetching and highlighting {}... arf!\n", filename);
    for line in reader.lines() {
        let line_content = line?;
        let regions = highlighter.highlight_lines.highlight(&line_content, &ps);
        let escaped = syntect::util::as_24_bit_terminal_escaped(&regions[..], true);
        println!("{}", escaped);
    }

    Ok(())
}

fn sniff_file(filename: &str) -> bool {
    Path::new(filename).exists()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{}", whine_error("Sniff sniff... no files to fetch! Usage: dog <file1> [--grep <pattern>] [--highlight]"));
        return;
    }

    let mut pattern: Option<Regex> = None;
    let mut highlight = false;

    let mut filenames: Vec<String> = Vec::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--grep" {
            if i + 1 < args.len() {
                pattern = Some(Regex::new(&args[i + 1]).expect("Invalid regex pattern"));
                i += 1;
            } else {
                eprintln!("{}", whine_error("Missing pattern after --grep"));
                return;
            }
        } else if args[i] == "--highlight" {
            highlight = true;
        } else {
            filenames.push(args[i].clone());
        }
        i += 1;
    }

    if filenames.is_empty() {
        eprintln!("{}", whine_error("Sniff sniff... no files to fetch! Usage: dog <file1> [--grep <pattern>] [--highlight]"));
        return;
    }

    for filename in filenames {
        if sniff_file(&filename) {
            if highlight {
                if let Err(e) = fetch_and_highlight_file(&filename) {
                    eprintln!("{}", whine_error(&format!("Couldn't fetch and highlight {}: {}", filename, e)));
                }
            } else if let Err(e) = fetch_and_grep_file(&filename, pattern.as_ref()) {
                eprintln!("{}", whine_error(&format!("Couldn't fetch {}: {}", filename, e)));
            }
        } else {
            eprintln!("{}", whine_error(&format!("Can't find {}... grrrr! (File not found)", filename)));
        }
    }
}
