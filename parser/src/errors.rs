use crate::string_utils::StringPositions;
use crate::Cursor;

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
    StaticStr(&'static str),
    Str(String),
    Eof,
}

impl ErrorKind {
    pub fn description(&self) -> &str {
        use ErrorKind::*;

        match self {
            StaticStr(s) => s,
            Str(s) => s.as_str(),
            Eof => "end of file",
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseError<'a> {
    errors: Vec<(Cursor<'a>, ErrorKind)>,
}

impl<'a> ParseError<'a> {
    pub fn new(input: Cursor<'a>, kind: ErrorKind) -> Self {
        Self {
            errors: vec![(input, kind)],
        }
    }

    pub fn push(mut self, input: Cursor<'a>, kind: ErrorKind) -> Self {
        self.errors.push((input, kind));
        self
    }

    /// Format an error into a debug trace message.
    pub fn format_debug(&self, input: &str, show_err_no: bool) -> String {
        use std::iter::repeat;

        let mut string_positions = StringPositions::new(input);
        let lines: Vec<_> = input.lines().map(String::from).collect();

        let mut result = String::new();

        for (err_no, (parser_input, err_kind)) in self.errors.iter().rev().enumerate() {
            let first_token = parser_input.iter().next();

            if show_err_no {
                result += &format!("{}: ", err_no);
            }

            let offset = match first_token {
                Some(tok) => tok.span.start,
                None => input.len(),
            };
            let pos = match string_positions.get_pos(offset) {
                Some(pos) => pos,
                None => string_positions.get_last().unwrap(),
            };

            result += &format!(
                "at line {} col {}, {}:\n",
                pos.line,
                pos.col,
                err_kind.description()
            );

            result += &lines[pos.line - 1];
            result += "\n";
            if pos.col > 0 {
                result += &repeat(' ').take(pos.col).collect::<String>();
            }
            result += "^\n\n";
        }

        result
    }

    /// Format an error into a user-facing error message.
    ///
    /// Uses the innermost error to build a user-facing error message and
    /// position.
    pub fn format_user(&self, input: &str) -> String {
        let deepest_error = self.errors.first().unwrap();

        let new_err = ParseError {
            errors: vec![deepest_error.clone()],
        };

        new_err.format_debug(input, false)
    }
}
