use self::error::LexerError;
use self::lexeme::{Lexeme, Pos, Token};

mod error;
pub mod lexeme;

pub struct Lexer {
    src: String,
    lexemes: Vec<Lexeme>,
    start: usize,
    current: usize,
    line: usize,
    pos: usize,
    errors: Vec<LexerError>,
}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src,
            lexemes: vec![],
            start: 0,
            current: 0,
            line: 1,
            pos: 1,
            errors: vec![],
        }
    }

    pub fn lex(&mut self) -> (&[Lexeme], &[LexerError]) {
        while !self.is_at_end() {
            self.start = self.current;
            self.token();
        }

        self.lexemes.push(Lexeme::new(Token::Eof, self.pos()));

        (&self.lexemes, &self.errors)
    }

    fn token(&mut self) {
        match self.advance() {
            '{' => self.add_lexeme(Token::LeftCurlyBrace),
            '}' => self.add_lexeme(Token::RightCurlyBrace),
            '(' => self.add_lexeme(Token::LeftParen),
            ')' => self.add_lexeme(Token::RightParen),
            '[' => self.add_lexeme(Token::LeftBracket),
            ']' => self.add_lexeme(Token::RightBracket),
            ',' => self.add_lexeme(Token::Comma),
            '.' => self.add_lexeme(Token::Dot),
            ';' => self.add_lexeme(Token::Semicolon),
            ':' => {
                let t = if self.match_char('=') {
                    Token::ColonEqual
                } else {
                    Token::Colon
                };
                self.add_lexeme(t);
            }
            '-' => {
                let t = if self.match_char('=') {
                    Token::MinusEqual
                } else if self.match_char('-') {
                    Token::Dec
                } else {
                    Token::Minus
                };
                self.add_lexeme(t);
            }
            '+' => {
                let t = if self.match_char('=') {
                    Token::PlusEqual
                } else if self.match_char('+') {
                    Token::Inc
                } else {
                    Token::Plus
                };
                self.add_lexeme(t);
            }
            '*' => {
                let t = if self.match_char('=') {
                    Token::AsteriskEqual
                } else {
                    Token::Asterisk
                };
                self.add_lexeme(t);
            }
            '%' => {
                let t = if self.match_char('=') {
                    Token::ModulusEqual
                } else {
                    Token::Modulus
                };
                self.add_lexeme(t);
            }
            '&' => {
                let t = if self.match_char('&') {
                    Token::LogicAnd
                } else if self.match_char('=') {
                    Token::BitwiseAndEqual
                } else if self.match_char('^') {
                    Token::BitClear
                } else {
                    Token::BitwiseAnd
                };
                self.add_lexeme(t);
            }
            '|' => {
                let t = if self.match_char('|') {
                    Token::LogicOr
                } else if self.match_char('=') {
                    Token::BitwiseOrEqual
                } else {
                    Token::BitwiseOr
                };
                self.add_lexeme(t);
            }
            '!' => {
                let t = if self.match_char('=') {
                    Token::BangEqual
                } else {
                    Token::Bang
                };
                self.add_lexeme(t);
            }
            '^' => {
                let t = if self.match_char('=') {
                    Token::BitwiseXorEqual
                } else {
                    Token::BitwiseXor
                };
                self.add_lexeme(t);
            }
            '=' => {
                let t = if self.match_char('=') {
                    Token::EqualEqual
                } else {
                    Token::Equal
                };
                self.add_lexeme(t);
            }
            '<' => {
                let t = if self.match_char('=') {
                    Token::LessEqual
                } else if self.match_char('<') {
                    if self.match_char('=') {
                        Token::LeftShiftEqual
                    } else {
                        Token::LeftShift
                    }
                } else {
                    Token::Less
                };
                self.add_lexeme(t);
            }
            '>' => {
                let t = if self.match_char('=') {
                    Token::GreaterEqual
                } else if self.match_char('>') {
                    if self.match_char('=') {
                        Token::RightShiftEqual
                    } else {
                        Token::RightShift
                    }
                } else {
                    Token::Greater
                };
                self.add_lexeme(t);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    loop {
                        if self.peek() == '*' && self.peek_next() == '/' {
                            break;
                        }

                        if self.is_at_end() {
                            self.errors.push(LexerError::UnclosedComment(self.pos()));
                            return;
                        }

                        self.advance();
                    }
                    self.advance();
                    self.advance();
                } else if self.match_char('=') {
                    self.add_lexeme(Token::SlashEqual);
                } else {
                    self.add_lexeme(Token::Slash);
                }
            }
            ' ' | '\r' | '\t' => {
                self.pos += 1;
            }
            '\n' => {
                self.line += 1;
                self.pos = 0;
                if self.is_auto_semicolon() {
                    self.add_lexeme(Token::Semicolon);
                }
            }
            '"' => self.string(),
            c => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alphabetic(c) {
                    self.identifier();
                } else {
                    self.pos += 1;
                    self.errors
                        .push(LexerError::UnknownCharacter(self.pos(), c));
                }
            }
        };
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.src.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        self.src.chars().nth(self.current - 1).unwrap()
    }

    fn add_lexeme(&mut self, token: Token) {
        self.lexemes.push(Lexeme::new(token, self.pos()));
        let text = self.src_substr(self.start, self.current);
        self.pos += text.len();
    }

    fn add_lexeme_with_literal(&mut self, token: Token, literal: &str) {
        self.lexemes.push(Lexeme::new_with_literal(
            token,
            self.pos(),
            literal.to_string(),
        ));

        self.pos += literal.len();
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.src.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.src.chars().nth(self.current).unwrap()
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.pos = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.errors.push(LexerError::UnterminatedString(self.pos()));
            return;
        }

        self.advance();

        let val = self.src_substr(self.start + 1, self.current - 1);
        self.add_lexeme_with_literal(Token::StringLiteral, &val);
    }

    fn src_substr(&self, start: usize, end: usize) -> String {
        self.src.chars().skip(start).take(end - start).collect()
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10)
    }

    fn is_alphabetic(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    fn is_alphanumeric(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    fn number(&mut self) {
        let mut float = false;

        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            float = true;
            // consume '.'
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_lexeme_with_literal(
            if float {
                Token::FloatLiteral
            } else {
                Token::IntLiteral
            },
            &self.src_substr(self.start, self.current),
        );
    }

    fn identifier(&mut self) {
        while self.is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = self.src_substr(self.start, self.current);
        let token = self.keyword(&text as &str);

        if let Some(t) = token {
            self.add_lexeme(t);
        } else {
            self.add_lexeme_with_literal(Token::Identifier, &text);
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.src.len() {
            return '\0';
        }

        self.src.chars().nth(self.current + 1).unwrap()
    }

    fn pos(&self) -> Pos {
        Pos(self.line, self.pos)
    }

    fn keyword(&self, keyword: &str) -> Option<Token> {
        use Token::*;
        let tok = match keyword {
            "break" => Break,
            "case" => Case,
            "chan" => Chan,
            "const" => Const,
            "continue" => Continue,
            "default" => Default,
            "defer" => Defer,
            "else" => Else,
            "fallthrough" => Fallthrough,
            "for" => For,
            "func" => Func,
            "go" => Go,
            "goto" => Goto,
            "if" => If,
            "import" => Import,
            "interface" => Interface,
            "map" => Map,
            "package" => Package,
            "range" => Range,
            "return" => Return,
            "select" => Select,
            "struct" => Struct,
            "switch" => Switch,
            "type" => Type,
            "var" => Var,

            "nil" => Nil,

            "bool" => Bool,
            "false" => False,
            "true" => True,

            "int8" => Int8,
            "int16" => Int16,
            "int32" => Int32,
            "rune" => Rune,
            "int64" => Int64,
            "int" => Int,

            "uint8" => Uint8,
            "byte" => Byte,
            "uint16" => Uint16,
            "uint32" => Uint32,
            "uint64" => Uint64,
            "uint" => Uint,
            "uintptr" => Uintptr,

            "float32" => Float32,
            "float64" => Float64,

            "complex64" => Complex64,
            "complex128" => Complex128,

            "string" => String,

            _ => return None,
        };

        Some(tok)
    }

    fn is_auto_semicolon(&self) -> bool {
        let last = self.lexemes.last();
        if let Some(l) = last {
            use Token::*;
            matches!(
                l.token,
                RightCurlyBrace | RightParen | Inc | Dec | Return | Fallthrough | Continue
                | Break | Bool | False | True | Int8 | Int16 | Int32 | Rune | Int64 | Int
                | Uint8 | Byte | Uint16 | Uint32 | Uint64 | Uint | Uintptr | Float32 | Float64
                | Complex64 | Complex128 | String | Identifier
            )
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let mut lexer = Lexer::new(str::to_string("var x uint64 = 100; y := \"str\""));
        let (lexemes, errs) = lexer.lex();
        assert!(errs.is_empty());
        assert_eq!(
            lexemes,
            &[
                Lexeme::new(Token::Var, Pos(1, 1)),
                Lexeme::new_with_literal(Token::Identifier, Pos(1, 5), String::from("x")),
                Lexeme::new(Token::Uint64, Pos(1, 7)),
                Lexeme::new(Token::Equal, Pos(1, 14)),
                Lexeme::new_with_literal(Token::IntLiteral, Pos(1, 16), String::from("100")),
                Lexeme::new(Token::Semicolon, Pos(1, 19)),
                Lexeme::new_with_literal(Token::Identifier, Pos(1, 21), String::from("y")),
                Lexeme::new(Token::ColonEqual, Pos(1, 23)),
                Lexeme::new_with_literal(Token::StringLiteral, Pos(1, 26), String::from("str")),
                Lexeme::new(Token::Eof, Pos(1, 29)),
            ]
        );
    }

    #[test]
    fn test_err_lex() {
        let mut lexer = Lexer::new(str::to_string("y := \"str"));
        let (lexemes, errs) = lexer.lex();
        assert_eq!(errs, &[LexerError::UnterminatedString(Pos(1, 6))]);
        assert_eq!(
            lexemes,
            &[
                Lexeme::new_with_literal(Token::Identifier, Pos(1, 1), String::from("y")),
                Lexeme::new(Token::ColonEqual, Pos(1, 3)),
                Lexeme::new(Token::Eof, Pos(1, 6)),
            ]
        );

        let mut lexer = Lexer::new(str::to_string("/* comment"));
        let (lexemes, errs) = lexer.lex();
        assert_eq!(errs, &[LexerError::UnclosedComment(Pos(1, 1))]);
        assert_eq!(lexemes, &[Lexeme::new(Token::Eof, Pos(1, 1)),]);
    }
}
