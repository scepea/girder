use std::env;
use std::string::String;

pub fn example() {
    let args: Vec<String> = env::args().collect();
    
    let mut count: Option<i32> = None;
    
    for arg in args.as_slice().windows(2) {
        match arg {
            [key, value] => match (key.as_str(), value) {
                ("--count",value) => match value.parse() {
                    Ok(value) => count = Some(value),
                    Err(error) => eprintln!("Problem with --count argument: {}", error)
                },
                _ => ()
            },
            _ => ()
        }
    }

    match count {
        Some(x) => print!("Count equals {}", x),
        None => ()
    }
}