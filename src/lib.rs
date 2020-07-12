//! # Prompto
//!
//! Simple, functional, monadic command-line prompts.
//! Heavily inspired by `readMaybe` from Haskell.

/// # SafeParsable
///
/// Defines a trait that is safe to parse from a string and has a default value
/// for `.unwrap_or_default()`
pub trait SafeParsable: Sized + Copy + Default + std::str::FromStr {}

impl<T> SafeParsable for T where T: Sized + Copy + Default + std::str::FromStr {}

pub mod result {
    //! # Result
    //!
    //! This module includes explicit errors, as opposed to the maybe module,
    //! which simply converts errors into None.
    use crate::*;
    use thiserror::Error;

    /// # PromptError
    ///
    /// Describes the kinds of errors these functions can throw.
    #[derive(Error, Debug)]
    pub enum PromptError {
        /// ### StdinError
        ///
        /// Throws in the event that `prompt_line()` fails.
        #[error("Failure reading line from stdin")]
        StdinError(#[from] std::io::Error),

        /// ### ReadError
        ///
        /// Throws in the event that `read()` fails.
        #[error("Failure converting string to data type")]
        ReadError,
    }

    /// Get a newline-terminated string from stdin,
    /// returning `StdinError` if `std::io::stdout.flush()` fails
    /// or if `std::io::stdin().read_line()` fails.
    ///
    /// # Arguments
    /// * `msg` – a message to display to the user.
    ///
    /// # Example
    /// ```
    /// use prompto::result::*;
    /// let res = get_line("What's your name?");
    /// match res {
    ///     Ok(s)  => println!("Nice to meet you, {}!", s),
    ///     Err(e) => eprintln!("I'm sorry! I got an error: {}", e)
    /// }
    /// ```
    ///
    /// The Haskell spec for this function is:
    /// ```hs
    /// promptLine :: String -> IO String
    /// promptLine msg = do
    ///     putStr msg
    ///     hFlush stdout
    ///     getLine
    /// ```
    pub fn get_line(msg: &str) -> Result<String, PromptError> {
        use std::io::Write;

        print!("{}", msg);

        std::io::stdout()
            .flush()
            .map_err(|source| PromptError::StdinError(source))?;

        let mut buffer: String = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .map_err(|source| PromptError::StdinError(source))?;

        Ok(buffer.trim_end().to_owned())
    }

    /// Attempts to convert the contents of a string to a type
    /// that implements `std::str::FromStr`.
    /// Returns `ReadError` if conversion failed.
    /// More or less a wrapper around `parse`.
    ///
    /// # Arguments
    /// * `arg` – string to attempt to convert.
    ///
    /// # Example
    /// ```
    /// use prompto::result::*;
    /// let res = read::<i32>("32").map(|x| x * 2).unwrap();
    /// println!("Value of res: {}.", res);
    /// ```
    ///
    /// The Haskell spec for this function is:
    /// ```hs
    /// readMaybe :: Read a => String -> Maybe a
    /// ```
    pub fn read<T>(line: &str) -> Result<T, PromptError>
    where
        T: std::str::FromStr,
    {
        line.parse::<T>().map_err(|_| PromptError::ReadError)
    }

    /// Gets a value of type `T` from the user, where `T` defines a default value
    /// and implements `std::str::FromStr`.
    /// This function returns `PromptError` (in other words, forwards the error from `get_line()` or `read()`)
    /// if it is not able to parse the user's input into `T`.
    ///
    /// # Arguments
    /// `msg` – a message to display to the user.
    ///
    /// # Example
    /// ```
    /// use prompto::result::*;
    /// let res = input::<i32>("Please enter a number: ");
    /// match res {
    ///     Ok(x)  => println!("Got {}.", x),
    ///     Err(e) => eprintln!("Got invalid input! {}", e)
    /// }
    /// ```
    ///
    /// I designed this function as a type-safe analogue of Python's `input` function.
    /// However, this function returns an Option because it has no way to validate
    /// the user's input.
    ///
    /// The Haskell spec for this function is:
    /// ```hs
    /// getLine >>= pure . (Text.Read.readMaybe :: String -> Maybe Int)
    /// ```
    pub fn input<T>(msg: &str) -> Result<T, PromptError>
    where
        T: SafeParsable,
    {
        get_line(msg).and_then(|s| read::<T>(&s))
    }

