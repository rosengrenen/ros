mod break_point;
mod brk;
mod cont;
mod fatal;
mod if_else;
mod noop;
mod notify;
mod release;
mod reset;
mod ret;
mod signal;
mod sleep;
mod stall;
mod wheel;

use self::{
    break_point::BreakPoint,
    brk::Break,
    cont::Continue,
    fatal::Fatal,
    if_else::{Else, IfElse},
    noop::Noop,
    notify::Notify,
    release::Release,
    reset::Reset,
    ret::Return,
    signal::Signal,
    sleep::Sleep,
    stall::Stall,
    wheel::While,
};
use crate::aml::{
    aml::{name::SuperName, term::term_arg::TermArg},
    Context,
};
use alloc::boxed::Box;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
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
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            Break::p.map(Self::Break),
            BreakPoint::p.map(Self::BreakPoint),
            Continue::p.map(Self::Continue),
            Else::p.map_res1(|def_else| match def_else {
                Some(def_else) => Ok(Self::Else(def_else)),
                None => Err(()),
            }),
            Fatal::p.map(Self::Fatal),
            IfElse::p.map(Self::IfElse),
            Noop::p.map(Self::Noop),
            Notify::p.map(Self::Notify),
            Release::p.map(Self::Release),
            Reset::p.map(Self::Reset),
            Return::p.map(Self::Return),
            Signal::p.map(Self::Signal),
            Sleep::p.map(Self::Sleep),
            Stall::p.map(Self::Stall),
            While::p.map(Self::While),
        )
            .alt()
            .add_context("Statement")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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
