pub mod parser {
    use crate::{Token, TokenType};
    use TokenType::*;

    #[derive(Debug)]
    pub enum BuiltinType {
	I8,
	U8,
	I16,
	U16,
	I32,
	U32,
	I64,
	U64,
    }

    #[derive(Debug)]
    pub enum Exp {
        FnCallExp(String, Vec<Exp>),
        IfExp(Box<Exp>, Vec<Stmt>, Vec<Stmt>),
        ArrayLitExp(Vec<Exp>),
        ArrayIndexExp(Box<Exp>),
        BinopExp(String, Box<Exp>, Box<Exp>),
        
        MatchExp(Box<Exp>) // todo
    }

    #[derive(Debug)]
    pub enum Stmt {
        LetStmt(String, Exp),
        RetStmt(Exp),
        IfStmt(Exp, Vec<Stmt>, Vec<Stmt>),
        MatchStmt(Exp), // todo
        WhileStmt(Exp, Vec<Stmt>),
        ForStmt(Exp, Exp, Exp, Vec<Stmt>),

    }   

    #[derive(Debug)]
    pub enum Cmd {
        FnCmd(String, Vec<Stmt>),
        IncludeCmd(Vec<String>),
        TypeCmd(String), // todo

    }

    #[derive(Debug)]
    pub enum AST {
        Expression(Exp),
        Statement(Stmt),
        Command(Cmd)
    }
    
    pub struct Parser {
        pub tokens: Vec<Token>,
        pub index: usize,
    }

    impl Parser {
        pub fn new(input: Vec<Token>) -> Self {
            Self {
                tokens: input,
                index: 1,
            }
        }

	fn inc(&mut self) {
	    self.index += 1;
	}

	fn has_next(&self) -> bool {
	    self.index+1 < self.tokens.len()-1
	}

	fn get(&self) -> Option<Token> {
	    if self.index < self.tokens.len()-1 {
		Some(self.tokens[self.index])
	    } else {
		None
	    }
	}

	fn peek(&self) -> Option<Token> {
	    if self.has_next() {
		Some(self.tokens[self.index+1])
	    } else {
		None
	    }
	}

	fn current_line(&self) -> i32 {
	    if let Some(token) = self.get() {
		token.line
	    } else {
		self.tokens[self.index-1].line
	    }
	}

	pub fn parse_error(&self, msg: &str, line: i32) {
	    println!("Error: Line {}: {}", line, msg);
	    std::process::exit(1);
	}



	fn parse_let(&mut self) -> Stmt {
	    self.inc(); // We already know the current token is Let
	    if let Some(token) = self.get() {
		match token.tt {
		    Ident(name) => {
			self.inc();
			if let Some(eq_or_type) = self.get() {
			    match eq_or_type.tt {
				Colon => {

				},
				Operator(eq) => {

				},
				_ => self.parse_error(format!("expected '=' or type, got {}", eq_or_type.tt.name()).as_str(), eq_or_type.line),
			    }
			} else {
			     self.parse_error("expected '=' or type, got nothing", self.current_line());
			}
		    },
		    _ => self.parse_error(format!("expected identifer, got {}", token.tt.name()).as_str(), token.line),
		}
	    } else {
		self.parse_error("expected identifier, got nothing.", self.current_line())
	    }
	}

	// Assume token not consumed when entering a parse function
        pub fn parse(&mut self) -> Vec<AST> {
            todo!()
        }
    }




}
