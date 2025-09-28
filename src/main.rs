use std::io::{Read, Write};
use std::env::Args;


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
	RCurly,
	Comma,
	Colon,
	Semicolon,
	Dot,
	Tilda,
	Ident(String)
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
	pub input: Vec<char>,
	pub index: usize,
	pub line: i32,
	pub digits: String,
	pub singles: String,
	pub ident_start: String,
}	

impl Lexer {
	pub fn new(input: String) -> Self {
		Self {
			input: input.chars().collect::<Vec<char>>(),
			index: 0,
			line: 1,
			digits: String::from("0123456789"),
			singles: String::from("[]{}():?,|;.~"),
			ident_start: String::from("_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"),
		}
	} 

	pub fn inc(&mut self) {
		self.index += 1;
	}

	pub fn now(&self) -> i32 {
		self.index as i32
	}

	pub fn get(&self) -> char {
		self.input[self.index]
	}

	pub fn get_at(&self, idx: usize) -> Result<char, ()> {
		if idx < self.input.len()-1 {
			Ok(self.input[idx])
		} else {
			Err(())
		}
	}

	pub fn lex_ident_or_keyword(&mut self) -> Token {
		todo!()
	}

	pub fn lex_number(&mut self) -> Token {
		let start = self.now();
		let mut number = String::new();

		while self.index < self.input.len() {
			
			if !self.digits.contains(self.get()) {
				break;
			}

			//println!("{}", self.get());
			
			number.push(self.get());
			self.inc();
		}
		//println!("just lexed: {}", &number);
		Token::new(TokenType::Number(number),self.line, start, self.now())
	}

	pub fn lex(&mut self) -> Vec<Token> {
		use TokenType::*;
		let length = self.input.len();
		
		let mut result: Vec<Token> = Vec::new();
		while self.index < length {
			let ch = self.get();
			//println!("{ch}");


			if ch == ' ' {
				self.inc();
				continue;
			}

			if ch == '\n' {
				result.push(Token::new(Newline, self.line, self.now(), self.now() + 1));
				self.inc();
				self.line += 1;
			} else if self.digits.contains(ch) {
				result.push(self.lex_number());
			} else if self.singles.contains(ch) {
				let tt = match ch {
					']' => RSquare,
					'[' => LSquare,
					'|' => Bar,
					'(' => LParen,
					')' => RParen,
					'{' => LCurly,
					'}' => RCurly,
					',' => Comma,
					'?' => Question,
					':' => Colon,
					';' => Semicolon,
					'.' => Dot,
					'~' => Tilda,
					_ => unreachable!(),
				};
				let token = Token::new(
						tt,
						self.line,
						self.now(),
						self.now() + 1
				);
				self.inc();
				result.push(token);
			}
		}
		result
	}
}

pub fn read_file(path: String) -> Result<String, ()> {
	if let Ok(mut file) = std::fs::File::open(path) {
		let mut buf = String::new();
		file.read_to_string(&mut buf).expect("Failed to read file.");
		Ok(buf)
	} else {
		Err(())
	}	
}

fn main() {
	let args = std::env::args().collect::<Vec<String>>();
	let path = &args[1];
	if let Ok(txt) = read_file(path.to_owned()) {
		let mut lexer = Lexer::new(txt);
		let tokens = lexer.lex();
		for token in tokens {
			println!("{:?}", token);
		}
	} else {
		println!("Error: file path is invalid.");
	}
}

