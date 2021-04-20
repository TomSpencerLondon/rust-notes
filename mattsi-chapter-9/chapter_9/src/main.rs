use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        }else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<File, io::Error> {
    ok(File::open("hello.txt")?)
}
