# Token Clipboard Copier

`Token Clipboard Copier` is a utility written in Rust to tokenize text files and copy chunks of them to your clipboard.

## Features
- Recursively reads text files from a specified directory that matches a given pattern.
- Tokenizes the files, and ensures each chunk of tokens doesn't exceed a predefined limit.
- Maintains integrity of files within chunks, i.e., chunks end on file boundaries.
- Easy to use CLI interface.

## Prerequisites
You need to have [Rust and Cargo](https://rustup.rs/) installed on your machine.

## Dependencies
- `clipboard`: To handle clipboard operations.
- `glob`: To handle file pattern matching.

You can install these dependencies by running:
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

3. Move the binary to `/usr/local/bin`:
```bash
sudo mv target/release/token_clipboard_copier /usr/local/bin/
```
Please ensure that `/usr/local/bin/` is in your system's PATH.

## Usage

Run the program:
```bash
token_clipboard_copier <directory> <pattern>
```
For example:
```bash
token_clipboard_copier src "*.tsx"
```

Follow the on-screen instructions to copy the chunks to your clipboard.

## Notes
- Make sure that no file individually exceeds the token limit as that will be taken as a separate chunk.
- You'll be prompted to press `Enter` after copying each chunk to your clipboard.
