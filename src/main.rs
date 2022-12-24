use atty::{isnt, Stream};
use copypasta_ext::x11_bin::ClipboardContext;
use copypasta_ext::prelude::ClipboardProvider;
use std::io::{self, Read, Write};

fn main() {
    let mut clip_ctx = ClipboardContext::new().unwrap();

    // If stdin is a pipe, read all available data and set the clipboards content to it
    if isnt(Stream::Stdin) {
        let mut buf = String::new();
        let mut stdin = io::stdin();

        match stdin.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error reading from stdin: {}", err);
                return;
            }
        };

        clip_ctx.set_contents(buf.to_owned()).unwrap();
    }

    // If stdout is a pipe, write the clipboard content to it
    if isnt(Stream::Stdout) {
        let mut stdout = io::stdout();

        let content: String = match clip_ctx.get_contents() {
            Ok(str) => str,
            Err(err) => {
                eprintln!("Error getting clipboard contents: {}", err);
                return;
            }
        };

        match stdout.write_all(content.as_bytes()) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("Error writing to stdout: {}", err);
                return;
            }
        };
    }
}
