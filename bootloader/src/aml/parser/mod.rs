pub mod combinator;
pub mod error;
pub mod input;
pub mod primitive;

use self::{error::ParserError, input::Input};

pub type ParserResult<'input, O> = Result<(ParserState<'input>, O), ParserError>;

#[derive(Clone, Copy, Debug, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn with_len(&self, len: usize) -> Self {
        Self {
            end: self.start + len,
            ..*self
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ParserState<'input> {
    span: Span,
    input: Input<'input>,
}

impl<'input> ParserState<'input> {
    pub fn new(input: &'input [u8]) -> Self {
        Self {
            span: Span::default(),
            input: Input::new(input),
        }
    }
}
