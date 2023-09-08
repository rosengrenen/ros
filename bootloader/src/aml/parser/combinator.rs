use super::{error::ParserError, ParserResult, ParserState};

pub fn map<'input, O1, O2>(
    parser: impl Fn(ParserState<'input>) -> ParserResult<O1>,
    f: impl Fn(O1) -> O2,
) -> impl Fn(ParserState<'input>) -> ParserResult<O2> {
    move |state| {
        let (state, value) = parser(state)?;
        let value = f(value);
        Ok((state, value))
    }
}

pub fn map_res<'input, O1, O2>(
    parser: impl Fn(ParserState<'input>) -> ParserResult<O1>,
    f: impl Fn(O1) -> Result<O2, ParserError>,
) -> impl Fn(ParserState<'input>) -> ParserResult<O2> {
    move |state| {
        let (state, value) = parser(state)?;
        let value = f(value)?;
        Ok((state, value))
    }
}
