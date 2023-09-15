use crate::aml::{
    data::{byte_data, dword_data, ExtOpPrefix},
    name::SuperName,
    pkg,
    term::{TermArg, TermList},
};
use alloc::boxed::Box;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::item::item,
    sequence::preceded,
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
            .parse(input, alloc)
    }
}

pub struct DefBreak;

impl DefBreak {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let break_op = item(0xa5);
        break_op
            .map(|_| Self)
            .add_context("DefBreak")
            .parse(input, alloc)
    }
}

pub struct DefBreakPoint;

impl DefBreakPoint {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let break_point_op = item(0xcc);
        break_point_op
            .map(|_| DefBreakPoint)
            .add_context("DefBreakPoint")
            .parse(input, alloc)
    }
}

pub struct DefContinue;

impl DefContinue {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let continue_op = item(0x9f);
        continue_op
            .map(|_| Self)
            .add_context("DefContinue")
            .parse(input, alloc)
    }
}

pub struct DefElse<A: Allocator> {
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefElse<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Option<Self>, E> {
        let else_op = item(0xa1);
        preceded(else_op, pkg(TermList::p))
            .map(|terms| Self { terms })
            .opt()
            .add_context("DefElse")
            .parse(input, alloc)
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
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let fatal_op = (ExtOpPrefix::p, item(0x32));
        let fatal_arg = TermArg::p;
        preceded(fatal_op, (byte_data, dword_data, fatal_arg).cut())
            .map(|(ty, code, arg)| Self { ty, code, arg })
            .add_context("DefFatal")
            .parse(input, alloc)
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
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let if_op = item(0xa0);
        let predicate = TermArg::p; // => Integer
        preceded(if_op, pkg((predicate, TermList::p, DefElse::p)))
            .map(|(predicate, terms, else_statement)| Self {
                predicate,
                terms,
                else_statement,
            })
            .add_context("DefIfElse")
            .parse(input, alloc)
    }
}

pub struct DefNoop;

impl DefNoop {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let noop_op = item(0xa3);
        noop_op
            .map(|_| Self)
            .add_context("DefNoop")
            .parse(input, alloc)
    }
}

pub struct DefNotify<A: Allocator> {
    obj: SuperName<A>,
    value: TermArg<A>,
}

impl<A: Allocator + Clone> DefNotify<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let notify_op = item(0x86);
        let notify_obj = SuperName::p; // => ThermalZone | Processor | Device
        let notify_value = TermArg::p; // => Integer
        preceded(notify_op, (notify_obj, notify_value).cut())
            .map(|(obj, value)| Self { obj, value })
            .add_context("DefNotify")
            .parse(input, alloc)
    }
}

pub struct DefRelease<A: Allocator> {
    mutex: MutexObj<A>,
}

impl<A: Allocator + Clone> DefRelease<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let release_op = (ExtOpPrefix::p, item(0x27));
        preceded(release_op, MutexObj::p.cut())
            .map(|mutex| Self { mutex })
            .add_context("DefRelease")
            .parse(input, alloc)
    }
}

pub struct MutexObj<A: Allocator>(Box<SuperName<A>, A>);

impl<A: Allocator + Clone> MutexObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let box_alloc = alloc.clone();
        SuperName::p
            .map(move |name| Self(Box::new(name, box_alloc.clone()).unwrap()))
            .add_context("MutexObj")
            .parse(input, alloc)
    }
}

pub struct DefReset<A: Allocator> {
    event: EventObj<A>,
}

impl<A: Allocator + Clone> DefReset<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let release_op = (ExtOpPrefix::p, item(0x26));
        preceded(release_op, EventObj::p.cut())
            .map(|event| Self { event })
            .add_context("DefReset")
            .parse(input, alloc)
    }
}

pub struct EventObj<A: Allocator>(SuperName<A>);

impl<A: Allocator + Clone> EventObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        SuperName::p
            .map(Self)
            .add_context("EventObj")
            .parse(input, alloc)
    }
}

pub struct DefReturn<A: Allocator> {
    arg: ArgObj<A>,
}

impl<A: Allocator + Clone> DefReturn<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let return_op = item(0xa4);
        preceded(return_op, ArgObj::p.cut())
            .map(|arg| Self { arg })
            .add_context("DefReturn")
            .parse(input, alloc)
    }
}

pub struct ArgObj<A: Allocator>(TermArg<A>);

impl<A: Allocator + Clone> ArgObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        TermArg::p
            .map(Self)
            .add_context("ArgObj")
            .parse(input, alloc)
    }
}

pub struct DefSignal<A: Allocator> {
    event: EventObj<A>,
}

impl<A: Allocator + Clone> DefSignal<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let signal_op = (ExtOpPrefix::p, item(0x24));
        preceded(signal_op, EventObj::p.cut())
            .map(|event| Self { event })
            .add_context("DefSignal")
            .parse(input, alloc)
    }
}

pub struct DefSleep<A: Allocator> {
    ms: TermArg<A>,
}

impl<A: Allocator + Clone> DefSleep<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let sleep_op = (ExtOpPrefix::p, item(0x22));
        let msec_time = TermArg::p; // => Integer
        preceded(sleep_op, msec_time.cut())
            .map(|ms| Self { ms })
            .add_context("DefSleep")
            .parse(input, alloc)
    }
}

pub struct DefStall<A: Allocator> {
    us: TermArg<A>,
}

impl<A: Allocator + Clone> DefStall<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let stall_op = (ExtOpPrefix::p, item(0x21));
        let usec_time = TermArg::p; // => Integer
        preceded(stall_op, usec_time.cut())
            .map(|us| Self { us })
            .add_context("DefStall")
            .parse(input, alloc)
    }
}

pub struct DefWhile<A: Allocator> {
    predicate: TermArg<A>,
    terms: TermList<A>,
}

impl<A: Allocator + Clone> DefWhile<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let stall_op = (ExtOpPrefix::p, item(0x21));
        let predicate = TermArg::p; // => Integer
        preceded(stall_op, pkg((predicate, TermList::p)))
            .map(|(predicate, terms)| Self { predicate, terms })
            .add_context("DefWhile")
            .parse(input, alloc)
    }
}
