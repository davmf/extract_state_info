use std::fmt;

pub struct States(Vec<State>);

impl States {
    pub fn new() -> States {
        States(Vec::new())
    }

    pub fn add_state(&mut self, state: &State) {
        self.0.push(state.clone());
    }
}

impl fmt::Debug for States {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for state in &self.0 {
            writeln!(f, "{:?}", state).expect("writeln! issue");
        }
        Ok(())
    }
}

impl fmt::Display for States {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for state in &self.0 {
            writeln!(f, "{}", state).expect("writeln! issue");
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub name: String,
    parent_name: String,
}

impl State {
    pub fn new(name: &str) -> State {
        State {
            name: name.to_string(),
            parent_name: "".to_string(),
        }
    }

    pub fn set_parent(&mut self, parent: &State) {
        self.parent_name = parent.name.clone();
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}  $state(\"{}\")", self.parent_name, self.name)
    }
}
