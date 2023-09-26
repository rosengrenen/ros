use crate::aml::{name::Target, ops::IndexOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct Index {
        buf_pkg_str_obj: TermArg<A>,
        index_value: TermArg<A>,
        target: Target<A>,
    },
    prefixed(IndexOp::p, (TermArg::p, TermArg::p, Target::p))
);
