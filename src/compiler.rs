use crate::chunk::{Chunk, OpCode};
use crate::lexer::lexeme::Pos;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub struct Compiler {
    chunk: Chunk,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
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

        if !errors.is_empty() {
            for error in errors {
                eprintln!("\x1b[0;31m{}\x1b[0m", error);
            }
            std::process::exit(1);
        }

        let mut parser = Parser::new(lexemes);
        self.chunk = parser.parse();
        self.chunk.write(OpCode::Return, Pos(0, 0));

        eprintln!("\x1b[0;34m{:#?}\x1b[0m", self.chunk);
        &self.chunk
    }
}
