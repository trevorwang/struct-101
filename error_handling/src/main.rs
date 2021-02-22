use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;
fn main() {
    // let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // open_file();
    read_username_from_file();
}

fn _open_file() {
    let f = File::open("hello2.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello2.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

use std::fs;

fn read_username_from_file3() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
