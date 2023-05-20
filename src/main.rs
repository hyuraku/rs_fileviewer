use std::env;
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::io::{self, BufRead};
use colored::Colorize;
use regex::Regex;


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = File::open(filename).unwrap_or_else(|err| panic!("couldn't open {}: {}", filename, err));
    let fivelines = io::BufReader::new(&contents).lines().take(5);

    for (index, line) in fivelines.enumerate() {
        if let Ok(line) = line {
            println!("{}| {}", index + 1, line);
        }
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let mut args = parts;

        match command {
            "line" => {
                let contents = File::open(filename).
                    unwrap_or_else(
                        |err| panic!("couldn't open {}: {}", filename, err)
                    );

                let lines = io::BufReader::new(contents).lines().into_iter();

                let number_str = args.next().unwrap();
                let num_connect_re = Regex::new(r"([0-9]*)-([0-9]*)").unwrap();
                if num_connect_re.is_match(&number_str){
                    let caps = num_connect_re.captures(number_str).unwrap();

                    let first_num = caps.get(1).map_or("", |m| m.as_str());
                    let first_num: usize = first_num.parse().unwrap();

                    let last_num = caps.get(2).map_or("", |m| m.as_str());
                    let last_num: usize = last_num.parse().unwrap();                    

                    for (index, line) in lines.enumerate(){
                        if let Ok(line) = line {
                            let num = index + 1;
                            if first_num <= num && num <= last_num  {
                                println!("{}| {}", format!("{}", index+1).bold().blue(), line);
                            }
                        }
                    }
                } else {
                let num_re = Regex::new(r"([0-9]*)").unwrap();
                if num_re.is_match(&number_str) {
                    let caps = num_re.captures(number_str).unwrap();
                    let num = caps.get(1).map_or("", |m| m.as_str());
                    let num = num.parse().unwrap();

                    for (index, line) in lines.enumerate() {
                    if let Ok(line) = line {
                        if (index + 1) == num {
                            println!("{}| {}", format!("{}", num).bold().blue(), line);
                        }
                        }
                    }
                }
            }

            },
            "exit" => {
                return
            },
            _ => {
                println!("put other command");
            }
        }
    }
}
