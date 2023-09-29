use crate::aml::{data::byte_data, name::NameString, ops::MethodOp, pkg_len::pkg, term::TermObj};
use alloc::vec::Vec;
use parser::{multi::many::many, sequence::preceded};

parser_struct_alloc!(
    struct Method {
        name: NameString<A>,
        flags: u8,
        terms: Vec<TermObj<A>, A>,
    },
    preceded(
        MethodOp::p,
        pkg((
            (NameString::p, byte_data).map_context(|(name, flags), context| {
                context.add_method(name, *flags as usize & 0x7);
                context.push_scope(name);
            }),
            many(TermObj::p)
        ))
    )
    .map(|((name, flags), terms)| (name, flags, terms))
    .map_context(|_, context| context.pop_scope())
);
