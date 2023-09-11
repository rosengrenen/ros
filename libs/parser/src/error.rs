pub type ParseResult<I, O, E> = Result<(I, O), ParserError<E>>;

#[derive(Debug)]
pub enum ParserError<I> {
    Error(I),
    Failure(I),
}

pub trait ParseError<I> {
    fn from_error_kind(input: I, kind: ParseErrorKind) -> Self;

    fn append(self, other: Self) -> Self;
}

pub trait FromExternalError<I, E> {
    fn from_external_error(input: I, kind: ParseErrorKind, error: E) -> Self;
}

impl<I, E> FromExternalError<I, E> for () {
    fn from_external_error(_input: I, _kind: ParseErrorKind, _e: E) -> Self {}
}

#[derive(Clone, Copy, Debug)]
pub enum ParseErrorKind {
    None,
}
