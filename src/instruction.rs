#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
    Unreachable = 0x00, // trap immediately
    Nop = 0x01,         // no operation
    Block = 0x02,       //sig : block_type	begin a sequence of bytecodes, yielding 0 or 1 values
    Loop = 0x03,        //sig : block_type	begin a block which can also form control flow loops
    If = 0x04,          //sig : block_type	begin if bytecodes
    Else = 0x05,        // begin else bytecodes of if
    End = 0x0b,         // end a block, loop, or if
    Br = 0x0c,          //relative_depth : varuint32	break that targets an outer nested block
    BrIf = 0x0d, //relative_depth : varuint32	conditional break that targets an outer nested block
    BrTable = 0x0e, //see below	branch table control flow construct
    Return = 0x0f,

    // Call operators https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#call-operators-described-here
    Call = 0x10,
    CallIndirect = 0x11,

    // Parametric operators https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#parametric-operators-described-here
    Drop = 0x1a,
    Select = 0x1b,

    // Variable access https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#variable-access-described-here
    GetLocal = 0x20,
    SetLocal = 0x21,
    TeeLocal = 0x22,
    GetGlobal = 0x23,
    SetGlobal = 0x24,

    // Memory-related operators https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#memory-related-operators-described-here
    I32Load = 0x28,
    I64Load = 0x29,
    F32Load = 0x2a,
    F64Load = 0x2b,
    I32Load8S = 0x2c,
    I32Load8U = 0x2d,
    I32Load16S = 0x2e,
    I32Load16U = 0x2f,
    I64Load8S = 0x30,
    I64Load8U = 0x31,
    I64Load16S = 0x32,
    I64Load16U = 0x33,
    I64Load32S = 0x34,
    I64Load32U = 0x35,
    I32Store = 0x36,
    I64Store = 0x37,
    F32Store = 0x38,
    F64Store = 0x39,
    I32Store8 = 0x3a,
    I32Store16 = 0x3b,
    I64Store8 = 0x3c,
    I64Store16 = 0x3d,
    I64Store32 = 0x3e,
    CurrentMemory = 0x3f,
    GrowMemory = 0x40,

    // Constants https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#constants-described-here
    I32Const = 0x41,
    I64Const = 0x42,
    F32Const = 0x43,
    F64Const = 0x44,

    // Comparison operators  https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#comparison-operators-described-here
    I32Eqz = 0x45,
    I32Eq = 0x46,
    I32Ne = 0x47,
    I32LtS = 0x48,
    I32LtU = 0x49,
    I32GtS = 0x4a,
    I32GtU = 0x4b,
    I32LeS = 0x4c,
    I32LeU = 0x4d,
    I32GeS = 0x4e,
    I32GeU = 0x4f,
    I64Eqz = 0x50,
    I64Eq = 0x51,
    I64Ne = 0x52,
    I64LtS = 0x53,
    I64LtU = 0x54,
    I64GtS = 0x55,
    I64GtU = 0x56,
    I64LeS = 0x57,
    I64LeU = 0x58,
    I64GeS = 0x59,
    I64GeU = 0x5a,
    F32Eq = 0x5b,
    F32Ne = 0x5c,
    F32Lt = 0x5d,
    F32Gt = 0x5e,
    F32Le = 0x5f,
    F32Ge = 0x60,
    F64Eq = 0x61,
    F64Ne = 0x62,
    F64Lt = 0x63,
    F64Gt = 0x64,
    F64Le = 0x65,
    F64Ge = 0x66,

    I32Clz = 0x67,
    I32Ctz = 0x68,
    I32Popcnt = 0x69,
    I32Add = 0x6a,
    I32Sub = 0x6b,
    I32Mul = 0x6c,
    I32DivS = 0x6d,
    I32DivU = 0x6e,
    I32RemS = 0x6f,
    I32RemU = 0x70,
    I32And = 0x71,
    I32Or = 0x72,
    I32Xor = 0x73,
    I32Shl = 0x74,
    I32ShrS = 0x75,
    I32ShrU = 0x76,
    I32Rotl = 0x77,
    I32Rotr = 0x78,
    I64Clz = 0x79,
    I64Ctz = 0x7a,
    I64Popcnt = 0x7b,
    I64Add = 0x7c,
    I64Sub = 0x7d,
    I64Mul = 0x7e,
    I64DivS = 0x7f,
    I64DivU = 0x80,
    I64RemS = 0x81,
    I64RemU = 0x82,
    I64And = 0x83,
    I64Or = 0x84,
    I64Xor = 0x85,
    I64Shl = 0x86,
    I64ShrS = 0x87,
    I64ShrU = 0x88,
    I64Rotl = 0x89,
    I64Rotr = 0x8a,
    F32Abs = 0x8b,
    F32Neg = 0x8c,
    F32Ceil = 0x8d,
    F32Floor = 0x8e,
    F32Trunc = 0x8f,
    F32Nearest = 0x90,
    F32Sqrt = 0x91,
    F32Add = 0x92,
    F32Sub = 0x93,
    F32Mul = 0x94,
    F32Div = 0x95,
    F32Min = 0x96,
    F32Max = 0x97,
    F32Copysign = 0x98,
    F64Abs = 0x99,
    F64Neg = 0x9a,
    F64Ceil = 0x9b,
    F64Floor = 0x9c,
    F64Trunc = 0x9d,
    F64Nearest = 0x9e,
    F64Sqrt = 0x9f,
    F64Add = 0xa0,
    F64Sub = 0xa1,
    F64Mul = 0xa2,
    F64Div = 0xa3,
    F64Min = 0xa4,
    F64Max = 0xa5,
    F64Copysign = 0xa6,
}

