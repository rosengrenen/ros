use super::statement::{EventObj, MutexObj};
use crate::aml::{
    aml::{
        data::{byte_data, ByteList, DataRefObj},
        misc::DebugObj,
        name::{NameString, SimpleName, SuperName, Target},
        term::{MethodInvocation, TermArg},
    },
    ops::{
        AcquireOp, AddOp, AndOp, BufferOp, ConcatOp, ConcatResOp, CondRefOfOp, CopyObjOp,
        DecrementOp, DerefOfOp, DivideOp, FindSetLeftBitOp, FindSetRightBitOp, FromBcdOp,
        IncrementOp, IndexOp, LAndOp, LEqualOp, LGreaterEqualOp, LGreaterOp, LLessEqualOp, LLessOp,
        LNotEqualOp, LNotOp, LOrOp, LoadOp, LoadTableOp, MatchOp, MidOp, ModOp, MultiplyOp, NandOp,
        NorOp, NotOp, ObjTypeOp, OrOp, PkgOp, RefOfOp, ShiftLeftOp, ShiftRightOp, SizeOfOp,
        StoreOp, SubtractOp, TimerOp, ToBcdOp, ToBufferOp, ToDecimalStringOp, ToHexStringOp,
        ToIntegerOp, ToStringOp, VarPkgOp, WaitOp, XorOp,
    },
    pkg, prefixed, Context,
};
use alloc::vec::Vec;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    multi::many::many,
    parser::Parser,
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
        context: &mut Context,
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
            .add_context("ExprOpcode")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            DefRefOf::p.map(Self::DefRefOf),
            DefDerefOf::p.map(Self::DefDerefOf),
            DefIndex::p.map(Self::DefIndex),
            MethodInvocation::p.map(Self::UserTermObj),
        )
            .alt()
            .add_context("RefTypeOpcode")
            .parse(input, context, alloc)
    }
}

pub struct DefAcquire<A: Allocator> {
    mutex: MutexObj<A>,
    timeout: Timeout,
}

impl<A: Allocator + Clone> DefAcquire<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(AcquireOp::p, (MutexObj::p, Timeout::p))
            .map(|(mutex, timeout)| Self { mutex, timeout })
            .add_context("DefAcquire")
            .parse(input, context, alloc)
    }
}

pub struct Timeout(u8);

impl Timeout {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        byte_data
            .map(Self)
            .add_context("Timeout")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(AddOp::p, (Operand::p, Operand::p, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefAdd")
            .parse(input, context, alloc)
    }
}

pub struct Operand<A: Allocator>(TermArg<A>);

impl<A: Allocator + Clone> Operand<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        TermArg::p
            .map(Self)
            .add_context("Operand")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(AndOp::p, (Operand::p, Operand::p, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefAnd")
            .parse(input, context, alloc)
    }
}

pub struct DefBuffer<A: Allocator> {
    len: TermArg<A>,
    bytes: ByteList<A>,
}

impl<A: Allocator + Clone> DefBuffer<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let buffer_size = TermArg::p; // => Integer
        prefixed(BufferOp::p, pkg((buffer_size, ByteList::p)))
            .map(|(len, bytes)| Self { len, bytes })
            .add_context("DefBuffer")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let data = TermArg::p; // => ComputationalData
        prefixed(ConcatOp::p, (data, data, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefConcat")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let buf_data = TermArg::p; // => Buffer
        prefixed(ConcatResOp::p, (buf_data, buf_data, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefConcatRes")
            .parse(input, context, alloc)
    }
}

pub struct DefCondRefOf<A: Allocator> {
    name: SuperName<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefCondRefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(CondRefOfOp::p, (SuperName::p, Target::p))
            .map(|(name, target)| Self { name, target })
            .add_context("DefCondRefOf")
            .parse(input, context, alloc)
    }
}

pub struct DefCopyObj<A: Allocator> {
    arg: TermArg<A>,
    name: SimpleName<A>,
}

impl<A: Allocator + Clone> DefCopyObj<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(CopyObjOp::p, (TermArg::p, SimpleName::p))
            .map(|(arg, name)| Self { arg, name })
            .add_context("DefCopyObj")
            .parse(input, context, alloc)
    }
}

