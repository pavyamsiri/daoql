mod lexer;
use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
    path::Path,
    process, str,
};

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Give a path to the SQL file.");
        process::exit(1);
    }

    let file_path = &args[1];
    let text = read_file(file_path).expect("Can't read file!");

    println!("Lexing {file_path}");
    let mut lexer = Lexer::new(&text);
    let mut position = 0;
    let mut highlighted_text = String::new();

    while let Some(token) = lexer.next_token() {
        // Append text before the token
        if position < token.span.start {
            highlighted_text.push_str(&text[position..token.span.start]);
        }

        // Append the highlighted token
        highlighted_text.push_str(token.begin_highlight());
        highlighted_text.push_str(&text[token.span.range()]);
        highlighted_text.push_str(token.end_highlight());

        // Update the position
        position = token.span.end();
    }

    // Append any remaining text after the last token
    if position < text.len() {
        highlighted_text.push_str(&text[position..]);
    }

    print!("{highlighted_text}");
}

fn read_file<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fn inner(path: &Path) -> io::Result<String> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        let _ = reader.read_to_end(&mut buffer);
        let content =
            str::from_utf8(&buffer).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(content.to_string())
    }

    inner(path.as_ref())
}
