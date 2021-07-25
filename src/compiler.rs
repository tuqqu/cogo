use crate::lexer::Lexer;
use crate::chunk::{Chunk, OpCode};
use crate::lexer::lexeme::{Pos, Token, Lexeme};
use std::collections::HashMap;
use std::any::Any;
use crate::value::Value;
use crate::parser::Parser;

pub struct Compiler {
    chunk: Chunk,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            chunk: Chunk::new(),
        }
    }

    pub fn compile(&mut self, src: String) -> &Chunk {
        let mut lexer = Lexer::new(src);
        let (lexemes, errors) = lexer.lex(); //FIXME: handle errs

        let mut parser = Parser::new(lexemes);
        self.chunk = parser.parse();
        self.chunk.write(OpCode::Return, Pos(0, 0));

        eprintln!("\x1b[0;34m{:#?}\x1b[0m", self.chunk);
        &self.chunk
    }
}

