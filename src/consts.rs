use crate::lexer::LiteralToken;
use crate::lexer::Token;

pub const STRICT_KEYWORDS: &'static [&'static str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
];

pub const RESERVED_KEYWORDS: &'static [&'static str] = &[
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try",
];

pub const LITERAL_TOKENS: &'static [LiteralToken] = &[
    LiteralToken {
        literal_token: "(",
        token_kind: Token::OpenParen,
    },
    LiteralToken {
        literal_token: ")",
        token_kind: Token::CloseParen,
    },
    LiteralToken {
        literal_token: "{",
        token_kind: Token::OpenCurley,
    },
    LiteralToken {
        literal_token: "}",
        token_kind: Token::CloseCurley,
    },
    LiteralToken {
        literal_token: ";",
        token_kind: Token::Semicolon,
    },
    LiteralToken {
        literal_token: "[",
        token_kind: Token::OpenSqaureBrackets,
    },
    LiteralToken {
        literal_token: "]",
        token_kind: Token::CloseSqaureBrackets,
    },
    LiteralToken {
        literal_token: "&&",
        token_kind: Token::And,
    },
    LiteralToken {
        literal_token: "&",
        token_kind: Token::Ampersand, //TODO This could be a ref, slice or bin op
    },
    LiteralToken {
        literal_token: "'",
        token_kind: Token::SingleQuote,
    },
    LiteralToken {
        literal_token: "<=",
        token_kind: Token::Le,
    },
    LiteralToken {
        literal_token: ">=",
        token_kind: Token::Ge,
    },
    LiteralToken {
        literal_token: "<",
        token_kind: Token::Lt,
    },
    LiteralToken {
        literal_token: ">",
        token_kind: Token::Gt,
    },
    LiteralToken {
        literal_token: "::",
        token_kind: Token::DoubleColon,
    },
    LiteralToken {
        literal_token: ":",
        token_kind: Token::Colon,
    },
    LiteralToken {
        literal_token: ",",
        token_kind: Token::Comma,
    },
    LiteralToken {
        literal_token: "->",
        token_kind: Token::Arrow,
    },
    LiteralToken {
        literal_token: "=>",
        token_kind: Token::FatArrow,
    },
    LiteralToken {
        literal_token: "?",
        token_kind: Token::Question,
    },
    LiteralToken {
        literal_token: "$",
        token_kind: Token::Dollar,
    },
    LiteralToken {
        literal_token: "==",
        token_kind: Token::Eq,
    },
    LiteralToken {
        literal_token: "!=",
        token_kind: Token::Ne,
    },
    LiteralToken {
        literal_token: "=",
        token_kind: Token::AssignOp,
    },
    LiteralToken {
        literal_token: "!",
        token_kind: Token::Not,
    },
];

//Some tokens are missing - too lazy

pub const LITERAL_TOKENS_LEN: usize = LITERAL_TOKENS.len();

pub const COMMENT_IDENTIFIER: &'static str = "//";
pub const STRING_IDENTIFIER: char = '"';
