use core::alloc::Allocator;

use super::{TermArg, TermObj};
use crate::aml::{
    context::Context,
    data::{byte_data, dword_data},
    name::SuperName,
    ops::{
        BreakOp, BreakPointOp, ContinueOp, ElseOp, FatalOp, IfOp, NoopOp, NotifyOp, ReleaseOp,
        ResetOp, ReturnOp, SignalOp, SleepOp, StallOp, WhileOp,
    },
    parser::{fail, fail_if_not_empty, Input, ParseResult, ParserError},
    pkg_len::pkg,
};
use alloc::{boxed::Box, vec::Vec};

pub enum Statement<A: Allocator> {
    Break(Break),
    BreakPoint(BreakPoint),
    Continue(Continue),
    Else(Else<A>),
    Fatal(Fatal<A>),
    IfElse(IfElse<A>),
    Noop(Noop),
    Notify(Notify<A>),
    Release(Release<A>),
    Reset(Reset<A>),
    Return(Return<A>),
    Signal(Signal<A>),
    Sleep(Sleep<A>),
    Stall(Stall<A>),
    While(While<A>),
}

impl<A: Allocator + Clone> Statement<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match Break::parse(input) {
            Ok((value, input)) => return Ok((Self::Break(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match BreakPoint::parse(input) {
            Ok((value, input)) => return Ok((Self::BreakPoint(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Continue::parse(input) {
            Ok((value, input)) => return Ok((Self::Continue(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Else::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Else(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Fatal::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Fatal(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match IfElse::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::IfElse(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Noop::parse(input) {
            Ok((value, input)) => return Ok((Self::Noop(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Notify::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Notify(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Release::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Release(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Reset::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Reset(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Return::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Return(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Signal::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Signal(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Sleep::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Sleep(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Stall::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Stall(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = While::parse(input, context, alloc)?;
        Ok((Self::While(value), input))
    }
}

pub struct BreakPoint;

impl BreakPoint {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = BreakPointOp::parse(input)?;
        Ok((Self, input))
    }
}

pub struct Break;

impl Break {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = BreakOp::parse(input)?;
        Ok((Self, input))
    }
}

pub struct Continue;

impl Continue {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = ContinueOp::parse(input)?;
        Ok((Self, input))
    }
}

pub struct Fatal<A: Allocator> {
    ty: u8,
    code: u32,
    arg: TermArg<A>,
}

impl<A: Allocator + Clone> Fatal<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = FatalOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (ty, input) = byte_data(input)?;
        let (code, input) = dword_data(input)?;
        let (arg, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { ty, code, arg }, input))
    }
}

pub struct IfElse<A: Allocator> {
    predicate: TermArg<A>,
    terms: Vec<TermObj<A>, A>,
    else_statement: Option<Else<A>>,
}

impl<A: Allocator + Clone> IfElse<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = IfOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (if_else, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((if_else, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (predicate, mut input) = TermArg::parse(input, context, alloc.clone())?;
        let mut terms = Vec::new(alloc.clone());
        while let Ok((term, i)) = TermObj::parse(input, context, alloc.clone()) {
            terms.push(term).unwrap();
            input = i;
        }

        let (else_statement, input) = match Else::parse(input, context, alloc) {
            Ok((value, input)) => (Some(value), input),
            Err(_) => (None, input),
        };

        let this = Self {
            predicate,
            terms,
            else_statement,
        };
        Ok((this, input))
    }
}

pub struct Else<A: Allocator> {
    terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> Else<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ElseOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (if_else, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((if_else, input))
    }

    fn parse_inner<'a>(
        mut input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let mut terms = Vec::new(alloc.clone());
        while let Ok((term, i)) = TermObj::parse(input, context, alloc.clone()) {
            terms.push(term).unwrap();
            input = i;
        }

        Ok((Self { terms }, input))
    }
}

pub struct Noop;

impl Noop {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = NoopOp::parse(input)?;
        Ok((Self, input))
    }
}

pub struct Notify<A: Allocator> {
    obj: SuperName<A>,
    value: TermArg<A>,
}

impl<A: Allocator + Clone> Notify<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = NotifyOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (obj, input) = SuperName::parse(input, context, alloc.clone())?;
        let (value, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { obj, value }, input))
    }
}

pub struct Release<A: Allocator> {
    mutex: MutexObj<A>,
}

impl<A: Allocator + Clone> Release<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ReleaseOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (mutex, input) = MutexObj::parse(input, context, alloc)?;
        Ok((Self { mutex }, input))
    }
}

pub struct Reset<A: Allocator> {
    event: EventObj<A>,
}

impl<A: Allocator + Clone> Reset<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ResetOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (event, input) = EventObj::parse(input, context, alloc)?;
        Ok((Self { event }, input))
    }
}

pub struct Return<A: Allocator> {
    arg: ArgObj<A>,
}

impl<A: Allocator + Clone> Return<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = ReturnOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (arg, input) = ArgObj::parse(input, context, alloc)?;
        Ok((Self { arg }, input))
    }
}

pub struct Signal<A: Allocator> {
    event: EventObj<A>,
}

impl<A: Allocator + Clone> Signal<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = SignalOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (event, input) = EventObj::parse(input, context, alloc)?;
        Ok((Self { event }, input))
    }
}

pub struct Sleep<A: Allocator> {
    ms: TermArg<A>,
}

impl<A: Allocator + Clone> Sleep<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = SleepOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (ms, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { ms }, input))
    }
}

pub struct Stall<A: Allocator> {
    us: TermArg<A>,
}

impl<A: Allocator + Clone> Stall<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = StallOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (us, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self { us }, input))
    }
}

pub struct While<A: Allocator> {
    predicate: TermArg<A>,
    terms: Vec<TermObj<A>, A>,
}

impl<A: Allocator + Clone> While<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = WhileOp::parse(input)?;
        let (pkg_input, input) = pkg(input)?;
        let (while_statement, pkg_input) = fail(Self::parse_inner(pkg_input, context, alloc))?;
        fail_if_not_empty(pkg_input)?;
        Ok((while_statement, input))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (predicate, mut input) = TermArg::parse(input, context, alloc.clone())?;
        let mut terms = Vec::new(alloc.clone());
        while let Ok((term, i)) = TermObj::parse(input, context, alloc.clone()) {
            terms.push(term).unwrap();
            input = i;
        }

        Ok((Self { predicate, terms }, input))
    }
}

pub struct MutexObj<A: Allocator>(Box<SuperName<A>, A>);

impl<A: Allocator + Clone> MutexObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (super_name, input) = SuperName::parse(input, context, alloc.clone())?;
        let super_name = Box::new(super_name, alloc).unwrap();
        Ok((Self(super_name), input))
    }
}

pub struct EventObj<A: Allocator>(SuperName<A>);

impl<A: Allocator + Clone> EventObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (super_name, input) = SuperName::parse(input, context, alloc)?;
        Ok((Self(super_name), input))
    }
}

pub struct ArgObj<A: Allocator>(TermArg<A>);

impl<A: Allocator + Clone> ArgObj<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (arg, input) = TermArg::parse(input, context, alloc)?;
        Ok((Self(arg), input))
    }
}
