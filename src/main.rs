use std::io::{self, Read, Write};

fn main() {
    let mut bytes = Vec::new();
    let mut buf: [u8; 1] = [0; 1];

    loop {
        let n = io::stdin()
            .read(&mut buf)
            .expect("Failed to read from stdin");

        //println!("Read {} bytes: {:?}", n, &buf[..n]);

        if n == 0 {
            break;
        }
        bytes.push(buf[0].reverse_bits());
    }

    bytes.reverse();
    let _ = io::stdout()
        .write(&bytes)
        .expect("Failed to write to stdout");
}