impl From<u8> for Instruction {
    fn from(byte: u8) -> Instruction {
        match byte {
            0x00 => Instruction::Unreachable,
            0x01 => Instruction::Nop,
            0x02 => Instruction::Block,
            0x03 => Instruction::Loop,
            0x04 => Instruction::If,
            0x05 => Instruction::Else,
            0x0b => Instruction::End,
            0x0c => Instruction::Br,
            0x0d => Instruction::BrIf,
            0x0e => Instruction::BrTable,
            0x0f => Instruction::Return,

            0x10 => Instruction::Call,
            0x11 => Instruction::CallIndirect,
            0x1a => Instruction::Drop,
            0x1b => Instruction::Select,

            0x20 => Instruction::GetLocal,
            0x21 => Instruction::SetLocal,
            0x22 => Instruction::TeeLocal,
            0x23 => Instruction::GetGlobal,
            0x24 => Instruction::SetGlobal,

            0x28 => Instruction::I32Load,
            0x29 => Instruction::I64Load,
            0x2a => Instruction::F32Load,
            0x2b => Instruction::F64Load,
            0x2c => Instruction::I32Load8S,
            0x2d => Instruction::I32Load8U,
            0x2e => Instruction::I32Load16S,
            0x2f => Instruction::I32Load16U,
            0x30 => Instruction::I64Load8S,
            0x31 => Instruction::I64Load8U,
            0x32 => Instruction::I64Load16S,
            0x33 => Instruction::I64Load16U,
            0x34 => Instruction::I64Load32S,
            0x35 => Instruction::I64Load32U,

            0x36 => Instruction::I32Store,
            0x37 => Instruction::I64Store,
            0x38 => Instruction::F32Store,
            0x39 => Instruction::F64Store,
            0x3a => Instruction::I32Store8,
            0x3b => Instruction::I32Store16,
            0x3c => Instruction::I64Store8,
            0x3d => Instruction::I64Store16,
            0x3e => Instruction::I64Store32,
            0x3f => Instruction::CurrentMemory,
            0x40 => Instruction::GrowMemory,

            0x41 => Instruction::I32Const,
            0x42 => Instruction::I64Const,
            0x43 => Instruction::F32Const,
            0x44 => Instruction::F64Const,

            0x45 => Instruction::I32Eqz,
            0x46 => Instruction::I32Eq,
            0x47 => Instruction::I32Ne,
            0x48 => Instruction::I32LtS,
            0x49 => Instruction::I32LtU,
            0x4a => Instruction::I32GtS,
            0x4b => Instruction::I32GtU,
            0x4c => Instruction::I32LeS,
            0x4d => Instruction::I32LeU,
            0x4e => Instruction::I32GeS,
            0x4f => Instruction::I32GeU,
            0x50 => Instruction::I64Eqz,
            0x51 => Instruction::I64Eq,
            0x52 => Instruction::I64Ne,
            0x53 => Instruction::I64LtS,
            0x54 => Instruction::I64LtU,
            0x55 => Instruction::I64GtS,
            0x56 => Instruction::I64GtU,
            0x57 => Instruction::I64LeS,
            0x58 => Instruction::I64LeU,
            0x59 => Instruction::I64GeS,
            0x5a => Instruction::I64GeU,
            0x5b => Instruction::F32Eq,
            0x5c => Instruction::F32Ne,
            0x5d => Instruction::F32Lt,
            0x5e => Instruction::F32Gt,
            0x5f => Instruction::F32Le,
            0x60 => Instruction::F32Ge,
            0x61 => Instruction::F64Eq,
            0x62 => Instruction::F64Ne,
            0x63 => Instruction::F64Lt,
            0x64 => Instruction::F64Gt,
            0x65 => Instruction::F64Le,
            0x66 => Instruction::F64Ge,

            // Conversions https://github.com/WebAssembly/design/blob/main/BinaryEncoding.md#conversions-described-here
            0x67 => Instruction::I32Clz,
            0x68 => Instruction::I32Ctz,
            0x69 => Instruction::I32Popcnt,
            0x6a => Instruction::I32Add,
            0x6b => Instruction::I32Sub,
            0x6c => Instruction::I32Mul,
            0x6d => Instruction::I32DivS,
            0x6e => Instruction::I32DivU,
            0x6f => Instruction::I32RemS,
            0x70 => Instruction::I32RemU,
            0x71 => Instruction::I32And,
            0x72 => Instruction::I32Or,
            0x73 => Instruction::I32Xor,
            0x74 => Instruction::I32Shl,
            0x75 => Instruction::I32ShrS,
            0x76 => Instruction::I32ShrU,
            0x77 => Instruction::I32Rotl,
            0x78 => Instruction::I32Rotr,
            0x79 => Instruction::I64Clz,
            0x7a => Instruction::I64Ctz,
            0x7b => Instruction::I64Popcnt,
            0x7c => Instruction::I64Add,
            0x7d => Instruction::I64Sub,
            0x7e => Instruction::I64Mul,
            0x7f => Instruction::I64DivS,
            0x80 => Instruction::I64DivU,
            0x81 => Instruction::I64RemS,
            0x82 => Instruction::I64RemU,
            0x83 => Instruction::I64And,
            0x84 => Instruction::I64Or,
            0x85 => Instruction::I64Xor,
            0x86 => Instruction::I64Shl,
            0x87 => Instruction::I64ShrS,
            0x88 => Instruction::I64ShrU,
            0x89 => Instruction::I64Rotl,
            0x8a => Instruction::I64Rotr,
            0x8b => Instruction::F32Abs,
            0x8c => Instruction::F32Neg,
            0x8d => Instruction::F32Ceil,
            0x8e => Instruction::F32Floor,
            0x8f => Instruction::F32Trunc,
            0x90 => Instruction::F32Nearest,
            0x91 => Instruction::F32Sqrt,
            0x92 => Instruction::F32Add,
            0x93 => Instruction::F32Sub,
            0x94 => Instruction::F32Mul,
            0x95 => Instruction::F32Div,
            0x96 => Instruction::F32Min,
            0x97 => Instruction::F32Max,
            0x98 => Instruction::F32Copysign,
            0x99 => Instruction::F64Abs,
            0x9a => Instruction::F64Neg,
            0x9b => Instruction::F64Ceil,
            0x9c => Instruction::F64Floor,
            0x9d => Instruction::F64Trunc,
            0x9e => Instruction::F64Nearest,
            0x9f => Instruction::F64Sqrt,
            0xa0 => Instruction::F64Add,
            0xa1 => Instruction::F64Sub,
            0xa2 => Instruction::F64Mul,
            0xa3 => Instruction::F64Div,
            0xa4 => Instruction::F64Min,
            0xa5 => Instruction::F64Max,
            0xa6 => Instruction::F64Copysign,

            _ => panic!("Invalid byte OpCode {} hex:{:x}", byte, byte),
        }
    }
}
