use clap::Parser;
use anyhow::Result;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use lazy_static::lazy_static;
use regex::Regex;

fn main() -> Result<()> {
    let args = Args::parse();
    let path = Path::new(&args.path);
    let mut source_lines = Vec::new();
    let mut states = Vec::new();
    let mut transitions = Transitions::new();
    
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            source_lines.push(line?.clone());
        }
    }

    let mut transition = Transition::new();
    let mut targets: Vec<String> = Vec::new();
    let mut current_state = String::new();
    
    for source_line in source_lines {

        if let Some(state) = get_state(&source_line) {
            println!("{}", &state);
            states.push(state.clone());
            current_state = state.clone();

            if !&transition.is_empty() {
                transitions.add_transition(&transition);
            }
        }
        else if let Some(event) = get_event(&source_line) {
            transition.initialise(&current_state, &event);
            println!("{}", &event);
        }
        else if let Some(next_state) = get_next_state(&source_line) {
            println!("{}", &next_state);
            targets.push(next_state)
        }
        else if let Some(handled) = get_handled(&source_line) {                    
            println!("{}", &handled);
        }
        else {}
    }

    output_states(&states);

    transitions.output();

    Ok(())
}


fn output_states(states: &Vec<String>) {
    println!("");
    
    for state in states {
        println!("$state(\"{}\")", state);
    }
}


fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename).expect("Path not found!");
    Ok(BufReader::new(file).lines())
}


fn get_state(line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^QP::QState .*?::(\w*)").unwrap();
    }

    get_match(&RE, line)
}


fn get_event(line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*case (\w*)").unwrap();
    }

    get_match(&RE, line)
}


fn get_next_state(line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*Q_TRAN\(&(\w*)").unwrap();
    }

    get_match(&RE, line)
}


fn get_handled(line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*?(Q_HANDLED)\(").unwrap();
    }

    get_match(&RE, line)
}


fn get_match(re: &Regex, line: &str) -> Option<String> {
    let caps = re.captures(line);

    match caps {
        Some(c) => {
            let group = c.get(1);
            group.map(|g| g.as_str().to_string())
        },
        None => None,
    }

}



struct Transitions {
    transitions: Vec<Transition>,
}

impl Transitions {

    fn new() -> Transitions {
        Transitions {
            transitions: Vec::new(),
        }
    }
    
    fn add_transition(&mut self, transition: &Transition) {
        self.transitions.push(transition.clone());
    }

    fn output(&self) {
        println!("");

        for transition in &self.transitions {
            println!("{:?}", &transition);
        }
    }
}


#[derive(Debug, Clone)]
struct Transition {
    state: String,
    targets: Vec<String>,
    event: String,
}

impl Transition {

    fn new() -> Transition {
        Transition {
            state: "".to_string(),
            targets: Vec::new(),
            event: "".to_string(),
        }
    }
    
    fn initialise(&mut self, state: &str, event: &str) {
        self.state = state.to_string();
        self.event = event.to_string();
        self.targets = Vec::new();
    }   
    
    fn add_target(&mut self, target: &str) {
        self.targets.push(target.to_string());
    }   

    fn is_empty(&self) -> bool {
        self.state == ""
    }

}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg()]
   path: String,
}
