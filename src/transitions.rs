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
            writeln!(f, "{:?}", item).expect("writeln! issue");
        }
        Ok(())
    }
}

impl fmt::Display for Transitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for event_transitions in &self.0 {
            if event_transitions.should_be_output() {
                writeln!(f, "{}", event_transitions).expect("writeln! issue");
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
        self.is_simple() || self.is_1_choice() || self.are_2_choices() || self.are_3_choices()
    }

    fn is_simple(&self) -> bool {
        self.is_valid() && self.target_names.len() == 1 && self.target_names[0] != self.state_name
    }

    fn is_1_choice(&self) -> bool {
        self.is_valid() && self.target_names.len() == 2
    }

    fn are_2_choices(&self) -> bool {
        self.is_valid() && self.target_names.len() == 3
    }

    fn are_3_choices(&self) -> bool {
        self.is_valid() && self.target_names.len() == 4
    }

    fn are_too_many_choices(&self) -> bool {
        self.is_valid() && self.target_names.len() > 4
    }

    fn is_valid(&self) -> bool {
        !self.target_names.is_empty()
            && self.event_name != "Q_ENTRY_SIG"
            && self.event_name != "Q_EXIT_SIG"
    }
}

impl fmt::Display for EventTransitions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_simple() {
            write!(
                f,
                "$transition(\"{}\", $event(\"{}\"), \"{}\"",
                &self.state_name, &self.event_name, &self.target_names[0]
            ).expect("writeln! issue");
        } else if self.is_1_choice() {
            write!(
                f,
                "$transitions_1_choice(\"{}\", $event(\"{}\")",
                &self.state_name, &self.event_name
            ).expect("writeln! issue");

            for target in &self.target_names {
                write!(f, ", \"{}\"", target).expect("writeln! issue");
            }
        } else if self.are_2_choices() {
            write!(
                f,
                "$transitions_2_choices(\"{}\", $event(\"{}\")",
                &self.state_name, &self.event_name
            ).expect("writeln! issue");

            for target in &self.target_names {
                write!(f, ", \"{}\"", target).expect("writeln! issue");
            }
        } else if self.are_3_choices() {
            write!(
                f,
                "$transitions_3_choices(\"{}\", $event(\"{}\")",
                &self.state_name, &self.event_name
            ).expect("writeln! issue");

            for target in &self.target_names {
                write!(f, ", \"{}\"", target).expect("writeln! issue");
            }
        } else if self.are_too_many_choices() {
            write!(f, "{} {}", &self.state_name, &self.event_name).expect("writeln! issue");
            panic!(" - too many choices!");
        }

        write!(f, ")").expect("writeln! issue");
        Ok(())
    }
}
