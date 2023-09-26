use core::alloc::Allocator;

pub type ParseResult<I, O, E> = Result<(I, O), ParserError<E>>;

#[derive(Debug)]
pub enum ParserError<E> {
    Error(E),
    Failure(E),
}

impl<E> ParserError<E> {
    pub(crate) fn append<I, A>(self, input: I, kind: ParseErrorKind) -> Self
    where
        E: ParseError<I, A>,
        A: Allocator,
    {
        match self {
            ParserError::Error(error) => ParserError::Error(error.replace(input, kind)),
            ParserError::Failure(error) => ParserError::Failure(error.append(input, kind)),
        }
    }
}

pub trait ParseError<I, A: Allocator>: Clone {
    fn from_error_kind(input: I, kind: ParseErrorKind, alloc: A) -> Self;

    fn append(self, input: I, kind: ParseErrorKind) -> Self;

    fn replace(self, input: I, kind: ParseErrorKind) -> Self;

    fn add_context(self, input: I, context: &'static str) -> Self;
}

pub trait FromExternalError<I, E, A: Allocator> {
    fn from_external_error(input: I, kind: ParseErrorKind, error: E, alloc: A) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub enum ParseErrorKind {
    Unknown,
    Alt,
    AndThen,
    MapBoxed,
    Cut,
    MapRes,
    MapRes1,
    Map,
    Opt,
    Or,
    Tuple,
    Fold,
    Fold1,
    FoldN,
    FoldMN,
    Many,
    Many1,
    ManyN,
    ManyMN,
    Fail,
    TakeOne,
    Item,
    Satisfy,
    Take,
    TakeConst,
    TakeWhile,
    TakeWhile1,
    TakeWhileN,
    TakeWhileMN,
    Preceded,
}
