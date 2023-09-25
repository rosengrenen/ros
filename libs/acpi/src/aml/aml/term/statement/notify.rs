use crate::aml::{
    aml::{name::SuperName, term::TermArg},
    ops::NotifyOp,
    prefixed, Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

pub struct Notify<A: Allocator> {
    pub obj: SuperName<A>,
    pub value: TermArg<A>,
}

impl<A: Allocator + Clone> Notify<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let notify_obj = SuperName::p; // => ThermalZone | Processor | Device
        let notify_value = TermArg::p; // => Integer
        prefixed(NotifyOp::p, (notify_obj, notify_value))
            .map(|(obj, value)| Self { obj, value })
            .add_context("Notify")
            .parse(input, context, alloc)
    }
}
