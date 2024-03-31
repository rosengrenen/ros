use super::parser::{item, Input, ParseResult};

macro_rules! op {
    ($opcode:literal, $name:ident) => {
        #[derive(Debug)]
        pub struct $name;
        impl $name {
            pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
                let (_, input) = item(input, $opcode)?;
                Ok(($name, input))
            }
        }
    };
    (($opcode1:literal, $opcode2:literal), $name:ident) => {
        #[derive(Debug)]
        pub struct $name;
        impl $name {
            pub fn parse<'a>(input: Input<'a>) -> ParseResult<'a, Self> {
                let (_, input) = item(input, $opcode1)?;
                let (_, input) = item(input, $opcode2)?;
                Ok(($name, input))
            }
        }
    };
}

op!(0x00, ZeroOp);
op!(0x01, OneOp);
op!(0x06, AliasOp);
op!(0x08, NameOp);
op!(0x0a, BytePrefix);
op!(0x0b, WordPrefix);
op!(0x0c, DWordPrefix);
op!(0x0d, StringPrefix);
op!(0x0e, QWordPrefix);
op!(0x10, ScopeOp);
op!(0x11, BufferOp);
op!(0x12, PkgOp);
op!(0x13, VarPkgOp);
op!(0x14, MethodOp);
op!(0x15, ExternalOp);
op!(0x2e, DualNamePrefix);
op!(0x2f, MultiNamePrefix);
// op!(0x5b, ExtOpPrefix);
op!((0x5b, 0x01), MutexOp);
op!((0x5b, 0x02), EventOp);
op!((0x5b, 0x12), CondRefOfOp);
op!((0x5b, 0x13), CreateFieldOp);
op!((0x5b, 0x1f), LoadTableOp);
op!((0x5b, 0x20), LoadOp);
op!((0x5b, 0x21), StallOp);
op!((0x5b, 0x22), SleepOp);
op!((0x5b, 0x23), AcquireOp);
op!((0x5b, 0x24), SignalOp);
op!((0x5b, 0x25), WaitOp);
op!((0x5b, 0x26), ResetOp);
op!((0x5b, 0x27), ReleaseOp);
op!((0x5b, 0x28), FromBcdOp);
op!((0x5b, 0x29), ToBcdOp);
op!((0x5b, 0x30), RevisionOp);
op!((0x5b, 0x31), DebugOp);
op!((0x5b, 0x32), FatalOp);
op!((0x5b, 0x33), TimerOp);
op!((0x5b, 0x80), OpRegionOp);
op!((0x5b, 0x81), FieldOp);
op!((0x5b, 0x82), DeviceOp);
op!((0x5b, 0x83), ProcessorOp);
op!((0x5b, 0x84), PowerResOp);
op!((0x5b, 0x85), ThermalZoneOp);
op!((0x5b, 0x86), IndexFieldOp);
op!((0x5b, 0x87), BankFieldOp);
op!((0x5b, 0x88), DataRegionOp);
op!(0x5c, RootChar);
op!(0x5e, ParentPrefixChar);
op!(0x60, Local0Op);
op!(0x61, Local1Op);
op!(0x62, Local2Op);
op!(0x63, Local3Op);
op!(0x64, Local4Op);
op!(0x65, Local5Op);
op!(0x66, Local6Op);
op!(0x67, Local7Op);
op!(0x68, Arg0Op);
op!(0x69, Arg1Op);
op!(0x6a, Arg2Op);
op!(0x6b, Arg3Op);
op!(0x6c, Arg4Op);
op!(0x6d, Arg5Op);
op!(0x6e, Arg6Op);
op!(0x70, StoreOp);
op!(0x71, RefOfOp);
op!(0x72, AddOp);
op!(0x73, ConcatOp);
op!(0x74, SubtractOp);
op!(0x75, IncrementOp);
op!(0x76, DecrementOp);
op!(0x77, MultiplyOp);
op!(0x78, DivideOp);
op!(0x79, ShiftLeftOp);
op!(0x7a, ShiftRightOp);
op!(0x7b, AndOp);
op!(0x7c, NAndOp);
op!(0x7d, OrOp);
op!(0x7e, NOrOp);
op!(0x7f, XOrOp);
op!(0x80, NotOp);
op!(0x81, FindSetLeftBitOp);
op!(0x82, FindSetRightBitOp);
op!(0x83, DerefOfOp);
op!(0x84, ConcatResOp);
op!(0x85, ModOp);
op!(0x86, NotifyOp);
op!(0x87, SizeOfOp);
op!(0x88, IndexOp);
op!(0x89, MatchOp);
op!(0x8a, CreateDWordFieldOp);
op!(0x8b, CreateWordFieldOp);
op!(0x8c, CreateByteFieldOp);
op!(0x8d, CreateBitFieldOp);
op!(0x8e, ObjTypeOp);
op!(0x8f, CreateQWordFieldOp);
op!(0x90, LAndOp);
op!(0x91, LOrOp);
op!(0x92, LNotOp);
op!((0x92, 0x93), LNotEqualOp);
op!((0x92, 0x94), LLessEqualOp);
op!((0x92, 0x95), LGreaterEqualOp);
op!(0x93, LEqualOp);
op!(0x94, LGreaterOp);
op!(0x95, LLessOp);
op!(0x96, ToBufferOp);
op!(0x97, ToDecimalStringOp);
op!(0x98, ToHexStringOp);
op!(0x99, ToIntegerOp);
op!(0x9c, ToStringOp);
op!(0x9d, CopyObjOp);
op!(0x9e, MidOp);
op!(0x9f, ContinueOp);
op!(0xa0, IfOp);
op!(0xa1, ElseOp);
op!(0xa2, WhileOp);
op!(0xa3, NoopOp);
op!(0xa4, ReturnOp);
op!(0xa5, BreakOp);
op!(0xcc, BreakPointOp);
op!(0xff, OnesOp);
