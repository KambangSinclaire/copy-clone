use std::{env, fs, io::Write, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = &args[1..];

    for arg in args {
        let arg: &str = &arg;
        match arg {
            "help" => println!("Help mode selected"),
            "cp" => {
                read_and_copy(&args[1], &args[2]).unwrap_or_else(|error| {
                    if error == false {
                        process::exit(0)
                    };
                    error
                });
            }
            _ => {
                println!("Exiting with arg as {}", arg);
                process::exit(0);
            }
        }
    }
}

fn read_and_copy(orig: &str, dest: &str) -> Result<bool, bool> {
    let source_contents = fs::read_to_string(orig);
    match source_contents {
        // declare contents as mutatble so we can append a new line character to it
        Ok(mut contents) => {
            contents.insert(contents.len(), '\n');
            let res = fs::OpenOptions::new().append(true).open(dest);
            match res {
                Ok(mut data) => match data.write_all(contents.as_bytes()) {
                    Ok(_) => Ok(true),
                    Err(_) => Err(false),
                },
                Err(_) => {
                    println!("Invalid destination file path");
                    Err(false)
                }
            }
        }
        Err(_) => {
            println!("Invalid source file path");
            Err(false)
        }
    }
}
