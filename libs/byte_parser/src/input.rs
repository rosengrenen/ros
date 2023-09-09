use crate::{
    error::{ParseError, ParserError},
    ParserResult,
};

use super::Span;

#[derive(Clone, Debug)]
pub struct Input<'input> {
    bytes: &'input [u8],
    pub(crate) span: Span,
}

impl<'input> Input<'input> {
    pub fn new(bytes: &'input [u8]) -> Self {
        Self::with_span(bytes, Span::new(0, bytes.len()))
    }

    pub(crate) fn with_span(bytes: &'input [u8], span: Span) -> Self {
        Self { bytes, span }
    }

    pub(crate) fn as_slice(&self) -> &'input [u8] {
        self.bytes
    }

    pub fn split_at_index(&self, index: usize) -> (Self, Self) {
        let (left, right) = self.bytes.split_at(index);
        let (left_span, right_span) = self.span.split_at(index);
        (
            Self::with_span(left, left_span),
            Self::with_span(right, right_span),
        )
    }

    fn position<P>(&self, pred: P) -> Option<usize>
    where
        P: Fn(u8) -> bool,
    {
        self.bytes.iter().position(|&b| pred(b))
    }

    pub(crate) fn split_at_position0<P>(&self, pred: P) -> ParserResult<'input, &'input [u8]>
    where
        P: Fn(u8) -> bool,
    {
        match self.position(pred) {
            Some(n) => {
                let (output, input) = self.split_at_index(n);
                Ok((input, output.bytes, output.span))
            }
            None => Err(ParseError::Error(ParserError::new(self.clone()))),
        }
    }

    pub(crate) fn split_at_position1<P>(&self, pred: P) -> ParserResult<'input, &'input [u8]>
    where
        P: Fn(u8) -> bool,
    {
        match self.position(pred) {
            Some(n) if n > 0 => {
                let (output, input) = self.split_at_index(n);
                Ok((input, output.bytes, output.span))
            }
            _ => Err(ParseError::Error(ParserError::new(self.clone()))),
        }
    }
}