pub struct DefDecrement<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefDecrement<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(DecrementOp::p, SuperName::p)
            .map(|name| Self { name })
            .add_context("DefDecrement")
            .parse(input, context, alloc)
    }
}

pub struct DefDerefOf<A: Allocator> {
    obj_ref: TermArg<A>,
}

impl<A: Allocator + Clone> DefDerefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let obj_ref = TermArg::p; // => ObjRef | String
        prefixed(DerefOfOp::p, obj_ref)
            .map(|obj_ref| Self { obj_ref })
            .add_context("DefDerefOf")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let dividend = TermArg::p; // => Integer
        let divisor = TermArg::p; // => Integer
        prefixed(DivideOp::p, (dividend, divisor, Target::p, Target::p))
            .map(|(dividend, divisor, remainder, quotient)| Self {
                dividend,
                divisor,
                remainder,
                quotient,
            })
            .add_context("DefDivide")
            .parse(input, context, alloc)
    }
}

pub struct DefFindSetLeftBit<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefFindSetLeftBit<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(FindSetLeftBitOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefFindSetLeftBit")
            .parse(input, context, alloc)
    }
}

pub struct DefFindSetRightBit<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefFindSetRightBit<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(FindSetRightBitOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefFindSetRightBit")
            .parse(input, context, alloc)
    }
}

pub struct DefFromBcd<A: Allocator> {
    value: TermArg<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefFromBcd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let bcd_value = TermArg::p; // => Integer
        prefixed(FromBcdOp::p, (bcd_value, Target::p))
            .map(|(value, target)| Self { value, target })
            .add_context("DefFromBcd")
            .parse(input, context, alloc)
    }
}

pub struct DefIncrement<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefIncrement<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(IncrementOp::p, SuperName::p)
            .map(|name| Self { name })
            .add_context("DefIncrement")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let buf_pkg_str_obj = TermArg::p; // => Buffer | Pkg | String
        let index_value = TermArg::p; // => Integer
        prefixed(IndexOp::p, (buf_pkg_str_obj, index_value, Target::p))
            .map(|(buf_pkg_str_obj, index_value, target)| Self {
                buf_pkg_str_obj,
                index_value,
                target,
            })
            .add_context("DefIndex")
            .parse(input, context, alloc)
    }
}

pub struct DefLAnd<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLAnd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LAndOp::p, (Operand::p, Operand::p))
            .map(|(left, right)| Self { left, right })
            .add_context("DefLAnd")
            .parse(input, context, alloc)
    }
}

pub struct DefLEqual<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLEqual<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LEqualOp::p, (Operand::p, Operand::p))
            .map(|(left, right)| Self { left, right })
            .add_context("DefLEqual")
            .parse(input, context, alloc)
    }
}

pub struct DefLGreater<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLGreater<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LGreaterOp::p, (Operand::p, Operand::p))
            .map(|(left, right)| Self { left, right })
            .add_context("DefLGreater")
            .parse(input, context, alloc)
    }
}

pub struct DefLGreaterEqual<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLGreaterEqual<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LGreaterEqualOp::p, (Operand::p, Operand::p))
            .map(|(left, right)| Self { left, right })
            .add_context("DefLGreaterEqual")
            .parse(input, context, alloc)
    }
}

pub struct DefLLess<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLLess<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LLessOp::p, (Operand::p, Operand::p))
            .map(|(left, right)| Self { left, right })
            .add_context("DefLLess")
            .parse(input, context, alloc)
    }
}

pub struct DefLLessEqual<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLLessEqual<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LLessEqualOp::p, (Operand::p, Operand::p))
            .map(|(left, right)| Self { left, right })
            .add_context("DefLLessEqual")
            .parse(input, context, alloc)
    }
}

pub struct DefLNot<A: Allocator> {
    operand: Operand<A>,
}

