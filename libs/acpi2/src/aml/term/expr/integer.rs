use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::name::SuperName;
use crate::aml::name::Target;
use crate::aml::ops::AddOp;
use crate::aml::ops::DecrementOp;
use crate::aml::ops::DivideOp;
use crate::aml::ops::IncrementOp;
use crate::aml::ops::MultiplyOp;
use crate::aml::ops::SubtractOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::parser::ParserError;
use crate::aml::term::TermArg;

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

impl<A: Allocator> core::fmt::Debug for Integer<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Add(arg0) => f.debug_tuple("Add").field(arg0).finish(),
            Self::Multiply(arg0) => f.debug_tuple("Multiply").field(arg0).finish(),
            Self::Subtract(arg0) => f.debug_tuple("Subtract").field(arg0).finish(),
            Self::Divide(arg0) => f.debug_tuple("Divide").field(arg0).finish(),
            Self::Decrement(arg0) => f.debug_tuple("Decrement").field(arg0).finish(),
            Self::Increment(arg0) => f.debug_tuple("Increment").field(arg0).finish(),
        }
    }
}

pub struct Add<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for Add<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Add")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}

pub struct Multiply<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for Multiply<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Multiply")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}

pub struct Subtract<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for Subtract<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Subtract")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}

pub struct Divide<A: Allocator> {
    pub dividend: TermArg<A>,
    pub divisor: TermArg<A>,
    pub remainder: Target<A>,
    pub quotient: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for Divide<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Divide")
            .field("dividend", &self.dividend)
            .field("divisor", &self.divisor)
            .field("remainder", &self.remainder)
            .field("quotient", &self.quotient)
            .finish()
    }
}

pub struct Decrement<A: Allocator> {
    pub name: SuperName<A>,
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

impl<A: Allocator> core::fmt::Debug for Decrement<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Decrement")
            .field("name", &self.name)
            .finish()
    }
}

pub struct Increment<A: Allocator> {
    pub name: SuperName<A>,
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

impl<A: Allocator> core::fmt::Debug for Increment<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Increment")
            .field("name", &self.name)
            .finish()
    }
}
