use clap::Parser;
use anyhow::Result;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Transition<'a> {
    state: String,
    targets: &'a Vec<String>,
    event: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg()]
   path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);
    let mut source_lines = Vec::new();
    let mut states = Vec::new();
    
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            source_lines.push(line?.clone());
        }
    }

    let mut state = String::new();
    let mut targets: Vec<String> = Vec::new();
    let mut event = String::new();
    
    for source_line in source_lines {
        let seek_state = get_state(&source_line);

        if seek_state.is_some() {

            state = seek_state.unwrap();
            println!("{}", &state);
            states.push(state);
        }
        else {
            let seek_event = get_event(&source_line);
            
            if seek_event.is_some() {
                event = seek_event.unwrap();
                println!("{}", &event);
            }
            else {
                let seek_next_state = get_next_state(&source_line);
                
                if seek_next_state.is_some() {
                    let next_state = seek_next_state.unwrap();
                    println!("{}", &next_state);
                    targets.push(next_state)
                }
                else {
                    let handled = get_handled(&source_line);
                    
                    if handled.is_some() {
                        println!("{}", &handled.unwrap());
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Path not found!");
    Ok(BufReader::new(file).lines())
}

fn get_state(line: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^QP::QState .*?::(\w*)").unwrap();
    }

    get_match(&RE, line)
}

fn get_event(line: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*case (\w*)").unwrap();
    }

    get_match(&RE, line)
}

fn get_next_state(line: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*Q_TRAN\(&(\w*)").unwrap();
    }

    get_match(&RE, line)
}

fn get_handled(line: &String) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*?(Q_HANDLED)\(").unwrap();
    }

    get_match(&RE, line)
}

fn get_match(re: &Regex, line: &String) -> Option<String> {
    let caps = re.captures(line);

    match caps {
        Some(c) => {
            let group = c.get(1);

            match group {
                Some(g) => return Some(g.as_str().to_string()),
                None => return None,
            };
        },
        None => return None,
    };

}