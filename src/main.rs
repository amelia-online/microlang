use std::io::{Read, Write};


#[derive(Debug)]
pub enum TokenType {
	Number(String),
	Str(String),
	Newline,
	Question,
	Operator(String),
	Bar,
	End,
	LParen,
	RParen,
	LSquare,
	RSquare,
	LCurly,
	RCurly
}

#[derive(Debug)]
pub struct Token {
	pub tt: TokenType,
	pub line: i32,
	pub start: i32,
	pub end: i32
}

impl Token {
	pub fn new(tt: TokenType, line: i32, start: i32, end: i32) -> Self {
		Self {
			tt,
			line,
			start,
			end
		}
	}
}

pub struct Lexer {
	pub input: String,
	pub index: i32,
	pub line: i32
}	

impl Lexer {
	pub fn new(input: String) -> Self {
		Self {
			input,
			index: 0,
			line: 0,
		}
	} 
}

pub fn read_file(path: String) -> Result<String, ()> {
	if let Ok(mut file) = std::fs::File::open(path) {
		let mut buf = String::new();
		file.read_to_string(&mut buf).expect("");
		Ok(buf)
	} else {
		Err(())
	}	
}

fn main() {
    println!("{:?}", Token::new(TokenType::Newline, 0, 0, 0));
}

