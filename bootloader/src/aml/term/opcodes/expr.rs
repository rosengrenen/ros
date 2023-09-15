use super::statement::{EventObj, MutexObj};
use crate::aml::{
    data::{byte_data, ByteList, DataRefObj, ExtOpPrefix},
    misc::DebugObj,
    name::{NameString, SimpleName, SuperName, Target},
    pkg,
    term::{MethodInvocation, TermArg},
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
    primitive::item::item,
    sequence::preceded,
};

pub enum ExprOpcode<A: Allocator> {
    DefAcquire(DefAcquire<A>),
    DefAdd(DefAdd<A>),
    DefAnd(DefAnd<A>),
    DefBuffer(DefBuffer<A>),
    DefConcat(DefConcat<A>),
    DefConcatRes(DefConcatRes<A>),
    DefCondRefOf(DefCondRefOf<A>),
    DefCopyObj(DefCopyObj<A>),
    DefDecrement(DefDecrement<A>),
    DefDerefOf(DefDerefOf<A>),
    DefDivide(DefDivide<A>),
    DefFindSetLeftBit(DefFindSetLeftBit<A>),
    DefFindSetRightBit(DefFindSetRightBit<A>),
    DefFromBcd(DefFromBcd<A>),
    DefIncrement(DefIncrement<A>),
    DefIndex(DefIndex<A>),
    DefLAnd(DefLAnd<A>),
    DefLEqual(DefLEqual<A>),
    DefLGreater(DefLGreater<A>),
    DefLGreaterEqual(DefLGreaterEqual<A>),
    DefLLess(DefLLess<A>),
    DefLLessEqual(DefLLessEqual<A>),
    DefMid(DefMid<A>),
    DefLNot(DefLNot<A>),
    DefLNotEqual(DefLNotEqual<A>),
    DefLoadTable(DefLoadTable<A>),
    DefLOr(DefLOr<A>),
    DefMatch(DefMatch<A>),
    DefMod(DefMod<A>),
    DefMultiply(DefMultiply<A>),
    DefNAnd(DefNAnd<A>),
    DefNOr(DefNOr<A>),
    DefNot(DefNot<A>),
    DefObjType(DefObjType<A>),
    DefOr(DefOr<A>),
    DefPkg(DefPkg<A>),
    DefVarPkg(DefVarPkg<A>),
    DefRefOf(DefRefOf<A>),
    DefShiftLeft(DefShiftLeft<A>),
    DefShiftRight(DefShiftRight<A>),
    DefSizeOf(DefSizeOf<A>),
    DefStore(DefStore<A>),
    DefSubtract(DefSubtract<A>),
    DefTimer(DefTimer),
    DefToBcd(DefToBcd<A>),
    DefToBuffer(DefToBuffer<A>),
    DefToDecimalString(DefToDecimalString<A>),
    DefToHexString(DefToHexString<A>),
    DefToInteger(DefToInteger<A>),
    DefToString(DefToString<A>),
    DefWait(DefWait<A>),
    DefXOr(DefXOr<A>),
    MethodInvocation(MethodInvocation<A>),
}

