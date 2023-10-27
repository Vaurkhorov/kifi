pub trait Output {
    fn add(&mut self, output: String);
    fn print(&self) -> Option<Vec<String>>;
}

pub struct ConsoleOutput {
    output: Vec<String>,
}

impl ConsoleOutput {
    pub fn new() -> Self {
        ConsoleOutput {
            output: Vec::new(),
        }
    }
}

impl Output for ConsoleOutput {
    fn add(&mut self, output: String) {
        self.output.push(output);
    }

    fn print(&self) -> Option<Vec<String>> {
        for line in &self.output {
            println!("{}", line);
        }
        None
    }
}

pub struct DebugOutput {
    output: Vec<String>,
}

impl DebugOutput {
    pub fn new() -> Self {
        DebugOutput {
            output: Vec::new(),
        }
    }
}

impl Output for DebugOutput {
    fn add(&mut self, output: String) {
        self.output.push(output);
    }

    fn print(&self) -> Option<Vec<String>> {
        Some(self.output.clone())
    }
}