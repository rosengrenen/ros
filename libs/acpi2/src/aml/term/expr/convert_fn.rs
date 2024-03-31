use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::{
        FromBcdOp, ToBcdOp, ToBufferOp, ToDecimalStringOp, ToHexStringOp, ToIntegerOp, ToStringOp,
    },
    parser::{fail, Input, ParseResult, ParserError},
    term::TermArg,
};

pub enum ConvertFn<A: Allocator> {
    FromBcd(FromBcd<A>),
    ToBcd(ToBcd<A>),
    ToBuffer(ToBuffer<A>),
    ToDecimalString(ToDecimalString<A>),
    ToHexString(ToHexString<A>),
    ToInteger(ToInteger<A>),
    ToString(ToString<A>),
}

impl<A: Allocator + Clone> ConvertFn<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match FromBcd::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::FromBcd(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ToBcd::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ToBcd(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ToBuffer::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ToBuffer(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ToDecimalString::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ToDecimalString(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ToHexString::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ToHexString(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ToInteger::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ToInteger(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = ToString::parse(input, context, alloc)?;
        Ok((Self::ToString(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for ConvertFn<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::FromBcd(arg0) => f.debug_tuple("FromBcd").field(arg0).finish(),
            Self::ToBcd(arg0) => f.debug_tuple("ToBcd").field(arg0).finish(),
            Self::ToBuffer(arg0) => f.debug_tuple("ToBuffer").field(arg0).finish(),
            Self::ToDecimalString(arg0) => f.debug_tuple("ToDecimalString").field(arg0).finish(),
            Self::ToHexString(arg0) => f.debug_tuple("ToHexString").field(arg0).finish(),
            Self::ToInteger(arg0) => f.debug_tuple("ToInteger").field(arg0).finish(),
            Self::ToString(arg0) => f.debug_tuple("ToString").field(arg0).finish(),
        }
    }
}

pub struct FromBcd<A: Allocator> {
    pub input: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> FromBcd<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = FromBcdOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (input_arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        Ok((
            Self {
                input: input_arg,
                target,
            },
            input,
        ))
    }
}

impl<A: Allocator> core::fmt::Debug for FromBcd<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FromBcd")
            .field("input", &self.input)
            .field("target", &self.target)
            .finish()
    }
}

pub struct ToBcd<A: Allocator> {
    pub input: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ToBcd<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ToBcdOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (input_arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        Ok((
            Self {
                input: input_arg,
                target,
            },
            input,
        ))
    }
}

impl<A: Allocator> core::fmt::Debug for ToBcd<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ToBcd")
            .field("input", &self.input)
            .field("target", &self.target)
            .finish()
    }
}

pub struct ToBuffer<A: Allocator> {
    pub input: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ToBuffer<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ToBufferOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (input_arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        Ok((
            Self {
                input: input_arg,
                target,
            },
            input,
        ))
    }
}

impl<A: Allocator> core::fmt::Debug for ToBuffer<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ToBuffer")
            .field("input", &self.input)
            .field("target", &self.target)
            .finish()
    }
}

pub struct ToDecimalString<A: Allocator> {
    pub input: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ToDecimalString<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ToDecimalStringOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (input_arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        Ok((
            Self {
                input: input_arg,
                target,
            },
            input,
        ))
    }
}

impl<A: Allocator> core::fmt::Debug for ToDecimalString<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ToDecimalString")
            .field("input", &self.input)
            .field("target", &self.target)
            .finish()
    }
}

pub struct ToHexString<A: Allocator> {
    pub input: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ToHexString<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ToHexStringOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (input_arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        Ok((
            Self {
                input: input_arg,
                target,
            },
            input,
        ))
    }
}

impl<A: Allocator> core::fmt::Debug for ToHexString<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ToHexString")
            .field("input", &self.input)
            .field("target", &self.target)
            .finish()
    }
}

pub struct ToInteger<A: Allocator> {
    pub input: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ToInteger<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ToIntegerOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (input_arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        Ok((
            Self {
                input: input_arg,
                target,
            },
            input,
        ))
    }
}

impl<A: Allocator> core::fmt::Debug for ToInteger<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ToInteger")
            .field("input", &self.input)
            .field("target", &self.target)
            .finish()
    }
}

pub struct ToString<A: Allocator> {
    pub arg1: TermArg<A>,
    pub length_arg: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ToString<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ToStringOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (arg1, input) = TermArg::parse(input, context, alloc.clone())?;
        let (length_arg, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        Ok((
            Self {
                arg1,
                length_arg,
                target,
            },
            input,
        ))
    }
}

impl<A: Allocator> core::fmt::Debug for ToString<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ToString")
            .field("arg1", &self.arg1)
            .field("length_arg", &self.length_arg)
            .field("target", &self.target)
            .finish()
    }
}
