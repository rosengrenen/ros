use crate::aml::Context;
use core::alloc::Allocator;
use parser::{
    error::{ParseError, ParseResult},
    input::Input,
    parser::Parser,
    primitive::item::item,
};

macro_rules! op_parser {
    ($opcode:literal) => {
        item($opcode)
    };
    (($opcode1:literal, $opcode2:literal)) => {
        (item($opcode1), item($opcode2))
    };
}

macro_rules! ops {
    {$($parser:tt => $name:ident)+} => {
        $(
            pub struct $name;

            impl $name {
                pub fn p<I: Input<Item=u8>, E: ParseError<I,A>, A: Allocator + Clone>(
                  input: I,
                  context: &mut Context,
                  alloc: A,
                ) -> ParseResult<I, Self, E> {
                    op_parser!($parser).map(|_| Self).parse(input, context, alloc)
                }
            }
        )+
    };
}

ops! {
    0x00 => ZeroOp
    0x01 => OneOp
    0x06 => AliasOp
    0x08 => NameOp
    0x0a => BytePrefix
    0x0b => WordPrefix
    0x0c => DWordPrefix
    0x0d => StringPrefix
    0x0e => QWordPrefix
    0x10 => ScopeOp
    0x11 => BufferOp
    0x12 => PkgOp
    0x13 => VarPkgOp
    0x14 => MethodOp
    0x15 => ExternalOp
    0x2e => DualNamePrefix
    0x2f => MultiNamePrefix
    0x5b => ExtOpPrefix
    (0x5b, 0x01) => MutexOp
    (0x5b, 0x02) => EventOp
    (0x5b, 0x12) => CondRefOfOp
    (0x5b, 0x13) => CreateFieldOp
    (0x5b, 0x1f) => LoadTableOp
    (0x5b, 0x20) => LoadOp
    (0x5b, 0x21) => StallOp
    (0x5b, 0x22) => SleepOp
    (0x5b, 0x23) => AcquireOp
    (0x5b, 0x24) => SignalOp
    (0x5b, 0x25) => WaitOp
    (0x5b, 0x26) => ResetOp
    (0x5b, 0x27) => ReleaseOp
    (0x5b, 0x28) => FromBcdOp
    (0x5b, 0x29) => ToBcdOp
    (0x5b, 0x30) => RevisionOp
    (0x5b, 0x31) => DebugOp
    (0x5b, 0x32) => FatalOp
    (0x5b, 0x33) => TimerOp
    (0x5b, 0x80) => OpRegionOp
    (0x5b, 0x81) => FieldOp
    (0x5b, 0x82) => DeviceOp
    (0x5b, 0x84) => PowerResOp
    (0x5b, 0x85) => ThermalZoneOp
    (0x5b, 0x86) => IndexFieldOp
    (0x5b, 0x87) => BankFieldOp
    (0x5b, 0x88) => DataRegionOp
    0x5c => RootChar
    0x5e => ParentPrefixChar
    0x60 => Local0Op
    0x61 => Local1Op
    0x62 => Local2Op
    0x63 => Local3Op
    0x64 => Local4Op
    0x65 => Local5Op
    0x66 => Local6Op
    0x67 => Local7Op
    0x68 => Arg0Op
    0x69 => Arg1Op
    0x6a => Arg2Op
    0x6b => Arg3Op
    0x6c => Arg4Op
    0x6d => Arg5Op
    0x6e => Arg6Op
    0x70 => StoreOp
    0x71 => RefOfOp
    0x72 => AddOp
    0x73 => ConcatOp
    0x74 => SubtractOp
    0x75 => IncrementOp
    0x76 => DecrementOp
    0x77 => MultiplyOp
    0x78 => DivideOp
    0x79 => ShiftLeftOp
    0x7a => ShiftRightOp
    0x7b => AndOp
    0x7c => NandOp
    0x7d => OrOp
    0x7e => NorOp
    0x7f => XorOp
    0x80 => NotOp
    0x81 => FindSetLeftBitOp
    0x82 => FindSetRightBitOp
    0x83 => DerefOfOp
    0x84 => ConcatResOp
    0x85 => ModOp
    0x86 => NotifyOp
    0x87 => SizeOfOp
    0x88 => IndexOp
    0x89 => MatchOp
    0x8a => CreateDWordFieldOp
    0x8b => CreateWordFieldOp
    0x8c => CreateByteFieldOp
    0x8d => CreateBitFieldOp
    0x8e => ObjTypeOp
    0x8f => CreateQWordFieldOp
    0x90 => LAndOp
    0x91 => LOrOp
    0x92 => LNotOp
    (0x92, 0x93) => LNotEqualOp
    (0x92, 0x94) => LLessEqualOp
    (0x92, 0x95) => LGreaterEqualOp
    0x93 => LEqualOp
    0x94 => LGreaterOp
    0x95 => LLessOp
    0x96 => ToBufferOp
    0x97 => ToDecimalStringOp
    0x98 => ToHexStringOp
    0x99 => ToIntegerOp
    0x9c => ToStringOp
    0x9d => CopyObjOp
    0x9e => MidOp
    0x9f => ContinueOp
    0xa0 => IfOp
    0xa1 => ElseOp
    0xa2 => WhileOp
    0xa3 => NoopOp
    0xa4 => ReturnOp
    0xa5 => BreakOp
    0xcc => BreakPointOp
    0xff => OnesOp
}
