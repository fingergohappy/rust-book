use core::panic;
use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn open_file() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("open file error:{:?}", error)
        }
    };
}

fn open_file_err_kind() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("create file error {:?}", e),
            },
            other_error => {
                panic!("open file error : {:?}", other_error)
            }
        },
    };
}

fn open_file_error_wrap() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("create file error {:?}", e);
            })
        } else {
            panic!("open file error : {:?}", error);
        }
    });
}

fn open_file_expect() {
    let greeting_file = File::open("hello.txt").expect("open file error");
}

fn read_uesr_name_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut user_name = String::new();
    username_file.read_to_string(&mut user_name);
    Ok(user_name)
}

fn read_from_user_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn main() {
    open_file_err_kind();
}
