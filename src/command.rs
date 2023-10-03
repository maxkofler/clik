use std::{error::Error, fmt::Display};

#[cfg(feature = "async")]
use async_recursion::async_recursion;

use crate::{Command, FnType};

#[derive(Debug)]
struct AsyncHandleError {}

impl Display for AsyncHandleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Tried to use async callback with sync handler!")
    }
}
impl Error for AsyncHandleError {}

impl<'a, T: Send> Command<'a, T> {
    /// Handle a prompt and see if there is some match. If there is an `async` callback, this will fail
    /// # Arguments
    /// * `state` - The state to provide to the callback if a match is found
    /// * `prompt` - A collection of strings that form the prompt
    pub fn handle(&self, state: &mut T, prompt: &[&str]) -> Result<(), Box<dyn Error>> {
        if let Some(cmd) = prompt.first() {
            if let Some(subcommand) = self.subcommands.get(cmd) {
                return subcommand.handle(state, &prompt[1..prompt.len()]);
            }
        }

        match self.callback {
            FnType::Sync(f) => (f)(state, prompt.iter().map(|a| a.to_string()).collect()),
            #[cfg(feature = "async")]
            FnType::Async(_) => Err(Box::new(AsyncHandleError {})),
        }
    }

    /// Handle a prompt asynchronously and see if there is some match
    /// # Arguments
    /// * `state` - The state to provide to the callback if a match is found
    /// * `prompt` - A collection of strings that form the prompt
    #[cfg(feature = "async")]
    #[async_recursion]
    pub async fn handle_async(&self, state: &mut T, prompt: &[&str]) -> Result<(), Box<dyn Error>> {
        if let Some(cmd) = prompt.first() {
            if let Some(subcommand) = self.subcommands.get(cmd) {
                return subcommand
                    .handle_async(state, &prompt[1..prompt.len()])
                    .await;
            }
        }

        match self.callback {
            FnType::Sync(f) => (f)(state, prompt.iter().map(|a| a.to_string()).collect()),
            FnType::Async(f) => (f)(state, prompt.iter().map(|a| a.to_string()).collect()).await,
        }
    }

    /// Add a new subcommand to this command
    /// # Arguments
    /// * `command` - The command to add
    pub fn add_subcommand(&mut self, command: Command<'a, T>) -> Option<Command<'_, T>> {
        self.subcommands.insert(command.name, command)
    }

    /// Provide some information about the command
    pub fn info(&self, f: &mut std::fmt::Formatter<'_>, depth: usize) -> std::fmt::Result {
        let indent = "|  ".repeat(depth);
        let string = format!("{}|-- {} ", indent, self.name);

        writeln!(f, "{:.<35} {}", string, self.help)?;

        for c in &self.subcommands {
            c.1.info(f, depth + 1)?;
        }

        Ok(())
    }
}
