#[derive(Copy, Clone, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    fn split_at_index(&self, index: usize) -> (Self, Self) {
        (
            Self {
                start: self.start,
                end: self.start + index,
            },
            Self {
                start: self.start + index,
                end: self.end,
            },
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Input<'a> {
    pub bytes: &'a [u8],
    pub span: Span,
}

impl<'a> Input<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let len = bytes.len();
        Self::with_span(bytes, Span { start: 0, end: len })
    }

    fn with_span(bytes: &'a [u8], span: Span) -> Self {
        Self { bytes, span }
    }
}

impl<'a> Input<'a> {
    pub fn split_at_index(&self, index: usize) -> Option<(Self, Self)> {
        if index > self.bytes.len() {
            return None;
        }

        let (left, right) = self.bytes.split_at(index);
        let (left_span, right_span) = self.span.split_at_index(index);
        Some((
            Self::with_span(left, left_span),
            Self::with_span(right, right_span),
        ))
    }
}

pub type ParseResult<'a, O> = Result<(O, Input<'a>), ParserError>;

#[derive(Debug)]
pub enum ParserError {
    Error,
    Failure,
}

pub fn fail<'a, O>(result: ParseResult<'a, O>) -> ParseResult<'a, O> {
    result.map_err(|_| ParserError::Failure)
}

pub fn take<'a>(input: Input<'a>, index: usize) -> ParseResult<'a, &'a [u8]> {
    let (left, right) = input.split_at_index(index).ok_or(ParserError::Failure)?;
    Ok((left.bytes, right))
}

pub fn take_one<'a>(input: Input<'a>) -> ParseResult<'a, u8> {
    let (bytes, input) = take(input, 1)?;
    Ok((bytes[0], input))
}

pub fn item<'a>(input: Input<'a>, byte: u8) -> ParseResult<'a, ()> {
    let (b, input) = take_one(input)?;
    if b != byte {
        return Err(ParserError::Error);
    }

    Ok(((), input))
}

pub fn satisfy<'a, F: FnOnce(u8) -> bool>(input: Input<'a>, pred: F) -> ParseResult<'a, u8> {
    let (b, input) = take_one(input)?;
    if !pred(b) {
        return Err(ParserError::Error);
    }

    Ok((b, input))
}

pub fn fail_if_not_empty<'a>(input: Input<'a>) -> ParseResult<'a, ()> {
    if !input.bytes.is_empty() {
        return Err(ParserError::Failure);
    }

    Ok(((), input))
}

// macro_rules! parse_enum_variant {
//     ($parser:ty, $variant:ident, $input:ident, $alloc:ident) => {
//         if let Ok((output, input)) = <$parser>::parse($input, $alloc) {
//             return Ok((Self::$variant(output), input));
//         }
//     };
// }

// macro_rules! parse_enum_variant_last {
//     ($parser:ty, $variant:ident, $input:ident, $alloc:ident) => {{
//         let (output, input) = <$parser>::parse($input, $alloc)?;
//         Ok((Self::$variant(output), input))
//     }};
// }
