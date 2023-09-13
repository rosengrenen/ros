use core::alloc::Allocator;

pub type ParseResult<'alloc, I, O, E> = Result<(I, O), ParserError<E>>;

#[derive(Debug)]
pub enum ParserError<E> {
    Error(E),
    Failure(E),
}

impl<E> ParserError<E> {
    pub(crate) fn map<F>(self, f: F) -> Self
    where
        F: FnOnce(E) -> E,
    {
        match self {
            ParserError::Error(error) => ParserError::Error(f(error)),
            ParserError::Failure(error) => ParserError::Failure(error),
        }
    }

    pub(crate) fn fail(self) -> Self {
        match self {
            ParserError::Error(error) => ParserError::Failure(error),
            ParserError::Failure(error) => ParserError::Failure(error),
        }
    }
}

pub trait ParseError<'alloc, I, A: Allocator> {
    fn from_error_kind(input: I, kind: ParseErrorKind, alloc: &'alloc A) -> Self;

    fn append(self, input: I, kind: ParseErrorKind) -> Self;
}

pub trait FromExternalError<'alloc, I, E, A: Allocator> {
    fn from_external_error(input: I, kind: ParseErrorKind, error: E, alloc: &'alloc A) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub enum ParseErrorKind {
    None,
}
