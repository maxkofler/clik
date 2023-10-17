Create a `clik::Command` from a function

# `clik_command`

This attribute-macro allows for easy annotation of functions as `clik::Command`s:

```rust
use clik_codegen::*;
use std::error::Error;

#[clik_command(my_command, "This is a command that returns the number that was input")]
#[clik_arg(number, "The number to display")]
fn my_command(state: &mut i32, number: i32) -> Result<(), Box<dyn Error>> {
    println!("You entered: {number}");
    *state = number;
    Ok(())
}
```

# Arguments

There are 2 arguments: `#[clik_command(<cmd_name>, <cmd_help>)]`

- `cmd_name`: The name of the command in the `CLI`

- `cmd_help`: The help string to display, describing the command

# Async

`clik` has support for async commands with the `async` feature.

If a function is marked with `async`, the `clik_command` macro will automatically create an async `clik::Command`.

# Attributes

There are some attributes that can be added **after** the `clik_command` macro:

### `clik_arg`:

This attribute describes an argument with a string: `#[clik_arg(<arg_name>, <arg_help>)]`

- `arg_name`: The `Ident` of the argument, this is checked

- `arg_help` A description for the argument

> **Note**
> 
> The argument documentation is checked, so describing non-existing arguments errors out
