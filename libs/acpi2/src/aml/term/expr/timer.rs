use crate::aml::{
    ops::TimerOp,
    parser::{Input, ParseResult},
};

#[derive(Debug)]
pub struct Timer;

impl Timer {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = TimerOp::parse(input)?;
        Ok((Self, input))
    }
}
