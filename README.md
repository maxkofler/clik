# CLIk

The `clik` crate provides an easy-to-use interactive CLI framework inspired by [shellfish](https://crates.io/crates/shellfish), expanded by new concepts, like subcommands and more!

# Example

```rust
use clik::{clik_command, Command, CLI};
use rustyline::DefaultEditor;

/// This is the state we want to store and reuse for all commands
struct EchoState {
    /// We store the last number that was typed
    number: i32,
}

fn main() {
    // Create a new rustyline editor for reading in history
    let mut readline = DefaultEditor::new().unwrap();

    // Our CLI instance
    let mut cli = CLI::new(EchoState { number: 0 });

    // Add the 'echo' command to the CLI
    cli.add_command(echo_command());

    // Handle all incoming lines
    loop {
        match readline.readline(">> ") {
            Ok(line) => {
                readline.add_history_entry(&line).unwrap();

                // Handle the line using the CLI struct and respond to errors
                match cli.handle(&line) {
                    Ok(()) => {}
                    Err(e) => println!("ERROR: {e}"),
                }
            }
            Err(_) => break,
        }
    }
}

/// This is the function that gets called if the 'echo' command is met
/// The 'state' variable is the one we previously passed to the CLI::new() function
/// All the additional args can be parsed by using the `clik_command` macro,
/// but they need to implement `FromStr`.
#[clik_command(echo, "Prints out the supplied number")]
/// We can use multiple `clik_arg` attributes after the `clik_command` macro
/// to describe our arguments.
#[clik_arg(number, "The number to echo back")]
fn echo_command(state: &mut EchoState, number: i32) {
    println!("Updating number from {} to {}", state.number, number);

    state.number = number;

    Ok(())
}
```

# Optional features

- `async` - Allow async functions and commands
