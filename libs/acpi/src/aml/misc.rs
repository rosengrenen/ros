use super::ops::{
    Arg0Op, Arg1Op, Arg2Op, Arg3Op, Arg4Op, Arg5Op, Arg6Op, DebugOp, Local0Op, Local1Op, Local2Op,
    Local3Op, Local4Op, Local5Op, Local6Op, Local7Op,
};

parser_enum!(
    enum ArgObj {
        Arg0(Arg0Op),
        Arg1(Arg1Op),
        Arg2(Arg2Op),
        Arg3(Arg3Op),
        Arg4(Arg4Op),
        Arg5(Arg5Op),
        Arg6(Arg6Op),
    }
);

parser_enum!(
    enum LocalObj {
        Local0(Local0Op),
        Local1(Local1Op),
        Local2(Local2Op),
        Local3(Local3Op),
        Local4(Local4Op),
        Local5(Local5Op),
        Local6(Local6Op),
        Local7(Local7Op),
    }
);

parser_struct_empty!(struct DebugObj;, DebugOp::p);
