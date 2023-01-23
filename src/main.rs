mod states;
mod transitions;

use states::{States, State};
use transitions::{Transitions, EventTransitions};

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
    let mut states = States::new();
    let mut transitions = Transitions::new();
    
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            source_lines.push(line?.clone());
        }
    }

    let mut event_transitions = EventTransitions::new();
    let mut current_state = State::new("");
    
    for source_line in source_lines {

        if let Some(state_name) = get_state(&source_line) {
            let state = State::new(&state_name);
            current_state = state.clone();
        }
        else if let Some(parent_name) = get_parent(&source_line) {
            let parent = State::new(&parent_name);
            current_state.set_parent(&parent);
            states.add_state(&current_state.clone());
        }
        else if let Some(event) = get_event(&source_line) {
            if !&event_transitions.is_empty() {
                transitions.add_transition(&event_transitions);
            }
            event_transitions.initialise(&current_state, &event);
        }
        else if let Some(target_name) = get_target(&source_line) {
            let target = State::new(&target_name);
            event_transitions.add_target(&target.clone());
        }
        else if is_handled(&source_line) {                    
            event_transitions.add_target(&current_state);
        }
        else {}
    }

    // add the final transition
    transitions.add_transition(&event_transitions);

    println!("{:?}", states);
    println!("{:?}", transitions);

    println!("{}", states);
    println!("{}", transitions);

    Ok(())
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


fn get_target(line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*Q_TRAN\(&(\w*)").unwrap();
    }

    get_match(&RE, line)
}


fn get_parent(line: &str) -> Option<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*Q_SUPER\(&(\w*)").unwrap();
    }

    get_match(&RE, line)
}


fn is_handled(line: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^.*?(Q_HANDLED)\(").unwrap();
    }

    get_match(&RE, line).is_some()
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg()]
   path: String,
}
