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

use core::alloc::Allocator;

use self::{
    acquire::Acquire, bitwise::Bitwise, buffer::Buffer, concat::Concat, concat_res::ConcatRes,
    cond_ref_of::CondRefOf, convert_fn::ConvertFn, copy_obj::CopyObj, deref_of::DerefOf,
    find_set_left_bit::FindSetLeftBit, find_set_right_bit::FindSetRightBit, index::Index,
    integer::Integer, load_table::LoadTable, logical::Logical, match_pkg::Match, mid::Mid,
    module::Mod, obj_type::ObjType, pkg::Pkg, ref_of::RefOf, size_of::SizeOf, store::Store,
    timer::Timer, var_pkg::VarPkg, wait::Wait,
};
use super::SymbolAccess;
use crate::aml::{
    context::Context,
    data::DataRefObj,
    name::{NameString, Target},
    ops::LoadOp,
    parser::{fail, Input, ParseResult},
};

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
    Load(Load<A>),
    LoadTable(LoadTable<A>),
    Logical(Logical<A>),
    Match(Match<A>),
    Mid(Mid<A>),
    Mod(Mod<A>),
    ObjType(ObjType<A>),
    Pkg(Pkg<A>),
    RefOf(RefOf<A>),
    SizeOf(SizeOf<A>),
    Store(Store<A>),
    Timer(Timer),
    VarPkg(VarPkg<A>),
    Wait(Wait<A>),
    SymbolAccess(SymbolAccess<A>),
}

impl<A: Allocator + Clone> Expr<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        if let Ok((value, input)) = Acquire::parse(input, context, alloc.clone()) {
            return Ok((Self::Acquire(value), input));
        }

        if let Ok((value, input)) = Bitwise::parse(input, context, alloc.clone()) {
            return Ok((Self::Bitwise(value), input));
        }

        if let Ok((value, input)) = Buffer::parse(input, context, alloc.clone()) {
            return Ok((Self::Buffer(value), input));
        }

        if let Ok((value, input)) = Concat::parse(input, context, alloc.clone()) {
            return Ok((Self::Concat(value), input));
        }

        if let Ok((value, input)) = ConcatRes::parse(input, context, alloc.clone()) {
            return Ok((Self::ConcatRes(value), input));
        }

        if let Ok((value, input)) = CondRefOf::parse(input, context, alloc.clone()) {
            return Ok((Self::CondRefOf(value), input));
        }

        if let Ok((value, input)) = ConvertFn::parse(input, context, alloc.clone()) {
            return Ok((Self::ConvertFn(value), input));
        }

        if let Ok((value, input)) = CopyObj::parse(input, context, alloc.clone()) {
            return Ok((Self::CopyObj(value), input));
        }

        if let Ok((value, input)) = DerefOf::parse(input, context, alloc.clone()) {
            return Ok((Self::DerefOf(value), input));
        }

        if let Ok((value, input)) = FindSetLeftBit::parse(input, context, alloc.clone()) {
            return Ok((Self::FindSetLeftBit(value), input));
        }

        if let Ok((value, input)) = FindSetRightBit::parse(input, context, alloc.clone()) {
            return Ok((Self::FindSetRightBit(value), input));
        }

        if let Ok((value, input)) = Index::parse(input, context, alloc.clone()) {
            return Ok((Self::Index(value), input));
        }

        if let Ok((value, input)) = Integer::parse(input, context, alloc.clone()) {
            return Ok((Self::Integer(value), input));
        }

        if let Ok((value, input)) = Load::parse(input, context, alloc.clone()) {
            return Ok((Self::Load(value), input));
        }

        if let Ok((value, input)) = LoadTable::parse(input, context, alloc.clone()) {
            return Ok((Self::LoadTable(value), input));
        }

        if let Ok((value, input)) = Logical::parse(input, context, alloc.clone()) {
            return Ok((Self::Logical(value), input));
        }

        if let Ok((value, input)) = Match::parse(input, context, alloc.clone()) {
            return Ok((Self::Match(value), input));
        }

        if let Ok((value, input)) = Mid::parse(input, context, alloc.clone()) {
            return Ok((Self::Mid(value), input));
        }

        if let Ok((value, input)) = Mod::parse(input, context, alloc.clone()) {
            return Ok((Self::Mod(value), input));
        }

        if let Ok((value, input)) = ObjType::parse(input, context, alloc.clone()) {
            return Ok((Self::ObjType(value), input));
        }

        if let Ok((value, input)) = Pkg::parse(input, context, alloc.clone()) {
            return Ok((Self::Pkg(value), input));
        }

        if let Ok((value, input)) = RefOf::parse(input, context, alloc.clone()) {
            return Ok((Self::RefOf(value), input));
        }

        if let Ok((value, input)) = SizeOf::parse(input, context, alloc.clone()) {
            return Ok((Self::SizeOf(value), input));
        }

        if let Ok((value, input)) = Store::parse(input, context, alloc.clone()) {
            return Ok((Self::Store(value), input));
        }

        if let Ok((value, input)) = Timer::parse(input) {
            return Ok((Self::Timer(value), input));
        }

        if let Ok((value, input)) = VarPkg::parse(input, context, alloc.clone()) {
            return Ok((Self::VarPkg(value), input));
        }

        if let Ok((value, input)) = Wait::parse(input, context, alloc.clone()) {
            return Ok((Self::Wait(value), input));
        }

        let (value, input) = SymbolAccess::parse(input, context, alloc)?;
        Ok((Self::SymbolAccess(value), input))
    }
}

pub enum RefTypeOpcode<A: Allocator> {
    RefOf(RefOf<A>),
    DerefOf(DerefOf<A>),
    Index(Index<A>),
    SymbolAccess(SymbolAccess<A>),
}

impl<A: Allocator + Clone> RefTypeOpcode<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        if let Ok((value, input)) = RefOf::parse(input, context, alloc.clone()) {
            return Ok((Self::RefOf(value), input));
        }

        if let Ok((value, input)) = DerefOf::parse(input, context, alloc.clone()) {
            return Ok((Self::DerefOf(value), input));
        }

        if let Ok((value, input)) = Index::parse(input, context, alloc.clone()) {
            return Ok((Self::Index(value), input));
        }

        let (value, input) = SymbolAccess::parse(input, context, alloc)?;
        Ok((Self::SymbolAccess(value), input))
    }
}

pub struct Load<A: Allocator> {
    name: NameString<A>,
    target: Target<A>,
}

impl<A: Allocator + Clone> Load<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (_, input) = LoadOp::parse(input)?;
        fail(Self::parse_inner(input, context, alloc))
    }

    fn parse_inner<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        let (name, input) = NameString::parse(input, alloc.clone())?;
        let (target, input) = Target::parse(input, context, alloc)?;
        Ok((Self { name, target }, input))
    }
}

pub enum PkgElement<A: Allocator> {
    DataRefObj(DataRefObj<A>),
    NameString(NameString<A>),
}

impl<A: Allocator + Clone> PkgElement<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        if let Ok((value, input)) = DataRefObj::parse(input, context, alloc.clone()) {
            return Ok((Self::DataRefObj(value), input));
        }

        let (value, input) = NameString::parse(input, alloc)?;
        Ok((Self::NameString(value), input))
    }
}
