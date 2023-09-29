use crate::aml::{
    name::NameString, ops::DeviceOp, pkg_len::pkg, prefixed::prefixed, term::TermObj,
};
use alloc::vec::Vec;
use parser::multi::many::many;

parser_struct_alloc!(
    struct Device {
        name: NameString<A>,
        terms: Vec<TermObj<A>, A>,
    },
    prefixed(
        DeviceOp::p,
        pkg((
            NameString::p.map_context(|name, context| context.push_scope(name)),
            many(TermObj::p)
        ))
    )
    .map_context(|_, context| context.pop_scope())
);
