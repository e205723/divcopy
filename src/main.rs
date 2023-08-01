use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use glob::glob;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn tokenize(content: String) -> Vec<String> {
    content.split(' ').map(|s| s.to_string()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Please provide directory and pattern as command line arguments");
        return;
    }
    let directory = &args[1];
    let pattern = &args[2];

    let mut token_limit = 4096;
    if args.len() > 3 {
        if let Ok(limit) = args[3].parse::<usize>() {
            token_limit = limit;
        }
    }

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

                file_content.push_str(&format!("=====<{}>=====\n", path.display()));

                for line in reader.lines() {
                    let line = line.expect("Failed to read line");
                    file_content.push_str(&line);
                    file_content.push_str("\n");
                }

                let mut tokens = tokenize(file_content.clone());

                while !tokens.is_empty() {
                    let tokens_len = tokens.len();
                    if count + tokens_len > token_limit {
                        if !chunk.is_empty() {
                            chunks.push(chunk);
                            chunk = Vec::new();
                        }
                        count = 0;
                    }

                    if tokens_len > token_limit {
                        let (to_take, remaining) = tokens.split_at(token_limit - count);
                        chunk.extend(to_take.iter().cloned());
                        tokens = remaining.to_vec();
                        count = chunk.len();
                        if !chunk.is_empty() {
                            chunks.push(chunk.clone());
                            chunk.clear();
                        }
                    } else {
                        count += tokens_len;
                        chunk.extend(tokens.clone());
                        tokens.clear();
                    }
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
        let text = chunk.join(" ");
        clipboard.set_contents(text.clone()).unwrap();
        println!("Copied a chunk. Press Enter to continue...");
        std::io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
    }
}
