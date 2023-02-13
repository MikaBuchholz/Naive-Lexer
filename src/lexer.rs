use crate::consts;

use std::{
    fs::File,
    io::{BufReader, Read},
};

use crate::lexer::consts::{
    COMMENT_IDENTIFIER, LITERAL_TOKENS, LITERAL_TOKENS_LEN, RESERVED_KEYWORDS, STRICT_KEYWORDS,
    STRING_IDENTIFIER,
};

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub enum Token {
    Unknown,
    End,
    StrictKeyword,
    ReservedKeyword,
    Symbol,
    Comment,
    Semicolon,
    String,
    OpenParen,
    CloseParen,
    OpenCurley,
    CloseCurley,
    OpenSqaureBrackets,
    CloseSqaureBrackets,
    Ampersand,
    SingleQuote,
    Lt,
    Gt,
    Le,
    Ge,
    Colon,
    Comma,
    Arrow,
    DoubleColon,
    FatArrow,
    Question,
    Dollar,
    Not,
    AssignOp,
    Ne,
    Eq,
    And,
}

#[derive(Debug)]
pub struct LiteralToken<'a> {
    pub literal_token: &'a str,
    pub token_kind: Token,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Loc {
    pub line: usize,
    pub token_pos: std::ops::Range<usize>,
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub token: Token,
    pub collected: String,
    pub loc: Loc,
}

#[derive(Debug)]
pub struct Lexer {
    content: String,
    content_len: usize,
    cursor: usize,
    line: usize,
    collected: String,
}

#[derive(Debug)]
pub enum LexerStatus {
    ConsumedSuccess,
    ConsumeFailed(String),
    RegressSuccess,
    RegressFailed(String),
    AdvanceSuccess,
    AdvanceFailed(String),
    CursorOutOfBounds(String),
}

