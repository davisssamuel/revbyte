use std::io::{self, BufWriter, Read, Write};

fn reverse_bits(byte: u8) -> u8 {
    let mut ret: u8 = 0;

    for i in 0..8 {
        let mut mask = (byte >> (7 - i)) & 1;
        mask <<= i;
        ret |= mask;
    }

    ret
}

fn main() {
    let mut bytes = Vec::new();

    io::stdin()
        .read_to_end(&mut bytes)
        .expect("Failed to read from stdin");

    let mut buf_writer = BufWriter::new(io::stdout());

    bytes.iter().rev().for_each(|b| {
        buf_writer
            .write_all(&[reverse_bits(*b)])
            .expect("Failed to write to stdout");
    });

    buf_writer.flush().expect("Failed to flush stdout");
}
