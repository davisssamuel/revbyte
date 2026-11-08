use std::io::{self, Read, Write};

fn reverse_bits(byte: &u8) -> u8 {
    let mut mask: u8 = 0b10000000;
    let mut ret: u8 = 0;

    for i in 0..8 {
        let a = byte & mask;
        if a > 0 {
            let mut mask = 1;
            mask <<= i;
            ret |= mask;
        }
        mask >>= 1;
    }
    ret
}

fn main() {
    let mut bytes = Vec::new();

    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Failed to read from stdin");

    let mut bytes_reversed = Vec::new();
    while let Some(b) = bytes.pop() {
        let b = reverse_bits(&b);
        bytes_reversed.push(b);
    }

    io::stdout()
        .write_all(&bytes_reversed)
        .expect("Failed to write to stdout");
}