    /// Prompts the user for a value of type `T` and validates it against `validator`.
    /// If input or validation fails, this function re-prompts the user.
    ///
    /// # Arguments
    /// * `msg` – a message to display to the user.
    /// * `validator` – a function which immutably borrows a single argument of type `T` and returns a `bool`.
    ///
    /// # Example
    /// ```
    /// use prompto::result::*;
    /// let res: u32 = prompt("Please enter a number between 1 and 100: ", |x| 1 <= x && x <= 100);
    /// ```
    ///
    /// Incidentally, the behaviour of this function is nearly the same as its counterpart in the option module.
    /// Since this function does not provide any hooks into the errors the functions above can generate,
    /// you are probably better off using the `Option` version, but I am providing this version anyway for completeness.
    pub fn prompt<T, F>(msg: &str, validator: F) -> T
    where
        T: SafeParsable,
        F: Fn(T) -> bool,
    {
        loop {
            let res: T = match input::<T>(msg) {
                Ok(val) => val,
                Err(_) => {
                    println!("Invalid input. Please try again.");
                    continue;
                }
            };

            if validator(res) {
                break res;
            } else {
                println!("Invalid input! Please try again.");
            }
        }
    }
}

pub mod maybe {
    //! # Maybe
    //!
    //! These functions wrap their results in `Option`.
    //! In the event of an error, these functions return `None`.
    //! If you need more fine-grained control over the errors, see the sibling result module.
    use crate::*;

    /// Get a newline-terminated string from stdin,
    /// returning `None` if `std::io::stdout.flush()` fails
    /// or if `std::io::stdin().read_line()` fails.
    ///
    /// # Arguments
    /// * `msg` – a message to display to the user.
    ///
    /// # Example
    /// ```
    /// use prompto::maybe::*;
    /// let res = get_line("What's your name?");
    /// match res {
    ///     Some(s) => println!("Nice to meet you, {}!", s),
    ///     None    => println!("I'm sorry!")
    /// }
    /// ```
    ///
    /// The Haskell spec for this function is:
    /// ```hs
    /// promptLine :: String -> IO String
    /// promptLine msg = do
    ///     putStr msg
    ///     hFlush stdout
    ///     getLine
    /// ```
    pub fn get_line(msg: &str) -> Option<String> {
        use std::io::Write;

        print!("{}", msg);

        // Force output to stdout before reading from stdin
        match std::io::stdout().flush() {
            Ok(()) => (),
            Err(_) => return None,
        }

        let mut buffer: String = String::new();

        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(_) => return None,
        }

        Some(buffer.trim_end().to_owned())
    }

