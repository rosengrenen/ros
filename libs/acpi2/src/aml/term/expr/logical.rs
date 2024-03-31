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

impl<A: Allocator> core::fmt::Debug for Logical<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::And(arg0) => f.debug_tuple("And").field(arg0).finish(),
            Self::Equal(arg0) => f.debug_tuple("Equal").field(arg0).finish(),
            Self::GreaterEqual(arg0) => f.debug_tuple("GreaterEqual").field(arg0).finish(),
            Self::Greater(arg0) => f.debug_tuple("Greater").field(arg0).finish(),
            Self::LessEqual(arg0) => f.debug_tuple("LessEqual").field(arg0).finish(),
            Self::Less(arg0) => f.debug_tuple("Less").field(arg0).finish(),
            Self::NotEqual(arg0) => f.debug_tuple("NotEqual").field(arg0).finish(),
            Self::Or(arg0) => f.debug_tuple("Or").field(arg0).finish(),
            Self::Not(arg0) => f.debug_tuple("Not").field(arg0).finish(),
        }
    }
}

pub struct And<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for And<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("And")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

pub struct Equal<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Equal<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Equal")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

pub struct GreaterEqual<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for GreaterEqual<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("GreaterEqual")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

pub struct Greater<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Greater<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Greater")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

pub struct LessEqual<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for LessEqual<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("LessEqual")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

pub struct Less<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Less<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Less")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

pub struct NotEqual<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for NotEqual<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NotEqual")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

pub struct Or<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Or<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Or")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

pub struct Not<A: Allocator> {
    pub operand: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Not<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Not")
            .field("operand", &self.operand)
            .finish()
    }
}
