use alloc::boxed::Box;
use alloc::vec::Vec;
use core::alloc::Allocator;

use super::TermArg;
use super::TermObj;
use crate::aml::context::Context;
use crate::aml::data::byte_data;
use crate::aml::data::dword_data;
use crate::aml::name::SuperName;
use crate::aml::ops::BreakOp;
use crate::aml::ops::BreakPointOp;
use crate::aml::ops::ContinueOp;
use crate::aml::ops::ElseOp;
use crate::aml::ops::FatalOp;
use crate::aml::ops::IfOp;
use crate::aml::ops::NoopOp;
use crate::aml::ops::NotifyOp;
use crate::aml::ops::ReleaseOp;
use crate::aml::ops::ResetOp;
use crate::aml::ops::ReturnOp;
use crate::aml::ops::SignalOp;
use crate::aml::ops::SleepOp;
use crate::aml::ops::StallOp;
use crate::aml::ops::WhileOp;
use crate::aml::parser::fail;
use crate::aml::parser::fail_if_not_empty;
use crate::aml::parser::Input;
use crate::aml::parser::ParseResult;
use crate::aml::parser::ParserError;
use crate::aml::pkg_len::pkg;

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

impl<A: Allocator> core::fmt::Debug for Statement<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Break(arg0) => f.debug_tuple("Break").field(arg0).finish(),
            Self::BreakPoint(arg0) => f.debug_tuple("BreakPoint").field(arg0).finish(),
            Self::Continue(arg0) => f.debug_tuple("Continue").field(arg0).finish(),
            Self::Else(arg0) => f.debug_tuple("Else").field(arg0).finish(),
            Self::Fatal(arg0) => f.debug_tuple("Fatal").field(arg0).finish(),
            Self::IfElse(arg0) => f.debug_tuple("IfElse").field(arg0).finish(),
            Self::Noop(arg0) => f.debug_tuple("Noop").field(arg0).finish(),
            Self::Notify(arg0) => f.debug_tuple("Notify").field(arg0).finish(),
            Self::Release(arg0) => f.debug_tuple("Release").field(arg0).finish(),
            Self::Reset(arg0) => f.debug_tuple("Reset").field(arg0).finish(),
            Self::Return(arg0) => f.debug_tuple("Return").field(arg0).finish(),
            Self::Signal(arg0) => f.debug_tuple("Signal").field(arg0).finish(),
            Self::Sleep(arg0) => f.debug_tuple("Sleep").field(arg0).finish(),
            Self::Stall(arg0) => f.debug_tuple("Stall").field(arg0).finish(),
            Self::While(arg0) => f.debug_tuple("While").field(arg0).finish(),
        }
    }
}

#[derive(Debug)]
pub struct BreakPoint;

impl BreakPoint {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = BreakPointOp::parse(input)?;
        Ok((Self, input))
    }
}

#[derive(Debug)]
pub struct Break;

impl Break {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = BreakOp::parse(input)?;
        Ok((Self, input))
    }
}

#[derive(Debug)]
pub struct Continue;

impl Continue {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = ContinueOp::parse(input)?;
        Ok((Self, input))
    }
}

pub struct Fatal<A: Allocator> {
    pub ty: u8,
    pub code: u32,
    pub arg: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Fatal<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Fatal")
            .field("ty", &self.ty)
            .field("code", &self.code)
            .field("arg", &self.arg)
            .finish()
    }
}

pub struct IfElse<A: Allocator> {
    pub predicate: TermArg<A>,
    pub terms: Vec<TermObj<A>, A>,
    pub else_statement: Option<Else<A>>,
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

impl<A: Allocator> core::fmt::Debug for IfElse<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IfElse")
            .field("predicate", &self.predicate)
            .field("terms", &self.terms)
            .field("else_statement", &self.else_statement)
            .finish()
    }
}

pub struct Else<A: Allocator> {
    pub terms: Vec<TermObj<A>, A>,
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

impl<A: Allocator> core::fmt::Debug for Else<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Else").field("terms", &self.terms).finish()
    }
}

#[derive(Debug)]
pub struct Noop;

impl Noop {
    pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
        let (_, input) = NoopOp::parse(input)?;
        Ok((Self, input))
    }
}

pub struct Notify<A: Allocator> {
    pub obj: SuperName<A>,
    pub value: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Notify<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Notify")
            .field("obj", &self.obj)
            .field("value", &self.value)
            .finish()
    }
}

pub struct Release<A: Allocator> {
    pub mutex: MutexObj<A>,
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

impl<A: Allocator> core::fmt::Debug for Release<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Release")
            .field("mutex", &self.mutex)
            .finish()
    }
}

pub struct Reset<A: Allocator> {
    pub event: EventObj<A>,
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

impl<A: Allocator> core::fmt::Debug for Reset<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Reset").field("event", &self.event).finish()
    }
}

pub struct Return<A: Allocator> {
    pub arg: ArgObj<A>,
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

impl<A: Allocator> core::fmt::Debug for Return<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Return").field("arg", &self.arg).finish()
    }
}

pub struct Signal<A: Allocator> {
    pub event: EventObj<A>,
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

impl<A: Allocator> core::fmt::Debug for Signal<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Signal")
            .field("event", &self.event)
            .finish()
    }
}

pub struct Sleep<A: Allocator> {
    pub ms: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Sleep<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Sleep").field("ms", &self.ms).finish()
    }
}

pub struct Stall<A: Allocator> {
    pub us: TermArg<A>,
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

impl<A: Allocator> core::fmt::Debug for Stall<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Stall").field("us", &self.us).finish()
    }
}

pub struct While<A: Allocator> {
    pub predicate: TermArg<A>,
    pub terms: Vec<TermObj<A>, A>,
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

impl<A: Allocator> core::fmt::Debug for While<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("While")
            .field("predicate", &self.predicate)
            .field("terms", &self.terms)
            .finish()
    }
}

pub struct MutexObj<A: Allocator>(pub Box<SuperName<A>, A>);

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

impl<A: Allocator> core::fmt::Debug for MutexObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("MutexObj").field(&self.0).finish()
    }
}

pub struct EventObj<A: Allocator>(pub SuperName<A>);

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

impl<A: Allocator> core::fmt::Debug for EventObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("EventObj").field(&self.0).finish()
    }
}

pub struct ArgObj<A: Allocator>(pub TermArg<A>);

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

impl<A: Allocator> core::fmt::Debug for ArgObj<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("ArgObj").field(&self.0).finish()
    }
}
