//! # Prompto
//!
//! You can use Prompto to handle user input by first defining an object of the Prompto type
//! to hold the handles to  your input/output streams,
//! and then you can call the methods on that object to get input from that stream.

use std::io::{self, BufRead, Write};

use thiserror::Error;

/// # SafeParsable
///
/// Defines a trait that is safe to parse from a string and has a default value
/// for `.unwrap_or_default()`
pub trait SafeParsable: Sized + Copy + Default + std::str::FromStr {}

impl<T> SafeParsable for T where T: Sized + Copy + Default + std::str::FromStr {}

/// # Prompto
///
/// Holds the input and output handles and redirects input and output to them.
///
/// # Example
/// To use this with stdio:
/// ```
/// use prompto::Prompto;
///
/// let stdio = std::io::stdin();
/// let input = stdio.lock();
/// let output = std::io::stdout();
///
/// let mut prompto = Prompto {
///     reader: input,
///     writer: output
/// };
/// ```
pub struct Prompto<R, W> {
    pub reader: R,
    pub writer: W,
}

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

impl<R, W> Prompto<R, W>
    where
        R: BufRead,
        W: Write,
{
    /// Get a newline-terminated string from stdin,
    /// returning `None` if `std::io::stdout.flush()` fails
    /// or if `std::io::stdin().read_line()` fails.
    ///
    /// # Arguments
    /// * `msg` – a message to display to the user.
    ///
    /// # Example
    /// ```
    /// use prompto::Prompto;
    ///
    /// let stdio = std::io::stdin();
    /// let input = stdio.lock();
    /// let output = std::io::stdout();
    ///
    /// let mut prompto = Prompto {
    ///     reader: input,
    ///     writer: output
    /// };
    ///
    /// let res = prompto.get_line("What's your name?");
    ///
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
    pub fn get_line(&mut self, msg: &str) -> Option<String> {
        match write!(&mut self.writer, "{}", msg) {
            Ok(()) => (),
            Err(_) => return None,
        }

        // Force output to stdout before reading from stdin
        match self.writer.flush() {
            Ok(()) => (),
            Err(_) => return None,
        }

        let mut buffer: String = String::new();

        match self.reader.read_line(&mut buffer) {
            Ok(_) => (),
            Err(_) => return None,
        }

        Some(buffer.trim_end().to_owned())
    }

    /// Same as `get_line()`, but returns a `Result<String, PromptError>`.
    /// Use this version if you need control over the errors.
    /// Returns `PromptError::StdinError` if:
    /// * `write!()` fails
    /// * `self.writer.flush()` fails
    /// * `self.reader.read_line()` fails
    ///
    /// # Arguments
    /// * `msg` – a message to display to the user.
    ///
    /// # Example
    /// ```
    /// use prompto::Prompto;
    ///
    /// let stdio = std::io::stdin();
    /// let input = stdio.lock();
    /// let output = std::io::stdout();
    ///
    /// let mut prompto = Prompto {
    ///     reader: input,
    ///     writer: output
    /// };
    ///
    /// let res = prompto.rget_line("What's your name?");
    ///
    /// match res {
    ///     Ok(s)  => println!("Nice to meet you, {}!", s),
    ///     Err(e) => eprintln!("I'm sorry! I got an error: {}", e)
    /// }
    /// ```
    pub fn rget_line(&mut self, msg: &str) -> Result<String, PromptError> {
        write!(&mut self.writer, "{}", msg)
            .map_err(|err| PromptError::StdinError(err))?;

        // Force output to stdout before reading from stdin
        self.writer.flush()
            .map_err(|err| PromptError::StdinError(err))?;

        let mut buffer: String = String::new();

        self.reader.read_line(&mut buffer)
            .map_err(|err| PromptError::StdinError(err))?;

        Ok(buffer.trim_end().to_owned())
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
    /// use prompto::Prompto;
    ///
    /// let stdio = std::io::stdin();
    /// let input = stdio.lock();
    /// let output = std::io::stdout();
    ///
    /// let mut prompto = Prompto {
    ///     reader: input,
    ///     writer: output
    /// };
    ///
    /// let res = prompto.read::<i32>("32").map(|x| x * 2).unwrap();
    ///
    /// println!("Value of res: {}.", res);
    /// ```
    ///
    /// The Haskell spec for this function is:
    /// ```hs
    /// readMaybe :: Read a => String -> Maybe a
    /// ```
    pub fn read<T>(&mut self, arg: &str) -> Option<T>
        where
            T: std::str::FromStr,
    {
        match T::from_str(arg) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }

    /// Same as `read()`, but returns a `Result<T, PromptError>`.
    /// Use this version if you need control over the errors.
    /// Returns `PromptError::ReadError` if:
    /// * `T::from_str(arg)` fails
    ///
    /// # Arguments
    /// * `arg` – string to attempt to convert.
    ///
    /// # Example
    /// ```
    /// use prompto::Prompto;
    ///
    /// let stdio = std::io::stdin();
    /// let input = stdio.lock();
    /// let output = std::io::stdout();
    ///
    /// let mut prompto = Prompto {
    ///     reader: input,
    ///     writer: output
    /// };
    ///
    /// let res = prompto.rread::<i32>("32").map(|x| x * 2).unwrap();
    ///
    /// println!("Value of res: {}.", res);
    /// ```
    pub fn rread<T>(&mut self, arg: &str) -> Result<T, PromptError> where T: std::str::FromStr {
        Ok(T::from_str(arg).map_err(|_| PromptError::ReadError)?)
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
    /// use prompto::Prompto;
    ///
    /// let stdio = std::io::stdin();
    /// let input = stdio.lock();
    /// let output = std::io::stdout();
    ///
    /// let mut prompto = Prompto {
    ///     reader: input,
    ///     writer: output
    /// };
    ///
    /// let res = prompto.input::<i32>("Please enter a number: ");
    ///
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
    pub fn input<T>(&mut self, msg: &str) -> Option<T>
        where
            T: SafeParsable,
    {
        self.get_line(msg).and_then(|s| self.read::<T>(&s))
    }

    /// Same as `input()`, but returns a `Result<T, PromptError>`.
    /// Use this version if you need control over the errors.
    /// Returns `PromptError` if:
    /// * `rget_line()` fails
    /// * `rread()` fails
    ///
    /// # Arguments
    /// `msg` – a message to display to the user.
    ///
    /// # Example
    /// ```
    /// use prompto::Prompto;
    ///
    /// let stdio = std::io::stdin();
    /// let input = stdio.lock();
    /// let output = std::io::stdout();
    ///
    /// let mut prompto = Prompto {
    ///     reader: input,
    ///     writer: output
    /// };
    ///
    /// let res = prompto.rinput::<i32>("Please enter a number: ");
    ///
    /// match res {
    ///     Ok(x)  => println!("Got {}.", x),
    ///     Err(_) => println!("Got invalid input!")
    /// }
    /// ```
    pub fn rinput<T>(&mut self, msg: &str) -> Result<T, PromptError> where T: SafeParsable {
        self.rget_line(msg).and_then(|s| self.rread(&s))
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
    /// use prompto::Prompto;
    ///
    /// let stdio = std::io::stdin();
    /// let input = stdio.lock();
    /// let output = std::io::stdout();
    ///
    /// let mut prompto = Prompto {
    ///     reader: input,
    ///     writer: output
    /// };
    ///
    /// let res: u32 = prompto.prompt("Please enter a number between 1 and 100: ", |x| 1 <= x && x <= 100);
    /// ```
    pub fn prompt<T, F>(&mut self, msg: &str, validator: F) -> T
        where
            T: SafeParsable,
            F: Fn(T) -> bool,
    {
        loop {
            let res: T = match self.input::<T>(msg) {
                Some(val) => val,
                None => {
                    match writeln!(&mut self.writer, "Invalid input! Please try again.") {
                        Ok(()) => (),
                        Err(_) => (),
                    }
                    continue;
                }
            };

            if validator(res) {
                break res;
            } else {
                match writeln!(&mut self.writer, "Invalid input! Please try again.") {
                    Ok(()) => (),
                    Err(_) => (),
                }
            }
        }
    }

    /// Same as `prompt()`, but internally uses the `Result` versions.
    /// This function is essentially the same as the `Option` version,
    /// but I have added it for completeness, and in case the emitted `Result`s
    /// are more useful for debugging.
    ///
    /// **Warning**: this function will panic if `writeln()` fails when:
    /// * `write!()` succeeds in `rget_line()` but `writeln!()` fails in this function.
    ///
    ///
    /// # Arguments
    /// * `msg` – a message to display to the user.
    /// * `validator` – a function which immutably borrows a single argument of type `T` and returns a `bool`.
    ///
    /// # Example
    /// ```
    /// use prompto::Prompto;
    ///
    /// let stdio = std::io::stdin();
    /// let input = stdio.lock();
    /// let output = std::io::stdout();
    ///
    /// let mut prompto = Prompto {
    ///     reader: input,
    ///     writer: output
    /// };
    ///
    /// let res: u32 = prompto.rprompt("Please enter a number between 1 and 100: ", |x| 1 <= x && x <= 100);
    /// ```
    ///
    /// # Panics
    /// If `write!()` succeeds in `rget_line()`, but `writeln!()` in this function somehow does not,
    /// this function panics with the message:
    /// `"writeln!() failed, even though write!() succeeded earlier"`
    ///
    /// I
    pub fn rprompt<T, F>(&mut self, msg: &str, validator: F) -> T
        where
            T: SafeParsable,
            F: Fn(T) -> bool,
    {
        loop {
            let res: T = match self.rinput::<T>(msg) {
                Ok(val) => val,
                Err(_) => {
                    match writeln!(&mut self.writer, "Invalid input! Please try again.") {
                        Ok(()) => (),
                        Err(_) => panic!("writeln!() failed, even though write!() succeeded earlier"),
                    }
                    continue;
                }
            };

            if validator(res) {
                break res;
            } else {
                match writeln!(&mut self.writer, "Invalid input! Please try again.") {
                    Ok(()) => (),
                    Err(_) => panic!("writeln!() failed, even though write!() succeeded earlier"),
                }
            }
        }
    }
}
