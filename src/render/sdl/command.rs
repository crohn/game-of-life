use std::str::{FromStr, SplitWhitespace};

#[derive(Debug)]
pub enum Command {
    // Clears the board setting all cells to Dead.
    BoardClear,
    // Move cursor to coordinates. If no cursor is active, creates one.
    Cursor(i32, i32),
    // Gracefully terminate the application.
    Quit,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidArgument,
    InvalidCommand,
    MissingArgument,
}

// Translates provided input into a supported command or returns an error if the
// command is not recognized.
// All command inputs start with ':' and are whitespace separated.
pub fn parse(input: &str) -> Result<Command, ParseError> {
    let mut parts = input.trim_start_matches(':').split_whitespace();
    let cmd = parts.next().unwrap_or_default();
    match cmd {
        "b" | "board" => parse_board_command(parts),
        "cur" | "cursor" => {
            let x = parse_numeric(parts.next())?;
            let y = parse_numeric(parts.next())?;
            Ok(Command::Cursor(x, y))
        }
        "q" | "quit" => Ok(Command::Quit),
        _ => Err(ParseError::InvalidCommand),
    }
}

fn parse_board_command(mut parts: SplitWhitespace) -> Result<Command, ParseError> {
    match parts.next().unwrap_or_default() {
        "clear" => Ok(Command::BoardClear),
        _ => Err(ParseError::InvalidCommand),
    }
}

// fn parse_cursor_command(mut parts: SplitWhitespace) -> Result<Command, ParseError> {
//     match parts.next().unwrap_or_default() {
//         "clear" => Ok(Command::BoardClear),
//         _ => Err(ParseError::InvalidCommand),
//     }
// }

fn parse_numeric<T>(value: Option<&str>) -> Result<T, ParseError>
where
    T: Numeric + FromStr,
{
    value
        .unwrap_or_default()
        .parse::<T>()
        .map_err(|_| ParseError::InvalidArgument)
}

trait Numeric {}

impl Numeric for i8 {}
impl Numeric for i16 {}
impl Numeric for i32 {}
impl Numeric for i64 {}
impl Numeric for i128 {}
impl Numeric for isize {}
impl Numeric for u8 {}
impl Numeric for u16 {}
impl Numeric for u32 {}
impl Numeric for u64 {}
impl Numeric for u128 {}
impl Numeric for usize {}