impl<A: Allocator + Clone> ExprOpcode<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            (
                DefAcquire::p.map(Self::DefAcquire),
                DefAdd::p.map(Self::DefAdd),
                DefAnd::p.map(Self::DefAnd),
                DefBuffer::p.map(Self::DefBuffer),
                DefConcat::p.map(Self::DefConcat),
                DefConcatRes::p.map(Self::DefConcatRes),
                DefCondRefOf::p.map(Self::DefCondRefOf),
                DefCopyObj::p.map(Self::DefCopyObj),
                DefDecrement::p.map(Self::DefDecrement),
                DefDerefOf::p.map(Self::DefDerefOf),
                DefDivide::p.map(Self::DefDivide),
                DefFindSetLeftBit::p.map(Self::DefFindSetLeftBit),
                DefFindSetRightBit::p.map(Self::DefFindSetRightBit),
                DefFromBcd::p.map(Self::DefFromBcd),
                DefIncrement::p.map(Self::DefIncrement),
                DefIndex::p.map(Self::DefIndex),
            )
                .alt(),
            (
                DefLAnd::p.map(Self::DefLAnd),
                DefLEqual::p.map(Self::DefLEqual),
                DefLGreater::p.map(Self::DefLGreater),
                DefLGreaterEqual::p.map(Self::DefLGreaterEqual),
                DefLLess::p.map(Self::DefLLess),
                DefLLessEqual::p.map(Self::DefLLessEqual),
                DefMid::p.map(Self::DefMid),
                DefLNot::p.map(Self::DefLNot),
                DefLNotEqual::p.map(Self::DefLNotEqual),
                DefLoadTable::p.map(Self::DefLoadTable),
                DefLOr::p.map(Self::DefLOr),
                DefMatch::p.map(Self::DefMatch),
                DefMod::p.map(Self::DefMod),
                DefMultiply::p.map(Self::DefMultiply),
                DefNAnd::p.map(Self::DefNAnd),
                DefNOr::p.map(Self::DefNOr),
            )
                .alt(),
            (
                DefNot::p.map(Self::DefNot),
                DefObjType::p.map(Self::DefObjType),
                DefOr::p.map(Self::DefOr),
                DefPkg::p.map(Self::DefPkg),
                DefVarPkg::p.map(Self::DefVarPkg),
                DefRefOf::p.map(Self::DefRefOf),
                DefShiftLeft::p.map(Self::DefShiftLeft),
                DefShiftRight::p.map(Self::DefShiftRight),
                DefSizeOf::p.map(Self::DefSizeOf),
                DefStore::p.map(Self::DefStore),
                DefSubtract::p.map(Self::DefSubtract),
                DefTimer::p.map(Self::DefTimer),
                DefToBcd::p.map(Self::DefToBcd),
                DefToBuffer::p.map(Self::DefToBuffer),
                DefToDecimalString::p.map(Self::DefToDecimalString),
                DefToHexString::p.map(Self::DefToHexString),
            )
                .alt(),
            DefToInteger::p.map(Self::DefToInteger),
            DefToString::p.map(Self::DefToString),
            DefWait::p.map(Self::DefWait),
            DefXOr::p.map(Self::DefXOr),
            MethodInvocation::p.map(Self::MethodInvocation),
        )
            .alt()
            .parse(input, alloc)
    }
}

pub enum RefTypeOpcode<A: Allocator> {
    DefRefOf(DefRefOf<A>),
    DefDerefOf(DefDerefOf<A>),
    DefIndex(DefIndex<A>),
    UserTermObj(MethodInvocation<A>),
}

impl<A: Allocator + Clone> RefTypeOpcode<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            DefRefOf::p.map(Self::DefRefOf),
            DefDerefOf::p.map(Self::DefDerefOf),
            DefIndex::p.map(Self::DefIndex),
            MethodInvocation::p.map(Self::UserTermObj),
        )
            .alt()
            .parse(input, alloc)
    }
}

pub struct DefAcquire<A: Allocator> {
    mutex: MutexObj<A>,
    timeout: Timeout,
}

impl<A: Allocator + Clone> DefAcquire<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let acquire_op = (ExtOpPrefix::p, item(0x23));
        preceded(acquire_op, (MutexObj::p, Timeout::p).cut())
            .map(|(mutex, timeout)| Self { mutex, timeout })
            .parse(input, alloc)
    }
}

pub struct Timeout(u8);

impl Timeout {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data.map(Self).parse(input, alloc)
    }
}

pub struct DefAdd<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefAdd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let add_op = item(0x72);
        preceded(add_op, (Operand::p, Operand::p, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct Operand<A: Allocator>(TermArg<A>);

impl<A: Allocator + Clone> Operand<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        TermArg::p.map(Self).parse(input, alloc)
    }
}

