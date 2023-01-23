use std::fmt;

use crate::states::State;

pub struct Transitions(Vec<EventTransitions>);

impl Transitions {
    pub fn new() -> Transitions {
        Transitions(Vec::new())
    }

    pub fn add_transition(&mut self, transition: &EventTransitions) {
        self.0.push(transition.clone());
    }
}

impl fmt::Debug for Transitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.0 {
            writeln!(f, "{:?}", item);
        }
        Ok(())
    }
}

impl fmt::Display for Transitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for event_transitions in &self.0 {
            if event_transitions.should_be_output() {
                writeln!(f, "{}", event_transitions);
            }
        }
        Ok(())
    }
}

// Transitions for a single state and event
#[derive(Debug, Clone)]
pub struct EventTransitions {
    state_name: String,
    event_name: String,
    target_names: Vec<String>,
}

impl EventTransitions {
    pub fn new() -> EventTransitions {
        EventTransitions {
            state_name: "".to_string(),
            event_name: "".to_string(),
            target_names: Vec::new(),
        }
    }
    
    pub fn initialise(&mut self, state: &State, event: &str) {
        self.state_name = state.name.clone();
        self.event_name = event.to_string();
        self.target_names = Vec::new();
    }
    
    pub fn add_target(&mut self, target: &State) {
        self.target_names.push(target.name.clone());
    }
    
    pub fn is_empty(&self) -> bool {
        self.state_name.is_empty()
    }

    pub fn should_be_output(&self) -> bool {
        self.is_simple() || self.is_choice() || self.is_choice_choice()
    }

    fn is_simple(&self) -> bool {
        self.is_valid() && self.target_names.len() == 1 && self.target_names[0] != self.state_name
    }
    
    fn is_choice(&self) -> bool {
        self.is_valid() && self.target_names.len() == 2
    }

    fn is_choice_choice(&self) -> bool {
        self.is_valid() && self.target_names.len() == 3
    }

    fn is_valid(&self) -> bool {
        !self.target_names.is_empty() && self.event_name != "Q_ENTRY_SIG" && self.event_name != "Q_EXIT_SIG"
    }
}

impl fmt::Display for EventTransitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_simple() {
            write!(f, "$transition(\"{}\", \"{}\", \"{}\"", &self.state_name, &self.event_name, &self.target_names[0]);
        }
        else if self.is_choice() {
            write!(f, "$choice_transition(\"{}\", \"{}\"", &self.state_name, &self.event_name);
            
            for target in &self.target_names {
                write!(f, ", \"{}\"", target);
            }
        }
        else if self.is_choice_choice() {
            write!(f, "$choice_choice_transition(\"{}\", \"{}\"", &self.state_name, &self.event_name);
            
            for target in &self.target_names {
                write!(f, ", \"{}\"", target);
            }
        }

        write!(f, ");");
        Ok(())
    }
}
