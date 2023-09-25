use crate::aml::{
    aml::{
        data::{byte_data, dword_data},
        name::SuperName,
        term::{TermArg, TermList},
    },
    ops::{
        BreakOp, BreakPointOp, ContinueOp, ElseOp, FatalOp, IfOp, NoopOp, NotifyOp, ReleaseOp,
        ResetOp, ReturnOp, SignalOp, SleepOp, StallOp, WhileOp,
    },
    pkg, prefixed, Context,
};
use alloc::boxed::Box;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::item::item,
};

pub enum StatementOpcode<A: Allocator> {
    DefBreak(DefBreak),
    DefBreakPoint(DefBreakPoint),
    DefContinue(DefContinue),
    DefElse(DefElse<A>),
    DefFatal(DefFatal<A>),
    DefIfElse(DefIfElse<A>),
    DefNoop(DefNoop),
    DefNotify(DefNotify<A>),
    DefRelease(DefRelease<A>),
    DefReset(DefReset<A>),
    DefReturn(DefReturn<A>),
    DefSignal(DefSignal<A>),
    DefSleep(DefSleep<A>),
    DefStall(DefStall<A>),
    DefWhile(DefWhile<A>),
}

impl<A: Allocator + Clone> StatementOpcode<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            DefBreak::p.map(Self::DefBreak),
            DefBreakPoint::p.map(Self::DefBreakPoint),
            DefContinue::p.map(Self::DefContinue),
            DefElse::p.map_res1(|def_else| match def_else {
                Some(def_else) => Ok(Self::DefElse(def_else)),
                None => Err(()),
            }),
            DefFatal::p.map(Self::DefFatal),
            DefIfElse::p.map(Self::DefIfElse),
            DefNoop::p.map(Self::DefNoop),
            DefNotify::p.map(Self::DefNotify),
            DefRelease::p.map(Self::DefRelease),
            DefReset::p.map(Self::DefReset),
            DefReturn::p.map(Self::DefReturn),
            DefSignal::p.map(Self::DefSignal),
            DefSleep::p.map(Self::DefSleep),
            DefStall::p.map(Self::DefStall),
            DefWhile::p.map(Self::DefWhile),
        )
            .alt()
            .add_context("StatementOpcode")
            .parse(input, context, alloc)
    }
}

pub struct DefBreak;

impl DefBreak {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        BreakOp::p
            .map(|_| Self)
            .add_context("DefBreak")
            .parse(input, context, alloc)
    }
}

pub struct DefBreakPoint;

impl DefBreakPoint {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        BreakPointOp::p
            .map(|_| DefBreakPoint)
            .add_context("DefBreakPoint")
            .parse(input, context, alloc)
    }
}

pub struct DefContinue;

impl DefContinue {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        ContinueOp::p
            .map(|_| Self)
            .add_context("DefContinue")
            .parse(input, context, alloc)
    }
}

pub struct DefElse<A: Allocator> {
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefElse<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Option<Self>, E> {
        prefixed(ElseOp::p, pkg(TermList::p))
            .map(|terms| Self { terms })
            .opt()
            .add_context("DefElse")
            .parse(input, context, alloc)
    }
}

pub struct DefFatal<A: Allocator> {
    ty: u8,
    code: u32,
    arg: TermArg<A>,
}

impl<A: Allocator + Clone> DefFatal<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let fatal_arg = TermArg::p;
        prefixed(FatalOp::p, (byte_data, dword_data, fatal_arg))
            .map(|(ty, code, arg)| Self { ty, code, arg })
            .add_context("DefFatal")
            .parse(input, context, alloc)
    }
}

pub struct DefIfElse<A: Allocator> {
    predicate: TermArg<A>,
    terms: TermList<A>,
    else_statement: Option<DefElse<A>>,
}

