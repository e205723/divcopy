# divcopy

`divcopy` is a utility written in Rust that reads and tokenizes text files, then copies chunks of them to your clipboard. Each chunk respects the boundary of files, and when a file contains tokens that exceed the predefined limit, the file content is gracefully split into multiple chunks.

## Features
- Recursively reads text files from a specified directory that matches a given pattern.
- Tokenizes the content of the files, ensuring that each chunk of tokens doesn't exceed a user-definable limit.
- Preserves the integrity of files within chunks — chunks always end at the end of a file.
- In case a file's tokens exceed the defined limit, the file content is split across multiple chunks.
- User-friendly CLI interface that prompts users to press `Enter` after each chunk is copied.

## Prerequisites
To use `Token Clipboard Copier`, you need to have [Rust and Cargo](https://rustup.rs/) installed on your machine.

## Dependencies
- `clipboard`: To handle clipboard operations.
- `glob`: To handle file pattern matching.

To install these dependencies, run:
```bash
cargo build
```

## Installation

1. Clone this repository:
```bash
git clone <repository-url>
cd <repository-directory>
```

2. Build the project:
```bash
cargo build --release
```

3. Move the binary to `/usr/local/bin` (or any directory included in your system's PATH):
```bash
sudo mv target/release/divcopy /usr/local/bin/
```

## Usage

Run the program with a specified directory and pattern. Optionally, you can specify a third argument for the token limit (default is 4096):
```bash
divcopy <directory> <pattern> [token_limit]
```
For example:
```bash
divcopy src "*.c" 5000
```

Follow the on-screen instructions to copy the chunks to your clipboard.

## Notes
- Be aware that if a single file's tokens exceed the defined token limit, its content will be split into multiple chunks.
- You'll be prompted to press `Enter` after copying each chunk to your clipboard. This allows you to handle each chunk of tokens individually.
