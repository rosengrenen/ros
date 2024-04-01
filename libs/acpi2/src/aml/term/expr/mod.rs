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

use alloc::boxed::Box;

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
    parser::{fail, Input, ParseResult, ParserError},
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
        match Acquire::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Acquire(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Bitwise::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Bitwise(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Buffer::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Buffer(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Concat::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Concat(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ConcatRes::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ConcatRes(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match CondRefOf::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::CondRefOf(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ConvertFn::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ConvertFn(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match CopyObj::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::CopyObj(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DerefOf::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::DerefOf(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match FindSetLeftBit::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::FindSetLeftBit(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match FindSetRightBit::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::FindSetRightBit(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Index::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Index(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Integer::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Integer(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Load::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Load(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match LoadTable::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::LoadTable(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Logical::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Logical(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Match::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Match(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Mid::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Mid(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Mod::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Mod(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match ObjType::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::ObjType(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Pkg::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Pkg(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match RefOf::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::RefOf(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match SizeOf::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::SizeOf(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Store::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Store(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Timer::parse(input) {
            Ok((value, input)) => return Ok((Self::Timer(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match VarPkg::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::VarPkg(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Wait::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Wait(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = SymbolAccess::parse(input, context, alloc)?;
        Ok((Self::SymbolAccess(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for Expr<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Acquire(arg0) => f.debug_tuple("Acquire").field(arg0).finish(),
            Self::Bitwise(arg0) => f.debug_tuple("Bitwise").field(arg0).finish(),
            Self::Buffer(arg0) => f.debug_tuple("Buffer").field(arg0).finish(),
            Self::Concat(arg0) => f.debug_tuple("Concat").field(arg0).finish(),
            Self::ConcatRes(arg0) => f.debug_tuple("ConcatRes").field(arg0).finish(),
            Self::CondRefOf(arg0) => f.debug_tuple("CondRefOf").field(arg0).finish(),
            Self::ConvertFn(arg0) => f.debug_tuple("ConvertFn").field(arg0).finish(),
            Self::CopyObj(arg0) => f.debug_tuple("CopyObj").field(arg0).finish(),
            Self::DerefOf(arg0) => f.debug_tuple("DerefOf").field(arg0).finish(),
            Self::FindSetLeftBit(arg0) => f.debug_tuple("FindSetLeftBit").field(arg0).finish(),
            Self::FindSetRightBit(arg0) => f.debug_tuple("FindSetRightBit").field(arg0).finish(),
            Self::Index(arg0) => f.debug_tuple("Index").field(arg0).finish(),
            Self::Integer(arg0) => f.debug_tuple("Integer").field(arg0).finish(),
            Self::Load(arg0) => f.debug_tuple("Load").field(arg0).finish(),
            Self::LoadTable(arg0) => f.debug_tuple("LoadTable").field(arg0).finish(),
            Self::Logical(arg0) => f.debug_tuple("Logical").field(arg0).finish(),
            Self::Match(arg0) => f.debug_tuple("Match").field(arg0).finish(),
            Self::Mid(arg0) => f.debug_tuple("Mid").field(arg0).finish(),
            Self::Mod(arg0) => f.debug_tuple("Mod").field(arg0).finish(),
            Self::ObjType(arg0) => f.debug_tuple("ObjType").field(arg0).finish(),
            Self::Pkg(arg0) => f.debug_tuple("Pkg").field(arg0).finish(),
            Self::RefOf(arg0) => f.debug_tuple("RefOf").field(arg0).finish(),
            Self::SizeOf(arg0) => f.debug_tuple("SizeOf").field(arg0).finish(),
            Self::Store(arg0) => f.debug_tuple("Store").field(arg0).finish(),
            Self::Timer(arg0) => f.debug_tuple("Timer").field(arg0).finish(),
            Self::VarPkg(arg0) => f.debug_tuple("VarPkg").field(arg0).finish(),
            Self::Wait(arg0) => f.debug_tuple("Wait").field(arg0).finish(),
            Self::SymbolAccess(arg0) => f.debug_tuple("SymbolAccess").field(arg0).finish(),
        }
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
        match RefOf::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::RefOf(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match DerefOf::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::DerefOf(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        match Index::parse(input, context, alloc.clone()) {
            Ok((value, input)) => return Ok((Self::Index(value), input)),
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = SymbolAccess::parse(input, context, alloc)?;
        Ok((Self::SymbolAccess(value), input))
    }
}

impl<A: Allocator> core::fmt::Debug for RefTypeOpcode<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::RefOf(arg0) => f.debug_tuple("RefOf").field(arg0).finish(),
            Self::DerefOf(arg0) => f.debug_tuple("DerefOf").field(arg0).finish(),
            Self::Index(arg0) => f.debug_tuple("Index").field(arg0).finish(),
            Self::SymbolAccess(arg0) => f.debug_tuple("SymbolAccess").field(arg0).finish(),
        }
    }
}

pub struct Load<A: Allocator> {
    pub name: NameString<A>,
    pub target: Target<A>,
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

impl<A: Allocator> core::fmt::Debug for Load<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Load")
            .field("name", &self.name)
            .field("target", &self.target)
            .finish()
    }
}

pub enum PkgElement<A: Allocator> {
    DataRefObj(Box<DataRefObj<A>, A>),
    NameString(Box<NameString<A>, A>),
}

impl<A: Allocator + Clone> PkgElement<A> {
    pub fn parse<'a>(
        input: Input<'a>,
        context: &mut Context<A>,
        alloc: A,
    ) -> ParseResult<'a, Self> {
        match DataRefObj::parse(input, context, alloc.clone()) {
            Ok((value, input)) => {
                return Ok((Self::DataRefObj(Box::new(value, alloc).unwrap()), input))
            }
            Err(ParserError::Failure) => return Err(ParserError::Failure),
            Err(_) => (),
        }

        let (value, input) = NameString::parse(input, alloc.clone())?;
        Ok((Self::NameString(Box::new(value, alloc).unwrap()), input))
    }
}

impl<A: Allocator> core::fmt::Debug for PkgElement<A> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::DataRefObj(arg0) => f.debug_tuple("DataRefObj").field(arg0).finish(),
            Self::NameString(arg0) => f.debug_tuple("NameString").field(arg0).finish(),
        }
    }
}
