const STRICT_KEYWORDS: &'static [&'static str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn",
];

const STRICT_KEYWORDS_LEN: usize = STRICT_KEYWORDS.len();

const RESERVED_KEYWORDS: &'static [&'static str] = &[
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized",
    "virtual", "yield", "try",
];

const RESERVED_KEYWORDS_LEN: usize = RESERVED_KEYWORDS.len();

const LITERAL_TOKENS: &'static [LiteralToken] = &[
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
];

const LITERAL_TOKENS_LEN: usize = LITERAL_TOKENS.len();

const COMMENT_IDENTIFIER: &'static str = "//";
const STRING_IDENTIFIER: char = '"';

#[allow(unused)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Unknown,
    End,
    Keyword,
    Symbol,
    Comment,
    Semicolon,
    String,
    OpenParen,
    CloseParen,
    OpenCurley,
    CloseCurley,
}

#[derive(Debug)]
struct LiteralToken<'a> {
    literal_token: &'a str,
    token_kind: Token,
}

#[allow(unused)]
#[derive(Debug)]
struct Loc {
    line: usize,
    from: usize,
    to: usize,
}

#[allow(unused)]
#[derive(Debug)]
struct TokenInfo {
    token: Token,
    collected: String,
    loc: Loc,
}

#[derive(Debug)]
struct Lexer {
    content: String,
    content_len: usize,
    cursor: usize,
    line: usize,
    collected: String,
}

#[derive(Debug)]
enum LexerStatus {
    ConsumedSuccess,
    ConsumeFailed(String),
    LenOutOfBounds(String),
}

impl Lexer {
    fn new(content: String) -> Self {
        Self {
            content_len: content.len() + 1,
            content,
            cursor: 0,
            line: 1,
            collected: String::new(),
        }
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
            return Err(LexerStatus::LenOutOfBounds(format!(
                "Provided `len`: {len} is too large for `content_len`: {}",
                self.content_len
            )));
        }

        let mut local_len = 0;

        loop {
            let c = self.current()?;

            self.collected.push(c);

            self.advance_cursor()?;

            local_len += 1;

            if local_len == len {
                break;
            }
        }

        Ok(LexerStatus::ConsumedSuccess)
    }

    fn advance_cursor(&mut self) -> Result<(), LexerStatus> {
        if self.current()?.is_line_break() {
            self.line += 1;
        }

        if self.cursor + 1 < self.content_len {
            self.cursor += 1;
        }

        Ok(())
    }

    fn regress_cursor_by(&mut self, n: usize) {
        if self.cursor - n > 0 {
            self.cursor -= n;
        }
    }

    fn content_starts_with(&mut self, prefix: &str) -> Result<bool, LexerStatus> {
        for i in 0..prefix.len() {
            let cur_char = prefix.chars().nth(i);

            match cur_char {
                Some(c) => {
                    if c != self.current()? {
                        self.regress_cursor_by(i);

                        return Ok(false);
                    }
                }
                None => {
                    return Err(LexerStatus::LenOutOfBounds(format!(
                        "TODO content_starts_with"
                    )))
                }
            }

            self.advance_cursor()?;
        }

        self.regress_cursor_by(prefix.len());

        Ok(true)
    }

    fn current(&mut self) -> Result<char, LexerStatus> {
        match self.content.chars().nth(self.cursor) {
            Some(c) => {
                return Ok(c);
            }
            None => return Err(LexerStatus::ConsumeFailed(format!("TODO! current"))),
        }
    }

    fn get_loc(&mut self, modifiy_loc: bool) -> (Loc, String) {
        //Todo this could be better
        let collected = self.collected.clone();
        let collected_len = collected.len();

        self.collected = String::new();

        let (from, to) = if modifiy_loc {
            (self.cursor - 1, self.cursor - 1)
        } else {
            (self.cursor - collected_len, self.cursor)
        };

        (
            Loc {
                line: self.line,
                from,
                to,
            },
            collected,
        )
    }

    fn next(&mut self) -> Result<TokenInfo, LexerStatus> {
        while self.cursor + 1 < self.content_len {
            self.trim_while_empty()?;

            if self.current()? == STRING_IDENTIFIER {
                self.advance_cursor()?;

                while self.current()? != STRING_IDENTIFIER && !self.current()?.is_line_break() {
                    self.consume(1)?;
                }

                let (loc, collected) = self.get_loc(false);
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

                let (loc, collected) = self.get_loc(false);
                self.advance_cursor()?;

                return Ok(TokenInfo {
                    token: Token::Comment,
                    collected,
                    loc,
                });
            }

            for i in 0..LITERAL_TOKENS_LEN {
                if self.content_starts_with(LITERAL_TOKENS[i].literal_token)? {
                    self.consume(1)?;
                    let (loc, collected) = self.get_loc(true);

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

                let (loc, collected) = self.get_loc(false);

                let token = if STRICT_KEYWORDS.contains(&collected.as_str()) {
                    Token::Keyword
                } else {
                    Token::Symbol
                };

                if STRICT_KEYWORDS.contains(&collected.as_str()) {}

                return Ok(TokenInfo {
                    token,
                    collected,
                    loc,
                });
            }

            self.advance_cursor()?;
        }
        Ok(TokenInfo {
            token: Token::Unknown,
            collected: String::new(),
            loc: Loc {
                line: 0,
                from: 0,
                to: 0,
            },
        })
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

fn main() {
    let input = "fn main() {
                          //Print Hello, World
                          println!(\"Hello, World\");
                          if a > b {
                            let x = 1;
                          }
                       }";
    let mut lexer = Lexer::new(input.into());

    let mut t = lexer.next().unwrap();

    while t.token != Token::Unknown {
        println!("{:#?}", t);
        t = lexer.next().unwrap();
    }

    //println!("{:#?}", lexer);
}
