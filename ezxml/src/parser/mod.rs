extern crate env_logger;

use std::io::prelude::*;

use snafu::{Backtrace, GenerateBacktrace, ResultExt};

use crate::error::Error::Bug;
use crate::error::{self, Result};
use crate::parser::DocState::BeforeFirstTag;
use crate::parser::TagStatus::{InsideTag, OutsideTag, TagOpen};
use crate::parser::UserDataStatus::Outside;
use crate::structure;
use crate::structure::{ElementContent, ParserMetadata};
use std::str::Chars;

// mod error;

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
pub enum DocState {
    BeforeFirstTag,
    XmlDeclaration,
    Doctype,
    RootElement,
}

impl Default for DocState {
    fn default() -> Self {
        DocState::BeforeFirstTag
    }
}

// Comparison traits: Eq, PartialEq, Ord, PartialOrd.
// Clone, to create T from &T via a copy.
// Copy, to give a type 'copy semantics' instead of 'move semantics'.
// Hash, to compute a hash from &T.
// Default, to create an empty instance of a data type.
// Debug, to format a value using the {:?} formatter.
// #[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]

const _BUFF_SIZE: usize = 1024;

pub fn _parse<R: BufRead>(r: &mut R) -> error::Result<structure::Document> {
    let mut s = String::new();
    let _ = r.read_to_string(&mut s).context(error::IoRead {
        parse_location: error::ParseLocation { line: 0, column: 0 },
    })?;
    parse_str(&s)
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash, Default)]
pub struct Position {
    pub line: u64,
    pub column: u64,
    pub absolute: u64,
}

impl Position {
    fn increment(&mut self, current_char: &char) {
        self.absolute += 1;
        if current_char == &'\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash, Default)]
struct ParserState {
    position: Position,
    doc_state: DocState,
    current_char: char,
    tag_status: TagStatus,
    user_data_status: UserDataStatus,
}

pub fn parse_str(s: &str) -> Result<structure::Document> {
    let mut state = ParserState {
        position: Default::default(),
        doc_state: DocState::BeforeFirstTag,
        current_char: '\0',
        tag_status: OutsideTag,
        user_data_status: Outside,
    };

    let mut iter = s.chars();
    while advance_parser(&mut iter, &mut state) {
        process_char(&mut iter, &mut state)?;
        trace!("{:?}", state);
    }

    Ok(structure::Document {
        // version: None,
        // encoding: None,
        root: structure::Element {
            parser_metadata: ParserMetadata {},
            namespace: None,
            name: "x".to_string(),
            content: ElementContent::Empty,
        },
    })
}

// <tag></tag>
#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
enum TagStatus {
    TagOpen,
    InsideTag,
    TagClose,
    OutsideTag,
}

impl Default for TagStatus {
    fn default() -> Self {
        return TagStatus::OutsideTag;
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialOrd, PartialEq, Hash)]
enum UserDataStatus {
    Inside,
    Outside,
}

impl Default for UserDataStatus {
    fn default() -> Self {
        return UserDataStatus::Outside;
    }
}

fn process_char(iter: &mut Chars, state: &mut ParserState) -> Result<()> {
    match state.tag_status {
        TagStatus::TagOpen => state.tag_status = TagStatus::InsideTag,
        TagStatus::InsideTag => {
            if state.user_data_status == UserDataStatus::Outside && state.current_char == '>' {
                state.tag_status = TagStatus::TagClose
            }
        }
        TagStatus::TagClose => {
            if state.user_data_status == UserDataStatus::Outside && state.current_char == '<' {
                state.tag_status = TagStatus::TagOpen;
            } else {
                state.tag_status = TagStatus::OutsideTag;
            }
        }
        OutsideTag => {
            if state.current_char == '<' {
                state.tag_status = TagStatus::TagOpen
            }
        }
    }
    Ok(())
}

fn advance_parser(iter: &mut Chars<'_>, state: &mut ParserState) -> bool {
    let option_char = iter.next();
    match option_char {
        Some(c) => {
            state.current_char = c;
            state.position.increment(&state.current_char);
            return true;
        }
        None => return false,
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// TESTS
////////////////////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    const XML1: &str = r##"
<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE something PUBLIC "-//Some//Path//EN" "http://www.example.org/dtds/partwise.dtd">
<cats>
  <cat id="b1">
    <name>
        Bones
    </name>
  <birthdate>2008-06-01</birthdate>
  </cat>
  <cat id="b2">
    <name>Bishop</name>
    <birthdate>2012-01-01</birthdate>
  </cat>
</cats>
    "##;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    // Check if a url with a trailing slash and one without trailing slash can both be parsed
    #[test]
    fn parse_a_doo_dah() {
        init_logger();
        let the_thing = XML1;
        let _ = parse_str(the_thing).unwrap();
    }
}