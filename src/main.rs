use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use glob::glob;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const TOKEN_LIMIT: usize = 4096;

fn tokenize(content: String) -> Vec<String> {
    content.split(' ').map(|s| s.to_string()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <directory> <pattern>", args[0]);
        std::process::exit(1);
    }

    let directory = &args[1];
    let pattern = &args[2];

    let pattern = format!("{}/**/{}", directory, pattern);
    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut chunk: Vec<String> = Vec::new();
    let mut count = 0;

    for entry in glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let file = File::open(&path).expect("Failed to open file");
                let reader = BufReader::new(file);
                let mut file_content = String::new();

                for line in reader.lines() {
                    let line = line.expect("Failed to read line");
                    file_content.push_str(&line);
                    file_content.push_str("\n");
                }

                let tokens = tokenize(file_content.clone());

                let tokens_len = tokens.len();
                if count + tokens_len > TOKEN_LIMIT {
                    chunks.push(chunk);
                    chunk = Vec::new();
                    count = 0;
                }

                if tokens_len > TOKEN_LIMIT {
                    println!("Warning: The file {} has more tokens than the TOKEN_LIMIT. It will be a separate chunk.", path.display());
                    chunks.push(tokens);
                } else {
                    count += tokens_len;
                    chunk.push(file_content);
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    if !chunk.is_empty() {
        chunks.push(chunk);
    }

    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();

    for chunk in chunks {
        let text = chunk.join("");
        clipboard.set_contents(text.clone()).unwrap();
        println!("Copied a chunk. Press Enter to continue...");
        std::io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
    }
}
