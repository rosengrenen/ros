use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::{SuperName, Target},
    ops::{AddOp, DecrementOp, DivideOp, IncrementOp, MultiplyOp, SubtractOp},
    parser::{fail, Input, ParseResult, ParserError},
    term::TermArg,
};

pub enum Integer<A: Allocator> {
    Add(Add<A>),
    Multiply(Multiply<A>),
    Subtract(Subtract<A>),
    Divide(Divide<A>),
    Decrement(Decrement<A>),
    Increment(Increment<A>),
}

impl<A: Allocator + Clone> Integer<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match Add::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Add(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Multiply::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Multiply(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Subtract::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Subtract(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Divide::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Divide(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Decrement::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Decrement(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Increment::parse(input, context, alloc)?;
        Ok((Self::Increment(value), input))
    }
}

pub struct Add<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> Add<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = AddOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        let this = Self {
            left,
            right,
            target,
        };
        Ok((this, input))
    }
}

pub struct Multiply<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> Multiply<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = MultiplyOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        let this = Self {
            left,
            right,
            target,
        };
        Ok((this, input))
    }
}

pub struct Subtract<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> Subtract<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = SubtractOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc.clone())?;
        let this = Self {
            left,
            right,
            target,
        };
        Ok((this, input))
    }
}

pub struct Divide<A: Allocator> {
    dividend: TermArg<A>,
    divisor: TermArg<A>,
    remainder: Target<A>,
    quotient: Target<A>,
}

impl<A: Allocator + Clone> Divide<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = DivideOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (dividend, input) = TermArg::parse(input, context, alloc.clone())?;
        let (divisor, input) = TermArg::parse(input, context, alloc.clone())?;
        let (remainder, input) = Target::parse(input, context, alloc.clone())?;
        let (quotient, input) = Target::parse(input, context, alloc.clone())?;
        let this = Self {
            dividend,
            divisor,
            remainder,
            quotient,
        };
        Ok((this, input))
    }
}

pub struct Decrement<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> Decrement<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = DecrementOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = SuperName::parse(input, context, alloc.clone())?;
        Ok((Self { name }, input))
    }
}

pub struct Increment<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> Increment<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = IncrementOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = SuperName::parse(input, context, alloc.clone())?;
        Ok((Self { name }, input))
    }
}
