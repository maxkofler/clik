use std::{error::Error, fmt::Display};

use crate::{Command, CLI};

impl<'a, T: Send> CLI<'a, T> {
    /// Handle an input line. This line gets split up and then processed by all the commands
    /// # Arguments
    /// * `line` - The input line to use for execution
    pub fn handle(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let prompt: Vec<&str> = line.split(' ').collect();

        if let Some(first) = prompt.first() {
            if let Some(command) = self.commands.get(first) {
                command.handle(&mut self.state, &prompt[1..prompt.len()])?
            }
        }

        Ok(())
    }

    /// Handle an input line asynchronously. This line gets split up and then processed by all the commands
    /// # Arguments
    /// * `line` - The input line to use for execution
    pub async fn handle_async(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let prompt: Vec<&str> = line.split(' ').collect();

        if let Some(first) = prompt.first() {
            if let Some(command) = self.commands.get(first) {
                command
                    .handle_async(&mut self.state, &prompt[1..prompt.len()])
                    .await?
            }
        }

        Ok(())
    }

    /// Add a new command to this CLI
    /// # Arguments
    /// * `command` - The command to add
    pub fn add_command(&mut self, command: Command<'a, T>) -> Option<Command<'_, T>> {
        self.commands.insert(command.name, command)
    }
}

impl<'a, T: Send> Display for CLI<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Available commands: \n")?;

        for c in &self.commands {
            c.1.info(f, 0)?;
        }

        Ok(())
    }
}
