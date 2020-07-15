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

### Motivation
This is the culmination of a few months of researching error handling in several different languages.
After learning about monadic error handling in Haskell and applying that to several other languages,
I found that Rust, in combination with the `std::str::FromStr` trait, came closest to the Haskell
implementation.

I also found that other prompt libraries like this one did not quite fit my needs: they were either
too complicated, too opaque, or did not account for validation internally.
So, I made this library as a simple, straightforward prompt library, complete with a `prompt()` method
that allows for a validator.
Everything I've done in this library is close to the Rust standard library.

Furthermore, thanks to dependency injection, you can even use this library for more than just stdin/stdout.
You could, for instance, pipe input from a file.

Meanwhile, I have tried to keep the error handling sensible: you can ignore internal errors with
the methods that return an `Option`, or you can use the `Result` versions to decide how to handle errors.

My library does not use any special traits; as you can see, `SafeParsable` merely uses `std::str::FromStr`,
`Sized`, `Copy`, and `Default`. This way, the types play nicely with the validator and have a sensible
result if you call `.unwrap_or_default()`.

You can also use only what you need: if, for instance, you only need to get a string from the user,
you can use just `get_line()`. You don't need to use `prompt()` at all.