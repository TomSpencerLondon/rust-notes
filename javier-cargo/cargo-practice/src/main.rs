use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"  Thank you for a fun session\n       Javier and Mattsi!\n Good evening fellow Rustaceans!";
    let width = 35;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
