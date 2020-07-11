//! # Prompto
//!
//! Simple, functional, monadic command-line prompts.
//! Heavily inspired by `readMaybe` from Haskell.

pub mod result {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum PromptError {
        #[error("Failure reading line from stdin")]
        Stdin(#[from] std::io::Error),
        #[error("Failure converting string to data type")]
        Read,
    }

    // promptLine :: String -> IO String
    pub fn prompt_line(msg: &str) -> Result<String, PromptError> {
        use std::io::Write;

        print!("{}", msg);
        
        std::io::stdout().flush().map_err(|source| PromptError::Stdin(source))?;

        let mut buffer: String = String::new();
        std::io::stdin().read_line(&mut buffer)
            .map_err(|source| PromptError::Stdin(source))?;

        Ok(buffer.trim_end().to_owned())
    }

    pub fn read<T>(line: &str) -> Result<T, PromptError> where T: std::str::FromStr {
        line.parse::<T>().map_err(|_| PromptError::Read)
    }

    pub fn input<T, F>(prompt: &str, validator: F) -> T
        where
            T: Copy + std::str::FromStr + std::default::Default,
            F: Fn(T) -> bool
    {
        loop {
            let res = prompt_line(prompt)
                .and_then(|s| read::<T>(&s))
                .unwrap_or_default();

            if validator(res) {
                break res;
            } else {
                println!("Invalid input! Please try again.");
            }
        }
    }
}

pub mod maybe {
    pub fn prompt_maybe(msg: &str) -> Option<String> {
        use std::io::Write;

        print!("{}", msg);

        // Force output to stdout before reading from stdin
        // Remember - ? returns early in case this throws an error!
        match std::io::stdout().flush() {
            Ok(()) => (),
            Err(_) => return None
        }

        let mut buffer: String = String::new();
        match std::io::stdin().read_line(&mut buffer) {
            Ok(_) => (),
            Err(_) => return None
        }

        Some(buffer.trim_end().to_owned())
    }

    pub fn read_maybe<T>(arg: &str) -> Option<T> where T: std::str::FromStr {
        match arg.parse::<T>() {
            Ok(res) => Some(res),
            Err(_) => None
        }
    }

    pub fn input_maybe<T, F>(prompt: &str, validator: F) -> T
        where
            T: Copy + std::str::FromStr + std::default::Default,
            F: Fn(T) -> bool
    {
        loop {
            let res = prompt_maybe(prompt)
                .and_then(|s| read_maybe::<T>(&s))
                .unwrap_or_default();

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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
