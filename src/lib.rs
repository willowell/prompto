//! # Promptor - Simple, functional, monadic command-line prompts.
//! Heavily inspired by `readMaybe` from Haskell.
//!
//! This crate provides an object that can hold input/output handles,
//! and several methods that can be called on the input stream to get input,
//! including validating input.
//! These methods depend on `std::str::FromStr` being defined on the type.
//!
//! # Examples
//! Say you'd like to get a number from the user via `stdin` that is in the closed interval [1, 100].
//! To do this with Promptor, you first define the Promptor object and then call the `prompt()` method on it, like so:
//! ```
//! use promptor::Promptor;
//!
//! let stdio = std::io::stdin();
//! let input = stdio.lock();
//! let output = std::io::stdout();
//!
//! let mut promptor = Promptor {
//!     reader: input,
//!     writer: output
//! };
//!
//! let res: u32 = promptor.prompt("Please enter a number between 1 and 100: ", |x| 1 <= x && x <= 100);
//! ```
//! If you only need a string, you can use `get_line()` instead:
//! ```
//! use promptor::Promptor;
//! let stdio = std::io::stdin();
//! let input = stdio.lock();
//! let output = std::io::stdout();
//!
//! let mut promptor = Promptor {
//!     reader: input,
//!     writer: output
//! };
//!
//! let name = promptor.get_line("What is your name? ").unwrap();
//! ```

pub use self::promptor::Promptor;

pub mod promptor;

#[cfg(test)]
mod tests;
