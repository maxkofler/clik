//! CLIk - A simple to use interactive CLI framework that just makes click
//!
//! This crate is inspired by [shellfish](https://crates.io/crates/shellfish) but extends it with new concepts like subcommands
//!
//! # Example
//! ```
//! use clik::{Command, FnType, CLI};
//! use rustyline::DefaultEditor;
//! use std::error::Error;
//!
//! /// This is the state we want to store and reuse for all commands
//! struct LightState {
//!     /// For now we take a light that we toggle
//!     light: bool,
//! }
//!
//! fn main() {
//!     // Create a new rustyline editor for reading in history
//!     let mut readline = DefaultEditor::new().unwrap();
//!
//!     // Our CLI instance
//!     let mut cli = CLI::new(LightState { light: false });
//!
//!     // Define the 'toggle' command
//!     let command = Command::new("toggle", "Toggles the light", FnType::Sync(toggle_function));
//!
//!     // Add the new command to the CLI
//!     cli.add_command(command);
//!
//!     // Handle all incoming lines
//!     loop {
//!         match readline.readline(">> ") {
//!             Ok(line) => {
//!                 readline.add_history_entry(&line).unwrap();
//!                 cli.handle(&line).unwrap()
//!             }
//!             Err(_) => break,
//!         }
//!     }
//! }
//!
//! /// This is the function that gets called if the 'toggle' command is met
//! /// The 'state' variable is the one we previously passed to the CLI::new() function
//! /// The 'args' variable contains all the arguments that did not match on any other command
//! fn toggle_function(state: &mut LightState, args: Vec<String>) -> Result<(), Box<dyn Error>> {
//!     state.light = !state.light;
//!
//!     println!(
//!         "The light is now {}",
//!         match state.light {
//!             true => "ON",
//!             false => "OFF",
//!         }
//!     );
//!
//!     Ok(())
//! }
//! ```

use std::{collections::HashMap, error::Error, future::Future, pin::Pin};

mod cli;
mod command;
mod prelude;

// NOTE: Taken from shellfish
/// A shorthand for a synchronous function pointer
pub type Fn<T> = fn(&mut T, Vec<String>) -> Result<(), Box<dyn Error>>;

// NOTE: Taken from shellfish
/// A shorthand for an asynchronous function pointer
pub type AsyncFn<T> = fn(
    &mut T,
    Vec<String>,
) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + '_>>;

// NOTE: Partially taken from shellfish
/// A function or callback can be either synchronous or asynchronous
pub enum FnType<T> {
    Sync(Fn<T>),
    Async(AsyncFn<T>),
}

/// The `CLI` struct is the main handle for a CLI interface that holds all the commands
pub struct CLI<'a, T: Send> {
    /// The state that gets represented to callbacks
    state: T,
    /// All the available commands
    commands: HashMap<&'a str, Command<'a, T>>,
}

impl<'a, T: Send> CLI<'_, T> {
    /// Create a new CLI with an internal state
    /// # Arguments
    /// * `state` - The state to provide to the callbacks
    pub fn new(state: T) -> Self {
        Self {
            state,
            commands: HashMap::new(),
        }
    }
}

/// A command that can have some subcommands
pub struct Command<'a, T> {
    name: &'a str,
    help: &'a str,
    callback: FnType<T>,
    subcommands: HashMap<&'a str, Command<'a, T>>,
}

impl<'a, T: Send> Command<'a, T> {
    /// Create a new command with a name and help string
    /// # Arguments
    /// * `name` - The name of the command, as typed into the CLI
    /// * `help` - The help string to describe this command
    /// * `callback` - The funcion to call when there is a match for this command
    pub fn new(name: &'a str, help: &'a str, callback: FnType<T>) -> Self {
        Self {
            name,
            help,
            callback,
            subcommands: HashMap::new(),
        }
    }
}
