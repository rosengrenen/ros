use crate::aml::{data::byte_data, ops::MatchOp, prefixed::prefixed, term::TermArg};

parser_struct_alloc!(
    struct Match {
        search_pkg: TermArg<A>,
        left_match_opcode: u8,
        left: TermArg<A>,
        right_match_opcode: u8,
        right: TermArg<A>,
        start_index: TermArg<A>,
    },
    prefixed(
        MatchOp::p,
        (
            TermArg::p,
            byte_data,
            TermArg::p,
            byte_data,
            TermArg::p,
            TermArg::p,
        ),
    )
);
