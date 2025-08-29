use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    
    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file : {e:?}"), // simply finish the program
                                                                       // if we are not able to
                                                                       // create the file.
            },
            _ => {
                panic!("Problem opening the file : {error:?}");
            },
        },
    };
}