macro_rules! repeat {
    ($n:expr, $code:block) => {{
        for _ in 0..$n {
            $code
        }
    }};
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            content_len: content.len(),
            content,
            cursor: 0,
            line: 1,
            collected: String::new(),
        }
    }

    pub fn from_path(path: &str) -> Result<Self, std::io::Error> {
        let mut buffer = String::new();
        let mut buffer_reader = BufReader::new(File::open(path)?);

        buffer_reader.read_to_string(&mut buffer)?;

        Ok(Self {
            content_len: buffer.len(),
            content: buffer,
            cursor: 0,
            line: 1,
            collected: String::new(),
        })
    }

    fn trim_while_empty(&mut self) -> Result<LexerStatus, LexerStatus> {
        loop {
            let c = self.current()?;

            if !c.is_empty() {
                break Ok(LexerStatus::ConsumedSuccess);
            }

            self.advance_cursor()?;
        }
    }

    fn consume(&mut self, len: usize) -> Result<LexerStatus, LexerStatus> {
        if self.cursor + len > self.content_len {
            return Err(LexerStatus::CursorOutOfBounds(format!(
                "Provided `len`: {} is too large for `content_len`: {}",
                self.cursor + len,
                self.content_len
            )));
        }

        repeat!(len, {
            let c = self.current()?;

            self.collected.push(c);

            self.advance_cursor()?;
        });

        Ok(LexerStatus::ConsumedSuccess)
    }

    fn advance_cursor(&mut self) -> Result<LexerStatus, LexerStatus> {
        if self.current()?.is_line_break() {
            self.line += 1;
        }

        if self.cursor + 1 < self.content_len {
            self.cursor += 1;

            return Ok(LexerStatus::AdvanceSuccess);
        }

        Err(LexerStatus::AdvanceFailed(format!(
            "Can not advance cursor | `cursor` : {} + 1 out of bounds for `content_len` : {}",
            self.cursor, self.content_len
        )))
    }

    fn regress_cursor_by(&mut self, n: usize) -> Result<LexerStatus, LexerStatus> {
        #[allow(unused_comparisons)] //Can be allowed, compiler can not know this might not happen
        if (self.cursor - n) >= 0 {
            self.cursor -= n;

            return Ok(LexerStatus::RegressSuccess);
        }

        Err(LexerStatus::RegressFailed(format!(
            "Cursor regression failed. Cursor can not be negative | `cursor` : {}",
            self.cursor - 1
        )))
    }

    fn content_starts_with(&mut self, prefix: &str) -> Result<bool, LexerStatus> {
        for i in 0..prefix.len() {
            match prefix.chars().nth(i) {
                Some(c) => {
                    if c != self.current()? {
                        self.regress_cursor_by(i)?;

                        return Ok(false);
                    }
                }
                None => {
                    return Err(LexerStatus::CursorOutOfBounds(format!(
                        "`cursor` : {} can not be >= then `content_len` : {}",
                        self.cursor + 1,
                        self.content_len
                    )))
                }
            }

            self.advance_cursor()?;
        }

        self.regress_cursor_by(prefix.len())?;

        Ok(true)
    }

    fn current(&mut self) -> Result<char, LexerStatus> {
        match self.content.chars().nth(self.cursor) {
            Some(c) => {
                return Ok(c);
            }
            None => {
                return Err(LexerStatus::ConsumeFailed(format!(
                    "`cursor` : {} can not be >= then `content_len` : {}",
                    self.cursor + 1,
                    self.content_len
                )))
            }
        }
    }

    fn get_loc(&mut self) -> (Loc, String) {
        let collected = self.collected.clone();
        let collected_len = collected.len();

        self.collected = String::new();

        (
            Loc {
                line: self.line,
                token_pos: self.cursor - collected_len..self.cursor,
            },
            collected,
        )
    }

    fn next(&mut self) -> Result<TokenInfo, LexerStatus> {
        while self.cursor + 1 < self.content_len {
            self.trim_while_empty()?;

            if self.current()? == STRING_IDENTIFIER {
                self.consume(1)?; //Consume starting `"`

                while self.current()? != STRING_IDENTIFIER && !self.current()?.is_line_break() {
                    self.consume(1)?;
                }

                self.consume(1)?; //Consume ending `"`

                let (loc, collected) = self.get_loc();

                self.advance_cursor()?;

                return Ok(TokenInfo {
                    token: Token::String,
                    collected,
                    loc,
                });
            }

            if self.content_starts_with(COMMENT_IDENTIFIER)? {
                while !self.current()?.is_line_break() {
                    self.consume(1)?;
                }

                let (loc, collected) = self.get_loc();

                self.advance_cursor()?;

                return Ok(TokenInfo {
                    token: Token::Comment,
                    collected,
                    loc,
                });
            }

            for i in 0..LITERAL_TOKENS_LEN {
                if self.content_starts_with(LITERAL_TOKENS[i].literal_token)? {
                    self.consume(LITERAL_TOKENS[i].literal_token.len())?;
                    let (loc, collected) = self.get_loc();

                    return Ok(TokenInfo {
                        token: LITERAL_TOKENS[i].token_kind,
                        collected,
                        loc,
                    });
                }
            }

            if self.current()?.is_symbol_start() {
                while self.current()?.is_symbol() && !self.current()?.is_line_break() {
                    self.consume(1)?;
                }

                let (loc, collected) = self.get_loc();

                let token = if STRICT_KEYWORDS.contains(&collected.as_str()) {
                    Token::StrictKeyword
                } else {
                    if RESERVED_KEYWORDS.contains(&collected.as_str()) {
                        Token::ReservedKeyword
                    } else {
                        Token::Symbol
                    }
                };

                return Ok(TokenInfo {
                    token,
                    collected,
                    loc,
                });
            }

            self.advance_cursor()?;
        }
        Ok(TokenInfo {
            token: Token::End,
            collected: String::from("End of stream"),
            loc: Loc {
                line: 0,
                token_pos: 0..0,
            },
        })
    }

    pub fn collect_tokens(&mut self) -> Result<Vec<TokenInfo>, LexerStatus> {
        let mut current = self.next()?;

        let mut tokens: Vec<TokenInfo> = Vec::new();

        while current.token != Token::End {
            tokens.push(current.clone());
            current = self.next()?;
        }

        Ok(tokens)
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}

trait CharUtil {
    fn is_empty(self) -> bool;
    fn is_line_break(self) -> bool;
    fn is_symbol_start(self) -> bool;
    fn is_symbol(self) -> bool;
}

impl CharUtil for char {
    fn is_empty(self) -> bool {
        self == ' '
    }

    fn is_line_break(self) -> bool {
        self == '\n'
    }

    fn is_symbol_start(self) -> bool {
        self.is_alphabetic() || self == '_'
    }

    fn is_symbol(self) -> bool {
        self.is_alphanumeric() || self == '_'
    }
}
