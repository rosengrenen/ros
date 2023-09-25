pub mod acquire;
pub mod bitwise;
pub mod buffer;
pub mod concat;
pub mod concat_res;
pub mod cond_ref_of;
pub mod convert_fn;
pub mod copy_obj;
pub mod deref_of;
pub mod find_set_left_bit;
pub mod find_set_right_bit;
pub mod index;
pub mod integer;
pub mod load_table;
pub mod logical;
pub mod match_pkg;
pub mod mid;
pub mod module;
pub mod obj_type;
pub mod pkg;
pub mod ref_of;
pub mod size_of;
pub mod store;
pub mod timer;
pub mod var_pkg;
pub mod wait;

use self::{
    acquire::Acquire, bitwise::Bitwise, buffer::Buffer, concat::Concat, concat_res::ConcatRes,
    cond_ref_of::CondRefOf, convert_fn::ConvertFn, copy_obj::CopyObj, deref_of::DerefOf,
    find_set_left_bit::FindSetLeftBit, find_set_right_bit::FindSetRightBit, index::Index,
    integer::Integer, load_table::LoadTable, logical::Logical, match_pkg::Match, mid::Mid,
    module::Mod, obj_type::ObjType, pkg::Pkg, ref_of::RefOf, size_of::SizeOf, store::Store,
    timer::Timer, var_pkg::VarPkg, wait::Wait,
};
use super::TermArg;
use crate::aml::{
    aml::{data::DataRefObj, name::NameString, term::MethodInvocation},
    Context,
};
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
};

#[derive(Debug)]
pub enum Expr<A: Allocator> {
    Acquire(Acquire<A>),
    Bitwise(Bitwise<A>),
    Buffer(Buffer<A>),
    Concat(Concat<A>),
    ConcatRes(ConcatRes<A>),
    CondRefOf(CondRefOf<A>),
    ConvertFn(ConvertFn<A>),
    CopyObj(CopyObj<A>),
    DerefOf(DerefOf<A>),
    FindSetLeftBit(FindSetLeftBit<A>),
    FindSetRightBit(FindSetRightBit<A>),
    Index(Index<A>),
    Integer(Integer<A>),
    Logical(Logical<A>),
    Mid(Mid<A>),
    LoadTable(LoadTable<A>),
    Match(Match<A>),
    Mod(Mod<A>),
    ObjType(ObjType<A>),
    Pkg(Pkg<A>),
    RefOf(RefOf<A>),
    SizeOf(SizeOf<A>),
    Store(Store<A>),
    Timer(Timer),
    VarPkg(VarPkg<A>),
    Wait(Wait<A>),
    MethodInvocation(MethodInvocation<A>),
}

impl<A: Allocator + Clone> Expr<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            (
                Acquire::p.map(Self::Acquire),
                Bitwise::p.map(Self::Bitwise),
                Buffer::p.map(Self::Buffer),
                Concat::p.map(Self::Concat),
                ConcatRes::p.map(Self::ConcatRes),
                CondRefOf::p.map(Self::CondRefOf),
                ConvertFn::p.map(Self::ConvertFn),
                CopyObj::p.map(Self::CopyObj),
                DerefOf::p.map(Self::DerefOf),
                FindSetLeftBit::p.map(Self::FindSetLeftBit),
                FindSetRightBit::p.map(Self::FindSetRightBit),
                Index::p.map(Self::Index),
                Integer::p.map(Self::Integer),
                Logical::p.map(Self::Logical),
                Mid::p.map(Self::Mid),
                LoadTable::p.map(Self::LoadTable),
            )
                .alt(),
            (
                Match::p.map(Self::Match),
                Mod::p.map(Self::Mod),
                ObjType::p.map(Self::ObjType),
                Pkg::p.map(Self::Pkg),
                VarPkg::p.map(Self::VarPkg),
                RefOf::p.map(Self::RefOf),
                SizeOf::p.map(Self::SizeOf),
                Store::p.map(Self::Store),
                Timer::p.map(Self::Timer),
                Wait::p.map(Self::Wait),
                MethodInvocation::p.map(Self::MethodInvocation),
            )
                .alt(),
        )
            .alt()
            .add_context("Expr")
            .parse(input, context, alloc)
    }
}

#[derive(Debug)]
pub enum RefTypeOpcode<A: Allocator> {
    RefOf(RefOf<A>),
    DerefOf(DerefOf<A>),
    Index(Index<A>),
    UserTermObj(MethodInvocation<A>),
}

impl<A: Allocator + Clone> RefTypeOpcode<A> {
    pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
        input: I,
        context: &mut Context,
        alloc: A,
    ) -> ParseResult<I, Self, E> {
        (
            RefOf::p.map(Self::RefOf),
            DerefOf::p.map(Self::DerefOf),
            Index::p.map(Self::Index),
            MethodInvocation::p.map(Self::UserTermObj),
        )
            .alt()
            .add_context("RefTypeOpcode")
            .parse(input, context, alloc)
    }
}

// #[derive(Debug)] pub struct Load<A: Allocator> {
//     name: NameString<A>,
//     target: Target<A>,
// }

// impl<A: Allocator + Clone> Load<A> {
//     pub fn p<I: Input<Item = u8>, E: ParseError<I, A>>(
//         input: I,
//         context: &mut Context,
//         alloc: A,
//     ) -> ParseResult<I, Self, E> {
//         prefixed(LoadOp::p, (NameString::p, Target::p))
//             .map(|(name, target)| Self { name, target })
//             .add_context("Load")
//             .parse(input, context, alloc)
//     }
// }

#[derive(Debug)]
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
