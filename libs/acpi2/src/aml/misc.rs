use super::{
    ops::{
        Arg0Op, Arg1Op, Arg2Op, Arg3Op, Arg4Op, Arg5Op, Arg6Op, DebugOp, Local0Op, Local1Op,
        Local2Op, Local3Op, Local4Op, Local5Op, Local6Op, Local7Op,
    },
    parser::{Input, ParseResult, ParserError},
};

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

pub struct DebugObj;

impl DebugObj {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = DebugOp::parse(input)?;
        Ok((Self, input))
    }
}
