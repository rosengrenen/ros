use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::{
        FromBcdOp, ToBcdOp, ToBufferOp, ToDecimalStringOp, ToHexStringOp, ToIntegerOp, ToStringOp,
    },
    parser::{fail, Input, ParseResult},
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
        if let Ok((value, input)) = FromBcd::parse(input, context, alloc.clone()) {
            return Ok((Self::FromBcd(value), input));
        }

        if let Ok((value, input)) = ToBcd::parse(input, context, alloc.clone()) {
            return Ok((Self::ToBcd(value), input));
        }

        if let Ok((value, input)) = ToBuffer::parse(input, context, alloc.clone()) {
            return Ok((Self::ToBuffer(value), input));
        }

        if let Ok((value, input)) = ToDecimalString::parse(input, context, alloc.clone()) {
            return Ok((Self::ToDecimalString(value), input));
        }

        if let Ok((value, input)) = ToHexString::parse(input, context, alloc.clone()) {
            return Ok((Self::ToHexString(value), input));
        }

        if let Ok((value, input)) = ToInteger::parse(input, context, alloc.clone()) {
            return Ok((Self::ToInteger(value), input));
        }

        let (value, input) = ToString::parse(input, context, alloc)?;
        Ok((Self::ToString(value), input))
    }
}

pub struct FromBcd<A: Allocator> {
    input: TermArg<A>,
    target: Target<A>,
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

pub struct ToBcd<A: Allocator> {
    input: TermArg<A>,
    target: Target<A>,
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

pub struct ToBuffer<A: Allocator> {
    input: TermArg<A>,
    target: Target<A>,
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

pub struct ToDecimalString<A: Allocator> {
    input: TermArg<A>,
    target: Target<A>,
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

pub struct ToHexString<A: Allocator> {
    input: TermArg<A>,
    target: Target<A>,
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

pub struct ToInteger<A: Allocator> {
    input: TermArg<A>,
    target: Target<A>,
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

pub struct ToString<A: Allocator> {
    arg1: TermArg<A>,
    length_arg: TermArg<A>,
    target: Target<A>,
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
