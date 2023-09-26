use super::{TermArg, TermObj};
use crate::aml::{
    data::{byte_data, dword_data},
    name::SuperName,
    ops::{
        BreakOp, BreakPointOp, ContinueOp, ElseOp, FatalOp, IfOp, NoopOp, NotifyOp, ReleaseOp,
        ResetOp, ReturnOp, SignalOp, SleepOp, StallOp, WhileOp,
    },
    pkg_len::pkg,
    prefixed::prefixed,
};
use alloc::{boxed::Box, vec::Vec};
use parser::multi::many::many;

parser_enum_alloc!(
    enum Statement {
        Break(Break),
        BreakPoint(BreakPoint),
        Continue(Continue),
        Else(Else<A>),
        Fatal(Fatal<A>),
        IfElse(IfElse<A>),
        Noop(Noop),
        Notify(Notify<A>),
        Release(Release<A>),
        Reset(Reset<A>),
        Return(Return<A>),
        Signal(Signal<A>),
        Sleep(Sleep<A>),
        Stall(Stall<A>),
        While(While<A>),
    }
);

parser_struct_empty!(struct BreakPoint;, BreakPointOp::p);
parser_struct_empty!(struct Break;, BreakOp::p);
parser_struct_empty!(struct Continue;, ContinueOp::p);
parser_struct_alloc!(
    struct Fatal {
        ty: u8,
        code: u32,
        arg: TermArg<A>,
    },
    prefixed(FatalOp::p, (byte_data, dword_data, TermArg::p))
);
parser_struct_alloc!(
    struct IfElse {
        predicate: TermArg<A>,
        terms: Vec<TermObj<A>, A>,
        else_statement: Option<Else<A>>,
    },
    prefixed(IfOp::p, pkg((TermArg::p, many(TermObj::p), Else::p.opt())))
);
parser_struct_alloc!(
    struct Else {
        terms: Vec<TermObj<A>, A>,
    },
    prefixed(ElseOp::p, pkg(many(TermObj::p)))
);
parser_struct_empty!(struct Noop;, NoopOp::p);
parser_struct_alloc!(
    struct Notify {
        obj: SuperName<A>,
        value: TermArg<A>,
    },
    prefixed(NotifyOp::p, (SuperName::p, TermArg::p))
);
parser_struct_alloc!(
    struct Release {
        mutex: MutexObj<A>,
    },
    prefixed(ReleaseOp::p, MutexObj::p)
);
parser_struct_alloc!(
    struct Reset {
        event: EventObj<A>,
    },
    prefixed(ResetOp::p, EventObj::p)
);
parser_struct_alloc!(
    struct Return {
        arg: ArgObj<A>,
    },
    prefixed(ReturnOp::p, ArgObj::p)
);
parser_struct_alloc!(
    struct Signal {
        event: EventObj<A>,
    },
    prefixed(SignalOp::p, EventObj::p)
);
parser_struct_alloc!(
    struct Sleep {
        ms: TermArg<A>,
    },
    prefixed(SleepOp::p, TermArg::p)
);
parser_struct_alloc!(
    struct Stall {
        us: TermArg<A>,
    },
    prefixed(StallOp::p, TermArg::p)
);
parser_struct_alloc!(
    struct While {
        predicate: TermArg<A>,
        terms: Vec<TermObj<A>, A>,
    },
    prefixed(WhileOp::p, pkg((TermArg::p, many(TermObj::p))))
);

parser_struct_wrapper_alloc!(struct MutexObj(Box<SuperName<A>, A>);, SuperName::p.boxed());
parser_struct_wrapper_alloc!(struct EventObj(SuperName<A>);, SuperName::p);
parser_struct_wrapper_alloc!(struct ArgObj(TermArg<A>);, TermArg::p);
