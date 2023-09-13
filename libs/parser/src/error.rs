use core::alloc::Allocator;

pub type ParseResult<'alloc, I, O, E> = Result<(I, O), ParserError<E>>;

#[derive(Debug)]
pub enum ParserError<I> {
    Error(I),
    Failure(I),
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
