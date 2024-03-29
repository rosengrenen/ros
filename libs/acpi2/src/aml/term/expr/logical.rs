use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    ops::{
        LAndOp, LEqualOp, LGreaterEqualOp, LGreaterOp, LLessEqualOp, LLessOp, LNotEqualOp, LNotOp,
        LOrOp,
    },
    parser::{fail, Input, ParseResult, ParserError},
    term::TermArg,
};

pub enum Logical<A: Allocator> {
    And(And<A>),
    Equal(Equal<A>),
    GreaterEqual(GreaterEqual<A>),
    Greater(Greater<A>),
    LessEqual(LessEqual<A>),
    Less(Less<A>),
    NotEqual(NotEqual<A>),
    Or(Or<A>),
    Not(Not<A>),
}

impl<A: Allocator + Clone> Logical<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match And::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::And(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Equal::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Equal(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match GreaterEqual::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::GreaterEqual(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Greater::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Greater(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match LessEqual::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::LessEqual(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Less::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Less(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match NotEqual::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::NotEqual(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Or::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Or(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = Not::parse(input, context, alloc)?;
        Ok((Self::Not(value), input))
    }
}

pub struct And<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
}

impl<A: Allocator + Clone> And<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LAndOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { left, right }, input))
    }
}

pub struct Equal<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
}

impl<A: Allocator + Clone> Equal<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LEqualOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { left, right }, input))
    }
}

pub struct GreaterEqual<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
}

impl<A: Allocator + Clone> GreaterEqual<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LGreaterEqualOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { left, right }, input))
    }
}

pub struct Greater<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
}

impl<A: Allocator + Clone> Greater<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LGreaterOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { left, right }, input))
    }
}

pub struct LessEqual<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
}

impl<A: Allocator + Clone> LessEqual<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LLessEqualOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { left, right }, input))
    }
}

pub struct Less<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
}

impl<A: Allocator + Clone> Less<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LLessOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { left, right }, input))
    }
}

pub struct NotEqual<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
}

impl<A: Allocator + Clone> NotEqual<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LNotEqualOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { left, right }, input))
    }
}

pub struct Or<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
}

impl<A: Allocator + Clone> Or<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LOrOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (left, input) = TermArg::parse(input, context, alloc.clone())?;
        let (right, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { left, right }, input))
    }
}

pub struct Not<A: Allocator> {
    operand: TermArg<A>,
}

impl<A: Allocator + Clone> Not<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LNotOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (operand, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { operand }, input))
    }
}
