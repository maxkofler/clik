#![doc = include_str!("../README.md")]
use std::{collections::HashMap, error::Error};

#[cfg(feature = "async")]
use std::{future::Future, pin::Pin};

mod cli;
mod command;
mod prelude;

// NOTE: Taken from shellfish
/// A shorthand for a synchronous function pointer
pub type Fn<T> = fn(&mut T, Vec<String>) -> Result<(), Box<dyn Error>>;

// NOTE: Taken from shellfish
/// A shorthand for an asynchronous function pointer
#[cfg(feature = "async")]
pub type AsyncFn<T> = fn(
    &mut T,
    Vec<String>,
) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn Error>>> + Send + '_>>;

// NOTE: Partially taken from shellfish
/// A function or callback can be either synchronous or asynchronous
pub enum FnType<T> {
    Sync(Fn<T>),
    #[cfg(feature = "async")]
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
    pub fn new(name: &'a str, help: &'a str, callback: Fn<T>) -> Self {
        Self {
            name,
            help,
            callback: FnType::Sync(callback),
            subcommands: HashMap::new(),
        }
    }

    /// Create a new command with a name and help string
    /// # Arguments
    /// * `name` - The name of the command, as typed into the CLI
    /// * `help` - The help string to describe this command
    /// * `callback` - The async funcion to call when there is a match for this command
    #[cfg(feature = "async")]
    pub fn new_async(name: &'a str, help: &'a str, callback: AsyncFn<T>) -> Self {
        Self {
            name,
            help,
            callback: FnType::Async(callback),
            subcommands: HashMap::new(),
        }
    }
}
