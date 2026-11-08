use std::io::{self, Read, Write};

fn main() {
    let mut bytes = Vec::new();

    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Failed to read from stdin");

    bytes.reverse();
    bytes.iter_mut().for_each(|b| *b = b.reverse_bits());

    io::stdout()
        .write_all(&bytes)
        .expect("Failed to write to stdout");
}