impl<A: Allocator + Clone> DefIfElse<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let predicate = TermArg::p; // => Integer
        prefixed(IfOp::p, pkg((predicate, TermList::p, DefElse::p)))
            .map(|(predicate, terms, else_statement)| Self {
                predicate,
                terms,
                else_statement,
            })
            .add_context("DefIfElse")
            .parse(input, context, alloc)
    }
}

pub struct DefNoop;

impl DefNoop {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        NoopOp::p
            .map(|_| Self)
            .add_context("DefNoop")
            .parse(input, context, alloc)
    }
}

pub struct DefNotify<A: Allocator> {
    obj: SuperName<A>,
    value: TermArg<A>,
}

impl<A: Allocator + Clone> DefNotify<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let notify_obj = SuperName::p; // => ThermalZone | Processor | Device
        let notify_value = TermArg::p; // => Integer
        prefixed(NotifyOp::p, (notify_obj, notify_value))
            .map(|(obj, value)| Self { obj, value })
            .add_context("DefNotify")
            .parse(input, context, alloc)
    }
}

pub struct DefRelease<A: Allocator> {
    mutex: MutexObj<A>,
}

impl<A: Allocator + Clone> DefRelease<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ReleaseOp::p, MutexObj::p)
            .map(|mutex| Self { mutex })
            .add_context("DefRelease")
            .parse(input, context, alloc)
    }
}

pub struct MutexObj<A: Allocator>(Box<SuperName<A>, A>);

impl<A: Allocator + Clone> MutexObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let box_alloc = alloc.clone();
        SuperName::p
            .map(move |name| Self(Box::new(name, box_alloc.clone()).unwrap()))
            .add_context("MutexObj")
            .parse(input, context, alloc)
    }
}

pub struct DefReset<A: Allocator> {
    event: EventObj<A>,
}

impl<A: Allocator + Clone> DefReset<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ResetOp::p, EventObj::p)
            .map(|event| Self { event })
            .add_context("DefReset")
            .parse(input, context, alloc)
    }
}

pub struct EventObj<A: Allocator>(SuperName<A>);

impl<A: Allocator + Clone> EventObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        SuperName::p
            .map(Self)
            .add_context("EventObj")
            .parse(input, context, alloc)
    }
}

pub struct DefReturn<A: Allocator> {
    arg: ArgObj<A>,
}

impl<A: Allocator + Clone> DefReturn<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ReturnOp::p, ArgObj::p)
            .map(|arg| Self { arg })
            .add_context("DefReturn")
            .parse(input, context, alloc)
    }
}

pub struct ArgObj<A: Allocator>(TermArg<A>);

impl<A: Allocator + Clone> ArgObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        TermArg::p
            .map(Self)
            .add_context("ArgObj")
            .parse(input, context, alloc)
    }
}

pub struct DefSignal<A: Allocator> {
    event: EventObj<A>,
}

impl<A: Allocator + Clone> DefSignal<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(SignalOp::p, EventObj::p)
            .map(|event| Self { event })
            .add_context("DefSignal")
            .parse(input, context, alloc)
    }
}

pub struct DefSleep<A: Allocator> {
    ms: TermArg<A>,
}

impl<A: Allocator + Clone> DefSleep<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let msec_time = TermArg::p; // => Integer
        prefixed(SleepOp::p, msec_time)
            .map(|ms| Self { ms })
            .add_context("DefSleep")
            .parse(input, context, alloc)
    }
}

pub struct DefStall<A: Allocator> {
    us: TermArg<A>,
}

impl<A: Allocator + Clone> DefStall<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let usec_time = TermArg::p; // => Integer
        prefixed(StallOp::p, usec_time)
            .map(|us| Self { us })
            .add_context("DefStall")
            .parse(input, context, alloc)
    }
}

pub struct DefWhile<A: Allocator> {
    predicate: TermArg<A>,
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefWhile<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let predicate = TermArg::p; // => Integer
        prefixed(WhileOp::p, pkg((predicate, TermList::p)))
            .map(|(predicate, terms)| Self { predicate, terms })
            .add_context("DefWhile")
            .parse(input, context, alloc)
    }
}
