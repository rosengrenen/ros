use super::{error::ParserError, parser::Parser};

pub fn map<'input, O1, O2>(
    parser: impl Parser<'input, O1>,
    f: impl Fn(O1) -> O2,
) -> impl Parser<'input, O2> {
    move |input| {
        let (input, value, span) = parser.parse(input)?;
        let value = f(value);
        Ok((input, value, span))
    }
}

pub fn map_res<'input, Out1, Out2, E>(
    parser: impl Parser<'input, Out1>,
    f: impl Fn(Out1) -> Result<Out2, E>,
) -> impl Parser<'input, Out2> {
    move |input| {
        let (input, value, span) = parser.parse(input)?;
        let value = {
            let input = input.clone();
            f(value).map_err(move |_| ParserError::new(input))?
        };
        Ok((input, value, span))
    }
}
