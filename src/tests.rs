#[cfg(test)]

/// Note: I am deliberately *not* testing the functions
/// in the result module because they are mostly identical
/// to the functions in the maybe module. The only difference
/// is that I would be checking for certain errors rather than None.
use std::str::FromStr;
use crate::Vento;

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

/// This test encompasses my sanity checks:
/// this test ensures that my `read` function is behaving correctly;
/// that is, it is doing the same thing as `parse` and `from_str`.
/// This test covers only primitive types because I feel I can trust that
/// there won't be any weird gotchas with the conversions.
#[test]
fn sanity_checks() {
    let input = b"";
    let mut output = Vec::new();

    let mut vento = Vento {
        reader: &input[..],
        writer: &mut output,
    };

    // parse and from_str should always be equal for the same arguments.
    assert_eq!("32".parse::<i32>().unwrap(), i32::from_str("32").unwrap());

    // A string with a valid integer should always succeed.
    assert!(vento.read::<i32>("32").is_some());

    // A string with any number of non-numeric characters should never succeed,
    // even if any part of the string *could* be valid.
    assert!(vento
        .read::<i32>("56 fdfs θ gx二éfs sdf34ごν53 df3dfsd2")
        .is_none());

    // Implicit widening conversions are okay...
    assert!(vento.read::<f32>("32").is_some());

    // But truncating conversions are not!
    assert!(vento.read::<i32>("32.32").is_none());
}

/// This test is the sanity check for composite types.
/// In this test, I am checking to see that `from_str` and my `read` function
/// do the same thing on a user-defined type that implements `std::str::FromStr`.
#[test]
fn composite_type_checks() {
    let input = b"";
    let mut output = Vec::new();

    let mut vento = Vento {
        reader: &input[..],
        writer: &mut output,
    };

    // Read should behave the same way as calling parse or calling from_str directly on the type.
    let call_through_trait = RGB::from_str(r"#fa7268").unwrap()
        == RGB {
        r: 250,
        g: 114,
        b: 104,
    };
    let call_through_maybe = vento.read::<RGB>(r"#fa7268").unwrap()
        == RGB {
        r: 250,
        g: 114,
        b: 104,
    };
    assert_eq!(call_through_trait, call_through_maybe);

    // Caveat: read cannot catch all possible errors in this case;
    // for instance, if you have a multi-byte character in this string,
    // the compiler itself will error out!
    assert!(vento.read::<RGB>(r"gkhgkjyfa7jhkhjk268").is_none());
}

/// In this test, I am checking that I can `fmap` through the values that `read` gives,
/// regardless of whether the Option is a `Some` or a `None`.
/// In this test, I use `unwrap_or_default()` to prevent the test from erroring out
/// in case a function gets `None`.
#[test]
fn chaining_checks() {
    let input = b"";
    let mut output = Vec::new();

    let mut vento = Vento {
        reader: &input[..],
        writer: &mut output,
    };

    let res = vento.read::<i32>("32").map(|x| x * 2).unwrap_or_default();

    assert_eq!(res, 64);

    let res = vento
        .read::<f32>("3.14")
        .map(|x| x * 2f32)
        .unwrap_or_default();

    assert_eq!(res, 6.28);

    let res = vento
        .read::<RGB>(r"#fa7268")
        .map(|rgb| rgb.r - 100)
        .unwrap_or_default();

    assert_eq!(res, 150);

    // Test a bad read
    let res = vento
        .read::<i32>("3fdgdf2")
        .map(|x| x * 2)
        .unwrap_or_default();

    assert_eq!(res, 0);
}

/// In this test, I am checking that my `input` function behaves well with good input.
/// I used a trick I found on SO to mock stdin/stdout here.
#[test]
fn stdio_good_input_check() {
    let input = b"32";
    let mut output = Vec::new();

    let mut vento = Vento {
        reader: &input[..],
        writer: &mut output,
    };

    let res = vento
        .input::<i32>("What's your favourite number? ")
        .unwrap();

    let output = String::from_utf8(output).unwrap();

    assert_eq!("What's your favourite number? ", output);
    assert_eq!(32, res);
}

/// Ditto, but for bad input.
#[test]
fn stdio_bad_input_check() {
    let input = b"gdfg32";
    let mut output = Vec::new();

    let mut vento = Vento {
        reader: &input[..],
        writer: &mut output,
    };

    let res = vento.input::<i32>("What's your favourite number? ");

    let output = String::from_utf8(output).unwrap();

    assert_eq!("What's your favourite number? ", output);
    assert!(res.is_none());
}

/// In this test, I am checking that my `prompt` function behaves well with good input.
/// Notice the checks at the bottom: the catch with this test and the next one is that
/// I have to give good input at some point or the functions will never end!
#[test]
fn stdio_good_prompt_check() {
    let input = b"32";
    let mut output = Vec::new();

    let mut vento = Vento {
        reader: &input[..],
        writer: &mut output,
    };

    let res: i32 = vento.prompt("Please enter a number between 1 and 50: ", |x| {
        1 <= x && x <= 50
    });

    let output = String::from_utf8(output).unwrap();

    assert_eq!("Please enter a number between 1 and 50: ", output);
    assert_eq!(32, res);
}

/// In this test, I am checking that my `prompt` function behaves well with bad input.
/// Notice the input and the checks at the bottom; like I said before, I have to give
/// this function good input at some point or else it will never stop.
#[test]
fn stdio_bad_prompt_check() {
    let input = b"3ghhj2\n25";
    let mut output = Vec::new();

    let mut vento = Vento {
        reader: &input[..],
        writer: &mut output,
    };

    let res: i32 = vento.prompt("Please enter a number between 1 and 50: ", |x| {
        1 <= x && x <= 50
    });

    let output = String::from_utf8(output).unwrap();

    assert_eq!("Please enter a number between 1 and 50: Invalid input! Please try again.\nPlease enter a number between 1 and 50: ", output);
    assert_eq!(25, res);
}
