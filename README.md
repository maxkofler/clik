# CLIk

The `clik` crate provides an easy-to-use interactive CLI framework inspired by [shellfish](https://crates.io/crates/shellfish), expanded by new concepts, like subcommands and more!

# Example

```rust
use clik::{Command, FnType, CLI};
use rustyline::DefaultEditor;
use std::error::Error;

/// This is the state we want to store and reuse for all commands
struct LightState {
    /// For now we take a light that we toggle
    light: bool,
}

fn main() {
    // Create a new rustyline editor for reading in history
    let mut readline = DefaultEditor::new().unwrap();

    // Our CLI instance
    let mut cli = CLI::new(LightState { light: false });

    // Define the 'toggle' command
    let command = Command::new("toggle", "Toggles the light", FnType::Sync(toggle_function));

    // Add the new command to the CLI
    cli.add_command(command);

    // Handle all incoming lines
    loop {
        match readline.readline(">> ") {
            Ok(line) => {
                readline.add_history_entry(&line).unwrap();
                cli.handle(&line).unwrap()
            }
            Err(_) => break,
        }
    }
}

/// This is the function that gets called if the 'toggle' command is met
/// The 'state' variable is the one we previously passed to the CLI::new() function
/// The 'args' variable contains all the arguments that did not match on any other command
fn toggle_function(state: &mut LightState, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    state.light = !state.light;

    println!(
        "The light is now {}",
        match state.light {
            true => "ON",
            false => "OFF",
        }
    );

    Ok(())
}
```
