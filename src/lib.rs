//! # Prompto - Simple, functional, monadic command-line prompts.
//! Heavily inspired by `readMaybe` from Haskell.
//!
//! This crate provides an object that can hold input/output handles,
//! and several methods that can be called on the input stream to get input,
//! including validating input.
//! These methods depend on `std::str::FromStr` being defined on the type.
//!
//! # Motivation
//! This is the culmination of a few months of researching error handling in several different languages.
//! After learning about monadic error handling in Haskell and applying that to several other languages,
//! I found that Rust, in combination with the `std::str::FromStr` trait, came closest to the Haskell
//! implementation.
//!
//! I also found that other prompt libraries like this one did not quite fit my needs: they were either
//! too complicated, too opaque, or did not account for validation internally.
//! So, I made this library as a simple, straightforward prompt library, complete with a `prompt()` method
//! that allows for a validator.
//! Everything I've done in this library is close to the Rust standard library.
//!
//! Furthermore, thanks to dependency injection, you can even use this library for more than just stdin/stdout.
//! You could, for instance, pipe input from a file.
//!
//! Meanwhile, I have tried to keep the error handling sensible: you can ignore internal errors with
//! the methods that return an `Option`, or you can use the `Result` versions to decide how to handle errors.
//!
//! My library does not use any special traits; as you can see, `SafeParsable` merely uses `std::str::FromStr`,
//! `Sized`, `Copy`, and `Default`. This way, the types play nicely with the validator and have a sensible
//! result if you call `.unwrap_or_default()`.
//!
//! You can also use only what you need: if, for instance, you only need to get a string from the user,
//! you can use just `get_line()`. You don't need to use `prompt()` at all.
//!
//! # Examples
//! Say you'd like to get a number from the user via `stdin` that is in the closed interval [1, 100].
//! To do this with Prompto, you first define the Prompto object and then call the `prompt()` method on it, like so:
//! ```
//! use prompto::Prompto;
//!
//! let stdio = std::io::stdin();
//! let input = stdio.lock();
//! let output = std::io::stdout();
//!
//! let mut prompto = Prompto {
//!     reader: input,
//!     writer: output
//! };
//!
//! let res: u32 = prompto.prompt("Please enter a number between 1 and 100: ", |x| 1 <= x && x <= 100);
//! ```
//! If you only need a string, you can use `get_line()` instead:
//! ```
//! use prompto::Prompto;
//! let stdio = std::io::stdin();
//! let input = stdio.lock();
//! let output = std::io::stdout();
//!
//! let mut prompto = Prompto {
//!     reader: input,
//!     writer: output
//! };
//!
//! let name = prompto.get_line("What is your name? ").unwrap();
//! ```

pub use self::prompto::Prompto;

pub mod prompto;

mod tests;
