use crate::chunk::OpCode;
use crate::lexer::lexeme::Pos;
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::vm::CUnitFrame;

pub struct Compiler {}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, src: String) -> CUnitFrame {
        let mut lexer = Lexer::new(src);
        let (lexemes, errors) = lexer.lex(); //FIXME: handle errs

        if !errors.is_empty() {
            for error in errors {
                eprintln!("\x1b[0;31m{}\x1b[0m", error);
            }
            std::process::exit(1);
        }

        let mut parser = Parser::new(lexemes);
        let mut cunit = parser.parse();
        cunit.chunk_mut().write(OpCode::Exit, Pos(0, 0));

        eprintln!("\x1b[0;34m{:#?}\x1b[0m", cunit.chunk());

        CUnitFrame::new(cunit)
    }
}
