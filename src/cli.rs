use std::{error::Error, fmt::Display};

use crate::{Command, CLI};

impl<'a, T: Send> CLI<'a, T> {
    /// Handle an input line. This line gets split up and then processed by all the commands
    /// # Arguments
    /// * `line` - The input line to use for execution
    pub fn handle(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let prompt = split_line(line);

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
        let prompt = split_line(line);

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

/// Splits a line into seperate parts according to following rules:
/// - Split at ' ' (space)
/// - If a string is quoted ("), there will be no split at the space
/// # Arguments
/// * `line` - The line to split up
fn split_line(line: &str) -> Vec<&str> {
    let mut in_string = false;
    let mut split: Vec<&str> = Vec::new();
    let mut start: usize = 0;

    for (i, c) in line.chars().enumerate() {
        if c == '"' {
            if !in_string {
                start += 1
            } else {
                start -= 1
            }
            in_string = !in_string;
            continue;
        }

        if c == ' ' && !in_string {
            split.push(&line[start..i]);
            start = i + 1;
        }
    }
    split.push(&line[start..line.len()]);

    split.retain(|s| s != &"");
    split
        .iter()
        .map(|s| s.trim_start_matches('"').trim_end_matches('"'))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_split() {
        let line = "help";
        assert_eq!(split_line(line), vec!["help"]);
    }

    #[test]
    fn test_split_spaces() {
        let line = "help cmd";
        assert_eq!(split_line(line), vec!["help", "cmd"])
    }

    #[test]
    fn test_quotes_no_split() {
        let line = "\"help\"";
        assert_eq!(split_line(line), vec!["help"]);
    }

    #[test]
    fn test_quotes_split_spaces() {
        let line = "\"help\" \"cmd\"";
        assert_eq!(split_line(line), vec!["help", "cmd"]);
    }
}
