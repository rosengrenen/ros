use core::alloc::Allocator;

use crate::aml::context::Context;
use crate::aml::name::Target;
use crate::aml::ops::AndOp;
use crate::aml::ops::NAndOp;
use crate::aml::ops::NOrOp;
use crate::aml::ops::NotOp;
use crate::aml::ops::OrOp;
use crate::aml::ops::ShiftLeftOp;
use crate::aml::ops::ShiftRightOp;
use crate::aml::ops::XOrOp;
use crate::aml::parser::fail;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::parser::ParserError;
use crate::aml::term::TermArg;

pub enum Bitwise<A: Allocator> {
    And(And<A>),
    NAnd(NAnd<A>),
    NOr(NOr<A>),
    Or(Or<A>),
    XOr(XOr<A>),
    Not(Not<A>),
    ShiftLeft(ShiftLeft<A>),
    ShiftRight(ShiftRight<A>),
}

impl<A: Allocator + Clone> Bitwise<A> {
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

        match NAnd::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::NAnd(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match NOr::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::NOr(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Or::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Or(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match XOr::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::XOr(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Not::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Not(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ShiftLeft::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ShiftLeft(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = ShiftRight::parse(input, context, alloc)?;
        Ok((Self::ShiftRight(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for Bitwise<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::And(arg0) => f.debug_tuple("And").field(arg0).finish(),
            Self::NAnd(arg0) => f.debug_tuple("NAnd").field(arg0).finish(),
            Self::NOr(arg0) => f.debug_tuple("NOr").field(arg0).finish(),
            Self::Or(arg0) => f.debug_tuple("Or").field(arg0).finish(),
            Self::XOr(arg0) => f.debug_tuple("XOr").field(arg0).finish(),
            Self::Not(arg0) => f.debug_tuple("Not").field(arg0).finish(),
            Self::ShiftLeft(arg0) => f.debug_tuple("ShiftLeft").field(arg0).finish(),
            Self::ShiftRight(arg0) => f.debug_tuple("ShiftRight").field(arg0).finish(),
        }
    }
}

pub struct And<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> And<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = AndOp::parse(input)?;
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

impl<A: Allocator> core::fmt::Debug for And<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("And")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}

pub struct NAnd<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> NAnd<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = NAndOp::parse(input)?;
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

impl<A: Allocator> core::fmt::Debug for NAnd<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NAnd")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}

pub struct NOr<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> NOr<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = NOrOp::parse(input)?;
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

impl<A: Allocator> core::fmt::Debug for NOr<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("NOr")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}

pub struct Or<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> Or<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = OrOp::parse(input)?;
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

impl<A: Allocator> core::fmt::Debug for Or<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Or")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}

pub struct XOr<A: Allocator> {
    pub left: TermArg<A>,
    pub right: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> XOr<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = XOrOp::parse(input)?;
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

impl<A: Allocator> core::fmt::Debug for XOr<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("XOr")
            .field("left", &self.left)
            .field("right", &self.right)
            .field("target", &self.target)
            .finish()
    }
}

pub struct Not<A: Allocator> {
    pub operand: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> Not<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = NotOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (operand, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        Ok((Self { operand, target }, input))
    }
}

impl<A: Allocator> core::fmt::Debug for Not<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Not")
            .field("operand", &self.operand)
            .field("target", &self.target)
            .finish()
    }
}

pub struct ShiftLeft<A: Allocator> {
    pub operand: TermArg<A>,
    pub shift_count: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ShiftLeft<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ShiftLeftOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (operand, input) = TermArg::parse(input, context, alloc.clone())?;
        let (shift_count, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        let this = Self {
            operand,
            shift_count,
            target,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for ShiftLeft<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ShiftLeft")
            .field("operand", &self.operand)
            .field("shift_count", &self.shift_count)
            .field("target", &self.target)
            .finish()
    }
}

pub struct ShiftRight<A: Allocator> {
    pub operand: TermArg<A>,
    pub shift_count: TermArg<A>,
    pub target: Target<A>,
}

impl<A: Allocator + Clone> ShiftRight<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ShiftRightOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (operand, input) = TermArg::parse(input, context, alloc.clone())?;
        let (shift_count, input) = TermArg::parse(input, context, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        let this = Self {
            operand,
            shift_count,
            target,
        };
        Ok((this, input))
    }
}

impl<A: Allocator> core::fmt::Debug for ShiftRight<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ShiftRight")
            .field("operand", &self.operand)
            .field("shift_count", &self.shift_count)
            .field("target", &self.target)
            .finish()
    }
}
