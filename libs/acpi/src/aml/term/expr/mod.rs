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
use super::SymbolAccess;
use crate::aml::{
    data::DataRefObj,
    name::{NameString, Target},
    ops::LoadOp,
    prefixed::prefixed,
};

parser_enum_alloc!(
    enum Expr {
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
);

parser_enum_alloc!(
    enum RefTypeOpcode {
        RefOf(RefOf<A>),
        DerefOf(DerefOf<A>),
        Index(Index<A>),
        SymbolAccess(SymbolAccess<A>),
    }
);

parser_struct_alloc!(
    struct Load {
        name: NameString<A>,
        target: Target<A>,
    },
    prefixed(LoadOp::p, (NameString::p, Target::p))
);

parser_enum_alloc!(
    enum PkgElement {
        DataRefObj(DataRefObj<A>),
        NameString(NameString<A>),
    }
);
