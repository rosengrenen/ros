use super::ops::Arg0Op;
use super::ops::Arg1Op;
use super::ops::Arg2Op;
use super::ops::Arg3Op;
use super::ops::Arg4Op;
use super::ops::Arg5Op;
use super::ops::Arg6Op;
use super::ops::DebugOp;
use super::ops::Local0Op;
use super::ops::Local1Op;
use super::ops::Local2Op;
use super::ops::Local3Op;
use super::ops::Local4Op;
use super::ops::Local5Op;
use super::ops::Local6Op;
use super::ops::Local7Op;
use super::parser::Input;
use super::parser::ParseResult;
use super::parser::ParserError;

#[derive(Debug)]
pub enum ArgObj {
    Arg0(Arg0Op),
    Arg1(Arg1Op),
    Arg2(Arg2Op),
    Arg3(Arg3Op),
    Arg4(Arg4Op),
    Arg5(Arg5Op),
    Arg6(Arg6Op),
}

impl ArgObj {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        match Arg0Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Arg0(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Arg1Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Arg1(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Arg2Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Arg2(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Arg3Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Arg3(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Arg4Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Arg4(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Arg5Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Arg5(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Arg6Op::parse(input)?;
        Ok((Self::Arg6(value), input))
    }
}

#[derive(Debug)]
pub enum LocalObj {
    Local0(Local0Op),
    Local1(Local1Op),
    Local2(Local2Op),
    Local3(Local3Op),
    Local4(Local4Op),
    Local5(Local5Op),
    Local6(Local6Op),
    Local7(Local7Op),
}

impl LocalObj {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        match Local0Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Local0(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Local1Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Local1(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Local2Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Local2(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Local3Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Local3(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Local4Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Local4(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Local5Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Local5(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Local6Op::parse(input) {
            Ok((value, input)) => return Ok((Self::Local6(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Local7Op::parse(input)?;
        Ok((Self::Local7(value), input))
    }
}

#[derive(Debug)]
pub struct DebugObj;

impl DebugObj {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = DebugOp::parse(input)?;
        Ok((Self, input))
    }
}
