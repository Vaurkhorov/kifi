pub trait Output {
    fn add(&mut self, output: String);
    fn add_str(&mut self, output: &str);
    fn print(&self) -> Option<Vec<String>>;
}

pub struct ConsoleOutput {
    output: Vec<String>,
}

impl ConsoleOutput {
    pub fn new() -> Self {
        ConsoleOutput { output: Vec::new() }
    }
}

impl Default for ConsoleOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Output for ConsoleOutput {
    fn add(&mut self, output: String) {
        self.output.push(output);
    }

    fn add_str(&mut self, output: &str) {
        self.output.push(output.to_string());
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
        DebugOutput { output: Vec::new() }
    }
}

impl Default for DebugOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl Output for DebugOutput {
    fn add(&mut self, output: String) {
        self.output.push(output);
    }

    fn add_str(&mut self, output: &str) {
        self.output.push(output.to_string());
    }

    fn print(&self) -> Option<Vec<String>> {
        Some(self.output.clone())
    }
}
