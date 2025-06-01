use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::fs;

fn main() {
    // let file = File::open("hello.txt");
    // let file = match file {
    //     Ok(f) => Ok(f),
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => File::create("hello.txt"),
    //         _ => panic!("Problem opening the file: {:?}", e),
    //     },
    // };

    // let file: File = File::open("hello.txt").unwrap();

    // let file: File = File::open("hello.txt").expect("Failed to open hello.txt");

    // let file: File = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").expect("Failed to create hello.txt")
    //     } else {
    //         panic!("Problem opening the file: {:?}", error)
    //     }
    // });

    let username = read_username();
    println!("Username: {:?}", username);
}

fn read_username() -> Result<String, Error> {
    // let file_result = File::open("hello.txt");
    // let mut file = match file_result {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };
    // let mut username = String::new();
    // match file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(error) => Err(error),
    // }

    // let mut username = String::new();
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    fs::read_to_string("hello.txt")
}
