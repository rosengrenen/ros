use core::alloc::Allocator;

use crate::aml::{
    context::Context,
    name::Target,
    ops::{AndOp, NAndOp, NOrOp, NotOp, OrOp, ShiftLeftOp, ShiftRightOp, XOrOp},
    parser::{fail, Input, ParseResult},
    term::TermArg,
};

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
        if let Ok((value, input)) = And::parse(input, context, alloc.clone()) {
            return Ok((Self::And(value), input));
        }

        if let Ok((value, input)) = NAnd::parse(input, context, alloc.clone()) {
            return Ok((Self::NAnd(value), input));
        }

        if let Ok((value, input)) = NOr::parse(input, context, alloc.clone()) {
            return Ok((Self::NOr(value), input));
        }

        if let Ok((value, input)) = Or::parse(input, context, alloc.clone()) {
            return Ok((Self::Or(value), input));
        }

        if let Ok((value, input)) = XOr::parse(input, context, alloc.clone()) {
            return Ok((Self::XOr(value), input));
        }

        if let Ok((value, input)) = Not::parse(input, context, alloc.clone()) {
            return Ok((Self::Not(value), input));
        }

        if let Ok((value, input)) = ShiftLeft::parse(input, context, alloc.clone()) {
            return Ok((Self::ShiftLeft(value), input));
        }

        let (value, input) = ShiftRight::parse(input, context, alloc)?;
        Ok((Self::ShiftRight(value), input))
    }
}

pub struct And<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
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

pub struct NAnd<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
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

pub struct NOr<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
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

pub struct Or<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
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

pub struct XOr<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
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

pub struct Not<A: Allocator> {
    operand: TermArg<A>,
    target: Target<A>,
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

pub struct ShiftLeft<A: Allocator> {
    operand: TermArg<A>,
    shift_count: TermArg<A>,
    target: Target<A>,
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

pub struct ShiftRight<A: Allocator> {
    operand: TermArg<A>,
    shift_count: TermArg<A>,
    target: Target<A>,
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