pub struct DefAnd<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefAnd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let and_op = item(0x7b);
        preceded(and_op, (Operand::p, Operand::p, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefBuffer<A: Allocator> {
    len: TermArg<A>,
    bytes: ByteList<A>,
}

impl<A: Allocator + Clone> DefBuffer<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let buffer_op = item(0x11);
        let buffer_size = TermArg::p; // => Integer
        preceded(buffer_op, pkg((buffer_size, ByteList::p)))
            .map(|(len, bytes)| Self { len, bytes })
            .parse(input, alloc)
    }
}

pub struct DefConcat<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefConcat<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let concat_op = item(0x73);
        let data = TermArg::p; // => ComputationalData
        preceded(concat_op, (data, data, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefConcatRes<A: Allocator> {
    left: TermArg<A>,
    right: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefConcatRes<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let concat_res_op = item(0x84);
        let buf_data = TermArg::p; // => Buffer
        preceded(concat_res_op, (buf_data, buf_data, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefCondRefOf<A: Allocator> {
    name: SuperName<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefCondRefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let cond_ref_of_op = (ExtOpPrefix::p, item(0x84));
        preceded(cond_ref_of_op, (SuperName::p, Target::p).cut())
            .map(|(name, target)| Self { name, target })
            .parse(input, alloc)
    }
}

pub struct DefCopyObj<A: Allocator> {
    arg: TermArg<A>,
    name: SimpleName<A>,
}

impl<A: Allocator + Clone> DefCopyObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let copy_obj_op = item(0x9d);
        preceded(copy_obj_op, (TermArg::p, SimpleName::p).cut())
            .map(|(arg, name)| Self { arg, name })
            .parse(input, alloc)
    }
}

pub struct DefDecrement<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefDecrement<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let decrement_op = item(0x76);
        preceded(decrement_op, SuperName::p.cut())
            .map(|name| Self { name })
            .parse(input, alloc)
    }
}

pub struct DefDerefOf<A: Allocator> {
    obj_ref: TermArg<A>,
}

impl<A: Allocator + Clone> DefDerefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let decrement_op = item(0x83);
        let obj_ref = TermArg::p; // => ObjRef | String
        preceded(decrement_op, obj_ref.cut())
            .map(|obj_ref| Self { obj_ref })
            .parse(input, alloc)
    }
}

pub struct DefDivide<A: Allocator> {
    dividend: TermArg<A>,
    divisor: TermArg<A>,
    remainder: Target<A>,
    quotient: Target<A>,
}

impl<A: Allocator + Clone> DefDivide<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let divide_op = item(0x78);
        let dividend = TermArg::p; // => Integer
        let divisor = TermArg::p; // => Integer
        preceded(divide_op, (dividend, divisor, Target::p, Target::p).cut())
            .map(|(dividend, divisor, remainder, quotient)| Self {
                dividend,
                divisor,
                remainder,
                quotient,
            })
            .parse(input, alloc)
    }
}

pub struct DefFindSetLeftBit<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefFindSetLeftBit<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let find_set_left_bit_op = item(0x81);
        preceded(find_set_left_bit_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub struct DefFindSetRightBit<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefFindSetRightBit<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let find_set_right_bit_op = item(0x82);
        preceded(find_set_right_bit_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub struct DefFromBcd<A: Allocator> {
    value: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefFromBcd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let from_bcd_op = (ExtOpPrefix::p, item(0x28));
        let bcd_value = TermArg::p; // => Integer
        preceded(from_bcd_op, (bcd_value, Target::p).cut())
            .map(|(value, target)| Self { value, target })
            .parse(input, alloc)
    }
}

pub struct DefIncrement<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefIncrement<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let increment_op = item(0x75);
        preceded(increment_op, SuperName::p.cut())
            .map(|name| Self { name })
            .parse(input, alloc)
    }
}

