use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use glob::glob;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// This function takes a string and returns a vector of substrings.
///
/// Each substring corresponds to a word in the input string.
/// The function splits the input string at whitespace characters and
/// retains newline characters.
///
/// # Arguments
///
/// * `content` - A String that holds the content to be tokenized
///
/// # Returns
///
/// * A vector of substrings
fn tokenize(content: String) -> Vec<String> {
    content
        .split(|c: char| c.is_whitespace() && c != '\n')
        .map(|s| s.to_string())
        .collect()
}

/// This function processes all the files that match the given glob pattern in the directory and its
/// subdirectories, reads the files' contents, tokenizes the contents, and returns the tokens
/// in chunks of a specified size.
///
/// # Arguments
///
/// * `directory` - The directory to search for files.
/// * `pattern` - The glob pattern to match file names against.
/// * `token_limit` - The maximum number of tokens to include in each chunk.
///
/// # Returns
///
/// * A vector of chunks, each chunk is a vector of strings (tokens).
fn process_files(directory: &str, pattern: &str, token_limit: usize) -> Vec<Vec<String>> {
    // Construct the glob pattern
    let pattern = format!("{}/**/{}", directory, pattern);
    let mut chunks: Vec<Vec<String>> = Vec::new();
    let mut chunk: Vec<String> = Vec::new();
    let mut count = 0;

    // Iterate through all the files that match the glob pattern
    for entry in glob(&pattern).unwrap_or_else(|_| panic!("Failed to read glob pattern")) {
        match entry {
            Ok(path) => {
                // Attempt to open the file
                let file = match File::open(&path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("Failed to open file {}: {}", path.display(), e);
                        continue;
                    }
                };

                let reader = BufReader::new(file);
                let mut file_content = String::new();

                // Append filename in content
                file_content.push_str(&format!("=====<{}>=====\n", path.display()));

                // Try to read each line in the file
                for line in reader.lines() {
                    match line {
                        Ok(line) => {
                            file_content.push_str(&line);
                            file_content.push_str("\n");
                        }
                        Err(e) => eprintln!("Failed to read line in {}: {}", path.display(), e),
                    }
                }

                // Tokenize the content and add tokens to chunks
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
            Err(e) => eprintln!("{:?}", e),
        }
    }

    if !chunk.is_empty() {
        chunks.push(chunk);
    }

    chunks
}

/// This function is the main entry point of the program.
///
/// It takes command-line arguments specifying a directory, a glob pattern, and an optional
/// token limit. It finds all files that match the given pattern in the directory and its
/// subdirectories, reads the files' contents, tokenizes the contents, and copies the tokens
/// to the clipboard in chunks of a specified size.
///
/// The program interacts with the user through the command line. If an error occurs
/// (such as a file cannot be opened), the error message is printed to the standard error.
///
/// # Command-line Arguments
///
/// 1. `directory` - The directory to search for files.
/// 2. `pattern` - The glob pattern to match file names against.
/// 3. `token_limit` - (Optional) The maximum number of tokens to include in each chunk.
///
/// # Example
///
/// ```
/// cargo run ./my_directory *.txt 100
/// ```
/// This command will find all `.txt` files in `./my_directory` and its subdirectories,
/// tokenize their contents, and copy the tokens to the clipboard in chunks of at most 100 tokens.
fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check for adequate command-line arguments
    if args.len() < 3 {
        eprintln!("Please provide directory and pattern as command line arguments");
        std::process::exit(1);
    }

    let directory = &args[1];
    let pattern = &args[2];

    // Set the default limit for tokens
    let mut token_limit = 4096;
    if args.len() > 3 {
        match args[3].parse::<usize>() {
            Ok(limit) => token_limit = limit,
            Err(e) => {
                eprintln!("Failed to parse token limit: {}", e);
                std::process::exit(1);
            }
        }
    }

    let chunks = process_files(directory, pattern, token_limit);

    let mut clipboard: ClipboardContext = match ClipboardProvider::new() {
        Ok(context) => context,
        Err(e) => {
            eprintln!("Failed to initialize the clipboard: {}", e);
            std::process::exit(1);
        }
    };

    let total_chunks = chunks.len();
    for (i, chunk) in chunks.into_iter().enumerate() {
        let text = chunk.join(" ");
        if clipboard.set_contents(text.clone()).is_err() {
            eprintln!("Failed to set clipboard contents");
            std::process::exit(1);
        }
        if i < total_chunks - 1 {
            println!("Copied a chunk. Press Enter to continue...");
            let mut input = String::new();
            if std::io::stdin().read_line(&mut input).is_err() {
                eprintln!("Failed to read line");
                std::process::exit(1);
            }
        }
    }

    println!("Copied a chunk. All chunks copied to clipboard!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = "Hello, world!\nThis is a test.".to_string();
        let expected_output = vec!["Hello,", "world!\nThis", "is", "a", "test."];
        assert_eq!(tokenize(input), expected_output);
    }

    #[test]
    fn test_process_files() {
        let directory = "./test_files";
        let pattern = "*.txt";
        let token_limit = 100;
        let expected_output = vec![vec![
            "=====<test_files/file1.txt>=====\nHello,",
            "world!\nThis",
            "is",
            "a",
            "test.\n",
            "=====<test_files/file2.txt>=====\nAnother",
            "test",
            "file.\nWith",
            "multiple",
            "lines.\n",
        ]];
        assert_eq!(
            process_files(directory, pattern, token_limit),
            expected_output
        );
    }
}
