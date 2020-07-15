# prompto
### Simple, functional, monadic command-line prompts.
Heavily inspired by `readMaybe` from Haskell.

A simple module for handling user input and user input validation from input streams.

### Features
* Functional: Each of the methods that return an `Option` has a translation from Haskell
    which you can view in the documentation.
* Transparent: Uses only traits in the standard library and defines only a trait alias.
* (Mostly) Safe: All of the methods except for `prompt()` and `rprompt()` return `Option` or `Result`, and use
    the chaining methods on `Option` and `Result`.
* Usable beyond stdio: You can define a `Prompto` object for any combination of objects that implement `BufRead` and `Write`.
* Use only what you need: don't need validation? Just use `input()`. Need only a string? Just use `get_line()`.

### Usage

Say you'd like to get a number from the user via `stdin` that is in the closed interval [1, 100].
To do this with Prompto, you first define the Prompto object and then call the `prompt()` method on it, like so:
```no_run
use prompto::Prompto;

let stdio = std::io::stdin();
let input = stdio.lock();
let output = std::io::stdout();

let mut prompto = Prompto {
    reader: input,
    writer: output
};

let res: u32 = prompto.prompt("Please enter a number between 1 and 100: ", |x| 1 <= x && x <= 100);
```
If you only need a string, you can use `get_line()` instead:
```no_run
use prompto::Prompto;
let stdio = std::io::stdin();
let input = stdio.lock();
let output = std::io::stdout();

let mut prompto = Prompto {
    reader: input,
    writer: output
};

let name = prompto.get_line("What is your name? ").unwrap();
```