pub struct DefIndex<A: Allocator> {
    buf_pkg_str_obj: TermArg<A>,
    index_value: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefIndex<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let index_op = item(0x88);
        let buf_pkg_str_obj = TermArg::p; // => Buffer | Pkg | String
        let index_value = TermArg::p; // => Integer
        preceded(index_op, (buf_pkg_str_obj, index_value, Target::p).cut())
            .map(|(buf_pkg_str_obj, index_value, target)| Self {
                buf_pkg_str_obj,
                index_value,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefLAnd<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLAnd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let land_op = item(0x90);
        preceded(land_op, (Operand::p, Operand::p).cut())
            .map(|(left, right)| Self { left, right })
            .parse(input, alloc)
    }
}

pub struct DefLEqual<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLEqual<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(lequal_op, (Operand::p, Operand::p).cut())
            .map(|(left, right)| Self { left, right })
            .parse(input, alloc)
    }
}

fn lequal_op<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, (), E> {
    item(0x93).map(|_| ()).parse(input, alloc)
}

pub struct DefLGreater<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLGreater<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(lgreater_op, (Operand::p, Operand::p).cut())
            .map(|(left, right)| Self { left, right })
            .parse(input, alloc)
    }
}

fn lgreater_op<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, (), E> {
    item(0x94).map(|_| ()).parse(input, alloc)
}

pub struct DefLGreaterEqual<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLGreaterEqual<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let lgreater_equal_op = (lnot_op, lless_op);
        preceded(lgreater_equal_op, (Operand::p, Operand::p).cut())
            .map(|(left, right)| Self { left, right })
            .parse(input, alloc)
    }
}

pub struct DefLLess<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLLess<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(lless_op, (Operand::p, Operand::p).cut())
            .map(|(left, right)| Self { left, right })
            .parse(input, alloc)
    }
}

fn lless_op<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, (), E> {
    item(0x95).map(|_| ()).parse(input, alloc)
}

pub struct DefLLessEqual<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLLessEqual<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let lless_equal_op = (lnot_op, lgreater_op);
        preceded(lless_equal_op, (Operand::p, Operand::p).cut())
            .map(|(left, right)| Self { left, right })
            .parse(input, alloc)
    }
}

pub struct DefLNot<A: Allocator> {
    operand: Operand<A>,
}

impl<A: Allocator + Clone> DefLNot<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        preceded(lnot_op, Operand::p.cut())
            .map(|operand| Self { operand })
            .parse(input, alloc)
    }
}

fn lnot_op<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
    input: I,
    alloc: A,
) -> ParseResult<I, (), E> {
    item(0x92).map(|_| ()).parse(input, alloc)
}

pub struct DefLNotEqual<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLNotEqual<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let lnot_equal_op = (lnot_op, lequal_op);
        preceded(lnot_equal_op, (Operand::p, Operand::p).cut())
            .map(|(left, right)| Self { left, right })
            .parse(input, alloc)
    }
}

pub struct DefLoad<A: Allocator> {
    name: NameString<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefLoad<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let load_op = (ExtOpPrefix::p, item(0x20));
        preceded(load_op, (NameString::p, Target::p).cut())
            .map(|(name, target)| Self { name, target })
            .parse(input, alloc)
    }
}

pub struct DefLoadTable<A: Allocator> {
    arg1: TermArg<A>,
    arg2: TermArg<A>,
    arg3: TermArg<A>,
    arg4: TermArg<A>,
    arg5: TermArg<A>,
    arg6: TermArg<A>,
}

impl<A: Allocator + Clone> DefLoadTable<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let load_table_op = (ExtOpPrefix::p, item(0x1f));
        preceded(
            load_table_op,
            (
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
            )
                .cut(),
        )
        .map(|(arg1, arg2, arg3, arg4, arg5, arg6)| Self {
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        })
        .parse(input, alloc)
    }
}

pub struct DefLOr<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLOr<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let lor_op = item(0x91);
        preceded(lor_op, (Operand::p, Operand::p).cut())
            .map(|(left, right)| Self { left, right })
            .parse(input, alloc)
    }
}

