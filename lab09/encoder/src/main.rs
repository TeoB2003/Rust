use base64::encode;
use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(version, about = "encode to base64")]
struct Args {
    #[arg(short, long, default_value = "*")]
    input: String,

    #[arg(short, long, default_value = "*")]
    output: String,
}

fn main() {
    let args = Args::parse();
    println!("Version: {}", env!("CARGO_PKG_VERSION"));

    // ... (Your target_os checks)

    if args.output == "*" && args.input == "*" {
        let mut input = String::new();
        println!("Enter a string: ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let byte_slice: &[u8] = input.trim().as_bytes();
        println!("{}", encode(byte_slice));
    } else {
        let mut file = match File::open(&args.input) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error opening file: {}", e);
                return;
            }
        };

        let mut file_contents = String::new();
        match file.read_to_string(&mut file_contents) {
            Ok(_) => {
                let input = file_contents.as_bytes();
                let content_to_write = encode(input);

                if args.output == "*" {
                    // If output is "*", print to stdout
                    println!("{}", content_to_write);
                } else {
                    let mut output_file = match File::create(&args.output) {
                        Ok(file) => file,
                        Err(e) => {
                            eprintln!("Error creating file: {}", e);
                            return;
                        }
                    };

                    match output_file.write_all(content_to_write.as_bytes()) {
                        Ok(_) => {
                            println!("Content written to the file successfully!");
                        }
                        Err(e) => {
                            eprintln!("Error writing to file: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading file: {}", e);
            }
        }
    }
}