    /// Attempts to convert the contents of a string to a type
    /// that implements `std::str::FromStr`.
    /// Returns `None` if conversion failed.
    /// More or less a wrapper around `parse`.
    ///
    /// # Arguments
    /// * `arg` – string to attempt to convert.
    ///
    /// # Example
    /// ```
    /// use prompto::maybe::*;
    /// let res = read::<i32>("32").map(|x| x * 2).unwrap();
    /// println!("Value of res: {}.", res);
    /// ```
    ///
    /// The Haskell spec for this function is:
    /// ```hs
    /// readMaybe :: Read a => String -> Maybe a
    /// ```
    pub fn read<T>(arg: &str) -> Option<T>
    where
        T: std::str::FromStr,
    {
        match arg.parse::<T>() {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }

    /// Gets a value of type `T` from the user, where `T` defines a default value
    /// and implements `std::str::FromStr`.
    /// This function returns `None` if it is not able to parse the user's input into `T`.
    ///
    /// # Arguments
    /// `msg` – a message to display to the user.
    ///
    /// # Example
    /// ```
    /// use prompto::maybe::*;
    /// let res = input::<i32>("Please enter a number: ");
    /// match res {
    ///     Some(x) => println!("Got {}.", x),
    ///     None => println!("Got invalid input!")
    /// }
    /// ```
    ///
    /// I designed this function as a type-safe analogue of Python's `input` function.
    /// However, this function returns an Option because it has no way to validate
    /// the user's input.
    ///
    /// The Haskell spec for this function is:
    /// ```hs
    /// getLine >>= pure . (Text.Read.readMaybe :: String -> Maybe Int)
    /// ```
    pub fn input<T>(msg: &str) -> Option<T>
    where
        T: SafeParsable,
    {
        get_line(msg).and_then(|s| read::<T>(&s))
    }

    /// Prompts the user for a value of type `T` and validates it against `validator`.
    /// If input or validation fails, this function re-prompts the user.
    ///
    /// # Arguments
    /// * `msg` – a message to display to the user.
    /// * `validator` – a function which immutably borrows a single argument of type `T` and returns a `bool`.
    ///
    /// # Example
    /// ```
    /// use prompto::maybe::*;
    /// let res: u32 = prompt("Please enter a number between 1 and 100: ", |x| 1 <= x && x <= 100);
    /// ```
    pub fn prompt<T, F>(msg: &str, validator: F) -> T
    where
        T: SafeParsable,
        F: Fn(T) -> bool,
    {
        loop {
            let res: T = match input::<T>(msg) {
                Some(val) => val,
                None => {
                    println!("Invalid input. Please try again.");
                    continue;
                }
            };

            if validator(res) {
                break res;
            } else {
                println!("Invalid input! Please try again.");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    // From https://rust-lang-nursery.github.io/rust-cookbook/text/string_parsing.html
    #[derive(Debug, PartialEq)]
    struct RGB {
        r: u8,
        g: u8,
        b: u8,
    }

    impl FromStr for RGB {
        type Err = std::num::ParseIntError;

        // Parses a color hex code of the form '#rRgGbB..' into an
        // instance of 'RGB'
        fn from_str(hex_code: &str) -> Result<Self, Self::Err> {
            // u8::from_str_radix(src: &str, radix: u32) converts a string
            // slice in a given base to u8
            let r: u8 = u8::from_str_radix(&hex_code[1..3], 16)?;
            let g: u8 = u8::from_str_radix(&hex_code[3..5], 16)?;
            let b: u8 = u8::from_str_radix(&hex_code[5..7], 16)?;

            Ok(RGB { r, g, b })
        }
    }

    #[test]
    fn sanity_checks() {
        // A string with a valid integer should always succeed.
        assert!(maybe::read::<i32>("32").is_some());

        // A string with any number of non-numeric characters should never succeed,
        // even if any part of the string *could* be valid.
        assert!(maybe::read::<i32>("56 fdfs θ gx二éfs sdf34ごν53 df3dfsd2").is_none());

        // Implicit widening conversions are okay...
        assert!(maybe::read::<f32>("32").is_some());

        // But truncating conversions are not!
        assert!(maybe::read::<i32>("32.32").is_none());
    }

    #[test]
    fn composite_type_checks() {
        // Read should behave the same way as calling parse or calling from_str directly on the type.
        let call_through_trait = RGB::from_str(r"#fa7268").unwrap()
            == RGB {
                r: 250,
                g: 114,
                b: 104,
            };
        let call_through_maybe = maybe::read::<RGB>(r"#fa7268").unwrap()
            == RGB {
                r: 250,
                g: 114,
                b: 104,
            };
        assert_eq!(call_through_trait, call_through_maybe);

        assert!(maybe::read::<RGB>("gkhgkjyfa7©268").is_none());
    }

    #[test]
    fn chaining_checks() {
        let res = maybe::read::<i32>("32").map(|x| x * 2).unwrap();

        assert_eq!(res, 64);
    }
}