pub struct DefMatch<A: Allocator> {
    search_pkg: TermArg<A>,
    left_match_opcode: u8,
    left: Operand<A>,
    right_match_opcode: u8,
    right: Operand<A>,
    start_index: TermArg<A>,
}

impl<A: Allocator + Clone> DefMatch<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let match_op = item(0x89);
        preceded(
            match_op,
            (
                TermArg::p,
                byte_data,
                Operand::p,
                byte_data,
                Operand::p,
                TermArg::p,
            )
                .cut(),
        )
        .map(
            |(search_pkg, left_match_opcode, left, right_match_opcode, right, start_index)| Self {
                search_pkg,
                left_match_opcode,
                left,
                right_match_opcode,
                right,
                start_index,
            },
        )
        .parse(input, alloc)
    }
}

pub struct DefMid<A: Allocator> {
    mid_obj: TermArg<A>,
    term1: TermArg<A>,
    term2: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefMid<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let mid_op = item(0x9e);
        let mid_obj = TermArg::p; // => Buffer | String
        preceded(mid_op, (mid_obj, TermArg::p, TermArg::p, Target::p).cut())
            .map(|(mid_obj, term1, term2, target)| Self {
                mid_obj,
                term1,
                term2,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefMod<A: Allocator> {
    dividend: TermArg<A>,
    divisor: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefMod<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let mod_op = item(0x85);
        let dividend = TermArg::p; // => Integer
        let divisor = TermArg::p; // => Integer
        preceded(mod_op, (dividend, divisor, Target::p).cut())
            .map(|(dividend, divisor, target)| Self {
                dividend,
                divisor,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefMultiply<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefMultiply<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let multiply_op = item(0x77);
        preceded(multiply_op, (Operand::p, Operand::p, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefNAnd<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefNAnd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let nand_op = item(0x7c);
        preceded(nand_op, (Operand::p, Operand::p, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefNOr<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefNOr<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let multiply_op = item(0x7e);
        preceded(multiply_op, (Operand::p, Operand::p, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefNot<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefNot<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let multiply_op = item(0x80);
        preceded(multiply_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub enum DefObjType<A: Allocator> {
    SimpleName(SimpleName<A>),
    DebugObj(DebugObj),
    DefRefOf(DefRefOf<A>),
    DefDerefOf(DefDerefOf<A>),
    DefIndex(DefIndex<A>),
}

impl<A: Allocator + Clone> DefObjType<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let obj_type_op = item(0x8e);
        preceded(
            obj_type_op,
            (
                SimpleName::p.map(Self::SimpleName),
                DebugObj::p.map(Self::DebugObj),
                DefRefOf::p.map(Self::DefRefOf),
                DefDerefOf::p.map(Self::DefDerefOf),
                DefIndex::p.map(Self::DefIndex),
            )
                .alt()
                .cut(),
        )
        .parse(input, alloc)
    }
}

pub struct DefOr<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefOr<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let or_op = item(0x7d);
        preceded(or_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub struct DefPkg<A: Allocator> {
    len: usize,
    elements: PkgElementList<A>,
}

impl<A: Allocator + Clone> DefPkg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let pkg_op = item(0x12);
        let num_elements = byte_data;
        preceded(pkg_op, pkg((num_elements, PkgElementList::p)))
            .map(|(len, elements)| Self {
                len: len as usize,
                elements,
            })
            .parse(input, alloc)
    }
}

pub struct DefVarPkg<A: Allocator> {
    len: TermArg<A>,
    elements: PkgElementList<A>,
}

impl<A: Allocator + Clone> DefVarPkg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let var_pkg_op = item(0x13);
        let var_num_elements = TermArg::p; // => Integer
        preceded(var_pkg_op, pkg((var_num_elements, PkgElementList::p)))
            .map(|(len, elements)| Self { len, elements })
            .parse(input, alloc)
    }
}

pub struct PkgElementList<A: Allocator>(Vec<PkgElement<A>, A>);

impl<A: Allocator + Clone> PkgElementList<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        many(PkgElement::p).map(Self).parse(input, alloc)
    }
}

pub enum PkgElement<A: Allocator> {
    DataRefObj(DataRefObj<A>),
    NameString(NameString<A>),
}

impl<A: Allocator + Clone> PkgElement<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            DataRefObj::p.map(Self::DataRefObj),
            NameString::p.map(Self::NameString),
        )
            .alt()
            .parse(input, alloc)
    }
}

pub struct DefRefOf<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefRefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let ref_of_op = item(0x71);
        preceded(ref_of_op, SuperName::p.cut())
            .map(|name| Self { name })
            .parse(input, alloc)
    }
}

pub struct DefShiftLeft<A: Allocator> {
    operand: Operand<A>,
    shift_count: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefShiftLeft<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let shift_left_op = item(0x79);
        let shift_count = TermArg::p; // => Integer
        preceded(shift_left_op, (Operand::p, shift_count, Target::p).cut())
            .map(|(operand, shift_count, target)| Self {
                operand,
                shift_count,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefShiftRight<A: Allocator> {
    operand: Operand<A>,
    shift_count: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefShiftRight<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let shift_left_op = item(0x7a);
        let shift_count = TermArg::p; // => Integer
        preceded(shift_left_op, (Operand::p, shift_count, Target::p).cut())
            .map(|(operand, shift_count, target)| Self {
                operand,
                shift_count,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefSizeOf<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefSizeOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let size_of_op = item(0x87);
        preceded(size_of_op, SuperName::p.cut())
            .map(|name| Self { name })
            .parse(input, alloc)
    }
}

pub struct DefStore<A: Allocator> {
    term: TermArg<A>,
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefStore<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let store_op = item(0x70);
        preceded(store_op, (TermArg::p, SuperName::p).cut())
            .map(|(term, name)| Self { term, name })
            .parse(input, alloc)
    }
}

pub struct DefSubtract<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefSubtract<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let subtract_op = item(0x74);
        preceded(subtract_op, (Operand::p, Operand::p, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefTimer;

impl DefTimer {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let timer_op = (item(0x5b), item(0x33));
        timer_op.map(|_| Self).parse(input, alloc)
    }
}

pub struct DefToBcd<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToBcd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let to_bcd_op = (ExtOpPrefix::p, item(0x29));
        preceded(to_bcd_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub struct DefToBuffer<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToBuffer<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let to_buffer_op = item(0x96);
        preceded(to_buffer_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub struct DefToDecimalString<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToDecimalString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let to_decimal_string_op = item(0x97);
        preceded(to_decimal_string_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub struct DefToHexString<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToHexString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let to_hex_string_op = item(0x98);
        preceded(to_hex_string_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub struct DefToInteger<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToInteger<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let to_integer_op = item(0x99);
        preceded(to_integer_op, (Operand::p, Target::p).cut())
            .map(|(operand, target)| Self { operand, target })
            .parse(input, alloc)
    }
}

pub struct DefToString<A: Allocator> {
    arg1: TermArg<A>,
    length_arg: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let to_string_op = item(0x9c);
        let length_arg = TermArg::p; // => Integer
        preceded(to_string_op, (TermArg::p, length_arg, Target::p).cut())
            .map(|(arg1, length_arg, target)| Self {
                arg1,
                length_arg,
                target,
            })
            .parse(input, alloc)
    }
}

pub struct DefWait<A: Allocator> {
    event: EventObj<A>,
    operand: Operand<A>,
}

impl<A: Allocator + Clone> DefWait<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let wait_op = (ExtOpPrefix::p, item(0x25));
        preceded(wait_op, (EventObj::p, Operand::p).cut())
            .map(|(event, operand)| Self { event, operand })
            .parse(input, alloc)
    }
}

pub struct DefXOr<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefXOr<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let xor_op = item(0x7f);
        preceded(xor_op, (Operand::p, Operand::p, Target::p).cut())
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .parse(input, alloc)
    }
}
