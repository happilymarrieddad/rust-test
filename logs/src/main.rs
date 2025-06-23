use std::fs;
use std::io::Error;

// use std::str::Utf8Error;
// use std::string::{FromUtf8Error,FromUtf16Error};
// use std::num::{ParseIntError,ParseFloatError,TryFromIntError};
// use std::thread::JoinError;


// fn string_test(
//     a: String,  // 
//     b: &String, // read only -> almost never (Rust turns these into &str automatically)
//     c: &str     // read only
// ) {

// }
//     string_test(
//         String::from("red"),
//         &String::from("red"),
//         "red"
//     );

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}



fn divide(a: f64, b :f64) -> Result<f64,Error> {
    if b == 0.0 {
        return Err(Error::other("can't divide by zero"))
    }

    Ok(a / b)
}

fn validate_email(email: String) -> Result<(),Error> {
    if email.contains("@") {
        return Ok(())
    }

    Err(Error::other("must contain an @"))
}

fn main() {



    let color = String::from("red");
    let portion = &color[1..3]; // == "ed" 


    // match divide(2.0, 3.0) {
    //     Ok(result_of_division) => {
    //         println!("result {}", result_of_division);
    //     }
    //     Err(err) => {
    //         println!("{}", err);
    //     }
    // }

    // match validate_email(String::from("nick@mail.com")) {
    //     Ok(..) => {
    //         println!("email is valid")
    //     }
    //     Err(err) => {
    //         println!("{}", err);
    //     }  
    // }

    let text = fs::read_to_string("logs.txt");
    if !text.is_ok() {
        println!("unable to read file");
        return
    }

    println!("{:#?}", text.unwrap());

    let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(txt) => {
            error_logs = extract_errors(txt.as_str());
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    println!("{:?}", error_logs);
}

enum LogType {
    Info,
    Warn,
    Debug,
    Error
}

pub fn log(typ: LogType, err: Error) {
    match typ {
        LogType::Info => {
            println!("Info  {} {}", "01-02-25 04:35:04", err)
        }
        LogType::Warn => {
            println!("Warn  {} {}", "01-02-25 04:35:04", err)
        }
        LogType::Debug => {
            println!("Debug {} {}", "01-02-25 04:35:04", err)
        }
        LogType::Error => {
            println!("Error {} {}", "01-02-25 04:35:04", err)
        }
    }
}
