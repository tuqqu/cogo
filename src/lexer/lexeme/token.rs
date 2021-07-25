use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug, Eq, Hash)]
pub enum Token {
    // Operators
    Colon,
    Semicolon,
    Comma,
    Dot,
    Backquote,

    LeftParen,
    RightParen,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftBracket,
    RightBracket,

    Plus,
    Minus,
    Slash,
    Modulus,
    Asterisk,

    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitClear,

    LeftShift,
    RightShift,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Bang,
    Equal,

    ColonEqual,

    EqualEqual,
    BangEqual,

    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,
    ModulusEqual,

    BitwiseAndEqual,
    BitwiseOrEqual,
    BitwiseXorEqual,
    BitClearEqual,
    LeftShiftEqual,
    RightShiftEqual,

    LogicAnd,
    LogicOr,

    Inc,
    Dec,

    ChanArrow,

    Identifier,
    StringLiteral,
    RawStringLiteral,
    IntLiteral,
    FloatLiteral,

    // Keywords
    Break,
    Case,
    Chan,
    Const,
    Continue,
    Default,
    Defer,
    Else,
    Fallthrough,
    For,
    Func,
    Go,
    Goto,
    If,
    Import,
    Interface,
    Map,
    Package,
    Range,
    Return,
    Select,
    Struct,
    Switch,
    Type,
    Var,

    // Types
    Nil,

    Bool,
    False,
    True,

    Int8,
    Int16,
    Int32, Rune,
    Int64,
    Int,

    Uint8, Byte,
    Uint16,
    Uint32,
    Uint64,
    Uint,
    Uintptr,

    Float32,
    Float64,

    Complex64,
    Complex128,

    String,

    Eof,
}

impl Token {
    pub fn to_string(&self) -> &'static str {
        match self {
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::Comma => ",",
            Self::Dot => ".",
            Self::Backquote => "`",

            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftCurlyBrace => "{",
            Self::RightCurlyBrace => "}",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",

            Self::Plus => "+",
            Self::Minus => "-",
            Self::Slash => "/",
            Self::Modulus => "%",
            Self::Asterisk => "*",

            Self::BitwiseAnd => "&",
            Self::BitwiseOr => "|",
            Self::BitwiseXor => "^",
            Self::BitClear => "&^",

            Self::LeftShift => "<<",
            Self::RightShift => ">>",

            Self::Greater => ">",
            Self::GreaterEqual => ">=",
            Self::Less => "<",
            Self::LessEqual => "<=",

            Self::Bang => "!",
            Self::Equal => "=",

            Self::ColonEqual => ":=",

            Self::EqualEqual => "==",
            Self::BangEqual => "!=",

            Self::PlusEqual => "+=",
            Self::MinusEqual => "-=",
            Self::AsteriskEqual => "*=",
            Self::SlashEqual => "/=",
            Self::ModulusEqual => "%=",

            Self::BitwiseAndEqual => "&=",
            Self::BitwiseOrEqual => "|=",
            Self::BitwiseXorEqual => "^=",
            Self::BitClearEqual => "&^=",
            Self::LeftShiftEqual => "<<=",
            Self::RightShiftEqual => ">>=",

            Self::LogicAnd => "&",
            Self::LogicOr => "|",

            Self::Inc => "++",
            Self::Dec => "--",

            Self::ChanArrow => "<-",

            Self::Identifier => "",
            Self::StringLiteral => "",
            Self::RawStringLiteral => "",
            Self::IntLiteral => "",
            Self::FloatLiteral => "",

            //Keywords
            Self::Break => "break",
            Self::Case => "case",
            Self::Chan => "chan",
            Self::Const => "const",
            Self::Continue => "continue",
            Self::Default => "default",
            Self::Defer => "defer",
            Self::Else => "else",
            Self::Fallthrough => "fallthrough",
            Self::For => "for",
            Self::Func => "func",
            Self::Go => "go",
            Self::Goto => "goto",
            Self::If => "if",
            Self::Import => "import",
            Self::Interface => "interface",
            Self::Map => "map",
            Self::Package => "package",
            Self::Range => "range",
            Self::Return => "return",
            Self::Select => "select",
            Self::Struct => "struct",
            Self::Switch => "switch",
            Self::Type => "type",
            Self::Var => "var",

            // Types
            Self::Nil => "nil",

            Self::Bool => "bool",
            Self::False => "false",
            Self::True => "true",

            Self::Int8 => "int8",
            Self::Int16 => "int16",
            Self::Int32 => "int32",
            Self::Rune => "rune",
            Self::Int64 => "int64",
            Self::Int => "int",

            Self::Uint8 => "uint8",
            Self::Byte => "byte",
            Self::Uint16 => "uint16",
            Self::Uint32 => "uint32",
            Self::Uint64 => "uint64",
            Self::Uint => "uint",
            Self::Uintptr => "uintptr",

            Self::Float32 => "float32",
            Self::Float64 => "float64",

            Self::Complex64 => "complex64",
            Self::Complex128 => "complex128",

            Self::String => "string",

            Self::Eof => "EOF",
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