impl<A: Allocator + Clone> DefLNot<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LNotOp::p, Operand::p)
            .map(|operand| Self { operand })
            .add_context("DefLNot")
            .parse(input, context, alloc)
    }
}

pub struct DefLNotEqual<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLNotEqual<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LNotEqualOp::p, (Operand::p, Operand::p))
            .map(|(left, right)| Self { left, right })
            .add_context("DefLNotEqual")
            .parse(input, context, alloc)
    }
}

pub struct DefLoad<A: Allocator> {
    name: NameString<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefLoad<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LoadOp::p, (NameString::p, Target::p))
            .map(|(name, target)| Self { name, target })
            .add_context("DefLoad")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            LoadTableOp::p,
            (
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
                TermArg::p,
            ),
        )
        .map(|(arg1, arg2, arg3, arg4, arg5, arg6)| Self {
            arg1,
            arg2,
            arg3,
            arg4,
            arg5,
            arg6,
        })
        .add_context("DefLoadTable")
        .parse(input, context, alloc)
    }
}

pub struct DefLOr<A: Allocator> {
    left: Operand<A>,
    right: Operand<A>,
}

impl<A: Allocator + Clone> DefLOr<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(LOrOp::p, (Operand::p, Operand::p))
            .map(|(left, right)| Self { left, right })
            .add_context("DefLOr")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            MatchOp::p,
            (
                TermArg::p,
                byte_data,
                Operand::p,
                byte_data,
                Operand::p,
                TermArg::p,
            ),
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
        .add_context("DefMatch")
        .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let mid_obj = TermArg::p; // => Buffer | String
        prefixed(MidOp::p, (mid_obj, TermArg::p, TermArg::p, Target::p))
            .map(|(mid_obj, term1, term2, target)| Self {
                mid_obj,
                term1,
                term2,
                target,
            })
            .add_context("DefMid")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let dividend = TermArg::p; // => Integer
        let divisor = TermArg::p; // => Integer
        prefixed(ModOp::p, (dividend, divisor, Target::p))
            .map(|(dividend, divisor, target)| Self {
                dividend,
                divisor,
                target,
            })
            .add_context("DefMod")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(MultiplyOp::p, (Operand::p, Operand::p, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefMultiply")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(NandOp::p, (Operand::p, Operand::p, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefNAnd")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(NorOp::p, (Operand::p, Operand::p, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefNOr")
            .parse(input, context, alloc)
    }
}

pub struct DefNot<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefNot<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(NotOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefNot")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(
            ObjTypeOp::p,
            (
                SimpleName::p.map(Self::SimpleName),
                DebugObj::p.map(Self::DebugObj),
                DefRefOf::p.map(Self::DefRefOf),
                DefDerefOf::p.map(Self::DefDerefOf),
                DefIndex::p.map(Self::DefIndex),
            )
                .alt(),
        )
        .add_context("DefObjType")
        .parse(input, context, alloc)
    }
}

pub struct DefOr<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefOr<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(OrOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefOr")
            .parse(input, context, alloc)
    }
}

pub struct DefPkg<A: Allocator> {
    len: usize,
    elements: PkgElementList<A>,
}

impl<A: Allocator + Clone> DefPkg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let num_elements = byte_data;
        prefixed(PkgOp::p, pkg((num_elements, PkgElementList::p)))
            .map(|(len, elements)| Self {
                len: len as usize,
                elements,
            })
            .add_context("DefPkg")
            .parse(input, context, alloc)
    }
}

pub struct DefVarPkg<A: Allocator> {
    len: TermArg<A>,
    elements: PkgElementList<A>,
}

impl<A: Allocator + Clone> DefVarPkg<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let var_num_elements = TermArg::p; // => Integer
        prefixed(VarPkgOp::p, pkg((var_num_elements, PkgElementList::p)))
            .map(|(len, elements)| Self { len, elements })
            .add_context("DefVarPkg")
            .parse(input, context, alloc)
    }
}

