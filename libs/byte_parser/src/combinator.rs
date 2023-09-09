use crate::{error::ParseError, ParserFn};

use super::error::ParserError;

pub fn map<'input, Out1, Out2>(
    parser: impl ParserFn<'input, Out1>,
    f: impl Fn(Out1) -> Out2,
) -> impl ParserFn<'input, Out2> {
    move |input| {
        let (input, value, span) = parser(input)?;
        let value = f(value);
        Ok((input, value, span))
    }
}

pub fn map_res<'input, Out1, Out2, E>(
    parser: impl ParserFn<'input, Out1>,
    f: impl Fn(Out1) -> Result<Out2, E>,
) -> impl ParserFn<'input, Out2> {
    move |input| {
        let (input, value, span) = parser(input)?;
        let value = {
            let input = input.clone();
            f(value).map_err(move |_| ParseError::Error(ParserError::new(input)))?
        };
        Ok((input, value, span))
    }
}

pub fn opt<'input, Out>(parser: impl ParserFn<'input, Out>) -> impl ParserFn<'input, Option<Out>> {
    move |input| match parser(input) {
        Ok((input, output, span)) => Ok((input, Some(output), span)),
        Err(ParseError::Error(error)) => {
            // TOOD: fix span
            let span = error.input.span;
            Ok((error.input, None, span))
        }
        Err(ParseError::Failure(error)) => Err(ParseError::Failure(error)),
    }
}

pub fn cut<'input, Out>(parser: impl ParserFn<'input, Out>) -> impl ParserFn<'input, Out> {
    move |input| match parser(input) {
        Err(ParseError::Error(error)) => Err(ParseError::Failure(error)),
        result => result,
    }
}
