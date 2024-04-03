use crate::aml::ops::TimerOp;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;

#[derive(Debug)]
pub struct Timer;

impl Timer {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = TimerOp::parse(input)?;
        Ok((Self, input))
    }
}