pub struct PkgElementList<A: Allocator>(Vec<PkgElement<A>, A>);

impl<A: Allocator + Clone> PkgElementList<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        many(PkgElement::p)
            .map(Self)
            .add_context("PkgElementList")
            .parse(input, context, alloc)
    }
}

pub enum PkgElement<A: Allocator> {
    DataRefObj(DataRefObj<A>),
    NameString(NameString<A>),
}

impl<A: Allocator + Clone> PkgElement<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            DataRefObj::p.map(Self::DataRefObj),
            NameString::p.map(Self::NameString),
        )
            .alt()
            .add_context("PkgElement")
            .parse(input, context, alloc)
    }
}

pub struct DefRefOf<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefRefOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(RefOfOp::p, SuperName::p)
            .map(|name| Self { name })
            .add_context("DefRefOf")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let shift_count = TermArg::p; // => Integer
        prefixed(ShiftLeftOp::p, (Operand::p, shift_count, Target::p))
            .map(|(operand, shift_count, target)| Self {
                operand,
                shift_count,
                target,
            })
            .add_context("DefShiftLeft")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let shift_count = TermArg::p; // => Integer
        prefixed(ShiftRightOp::p, (Operand::p, shift_count, Target::p))
            .map(|(operand, shift_count, target)| Self {
                operand,
                shift_count,
                target,
            })
            .add_context("DefShiftRight")
            .parse(input, context, alloc)
    }
}

pub struct DefSizeOf<A: Allocator> {
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefSizeOf<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(SizeOfOp::p, SuperName::p)
            .map(|name| Self { name })
            .add_context("DefSizeOf")
            .parse(input, context, alloc)
    }
}

pub struct DefStore<A: Allocator> {
    term: TermArg<A>,
    name: SuperName<A>,
}

impl<A: Allocator + Clone> DefStore<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(StoreOp::p, (TermArg::p, SuperName::p))
            .map(|(term, name)| Self { term, name })
            .add_context("DefStore")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(SubtractOp::p, (Operand::p, Operand::p, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefSubtract")
            .parse(input, context, alloc)
    }
}

pub struct DefTimer;

impl DefTimer {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>, A: Allocator + Clone>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        TimerOp::p
            .map(|_| Self)
            .add_context("DefTimer")
            .parse(input, context, alloc)
    }
}

pub struct DefToBcd<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToBcd<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ToBcdOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefToBcd")
            .parse(input, context, alloc)
    }
}

pub struct DefToBuffer<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToBuffer<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ToBufferOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefToBuffer")
            .parse(input, context, alloc)
    }
}

pub struct DefToDecimalString<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToDecimalString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ToDecimalStringOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefToDecimalString")
            .parse(input, context, alloc)
    }
}

pub struct DefToHexString<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToHexString<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ToHexStringOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefToHexString")
            .parse(input, context, alloc)
    }
}

pub struct DefToInteger<A: Allocator> {
    operand: Operand<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> DefToInteger<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(ToIntegerOp::p, (Operand::p, Target::p))
            .map(|(operand, target)| Self { operand, target })
            .add_context("DefToInteger")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        let length_arg = TermArg::p; // => Integer
        prefixed(ToStringOp::p, (TermArg::p, length_arg, Target::p))
            .map(|(arg1, length_arg, target)| Self {
                arg1,
                length_arg,
                target,
            })
            .add_context("DefToString")
            .parse(input, context, alloc)
    }
}

pub struct DefWait<A: Allocator> {
    event: EventObj<A>,
    operand: Operand<A>,
}

impl<A: Allocator + Clone> DefWait<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(WaitOp::p, (EventObj::p, Operand::p))
            .map(|(event, operand)| Self { event, operand })
            .add_context("DefWait")
            .parse(input, context, alloc)
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
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        prefixed(XorOp::p, (Operand::p, Operand::p, Target::p))
            .map(|(left, right, target)| Self {
                left,
                right,
                target,
            })
            .add_context("DefXOr")
            .parse(input, context, alloc)
    }
}
