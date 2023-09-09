use crate::{error::ParserError, ParserResult};

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

    pub(super) fn with_span(bytes: &'input [u8], span: Span) -> Self {
        Self { bytes, span }
    }

    pub(super) fn as_slice(&self) -> &'input [u8] {
        self.bytes
    }

    pub(super) fn split_at_index(&self, index: usize) -> (Self, Self) {
        let (left, right) = self.bytes.split_at(index);
        let (left_span, right_span) = self.span.split_at(index);
        (
            Self::with_span(left, left_span),
            Self::with_span(right, right_span),
        )
    }

    pub(super) fn split_at_position<P>(&self, pred: P) -> ParserResult<'input, &'input [u8]>
    where
        P: Fn(u8) -> bool,
    {
        match self.bytes.iter().position(|&b| pred(b)) {
            Some(n) => {
                let (output, input) = self.split_at_index(n);
                Ok((input, output.bytes, output.span))
            }
            None => Err(ParserError::new(self.clone())),
        }
    }
}
