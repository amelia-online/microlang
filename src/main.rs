use std::io::{Read};


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
	Let,
	Const,
	Ret,
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
	pub operator_start: String,
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
			operator_start: String::from("+=-*/%<>^&!"),
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

	pub fn has_next(&self) -> bool {
		self.index + 1 < self.input.len()
	}

	pub fn get_at(&self, idx: usize) -> Result<char, ()> {
		if idx < self.input.len()-1 {
			Ok(self.input[idx])
		} else {
			Err(())
		}
	}

	pub fn peek(&self) -> Option<char> {
		if self.has_next() {
			Some(self.input[self.index + 1])
		} else {
			None
		}
	}

	pub fn lex_keyword_or_ident(&mut self) -> Token {

		use TokenType::*;
		let start = self.now();
		let mut valid: String = String::new();
		valid.push_str(&self.digits);
		valid.push_str(&self.ident_start);

		//println!("{valid}");

		let mut value = String::new();

		while self.index < self.input.len() {

			if !valid.contains(self.get()) {
				break;
			}

			value.push(self.get());
			self.inc();
		}

		let tt = match value.as_str() {
			"let" => Let,
			"const" => Const,
			"ret" => Ret,
			"end" => End,
			"lor" => Operator("lor".to_string()),
			"or" => Operator("or".to_string()),
			_ => Ident(value),
		};

		Token::new(tt, self.line, start, self.now())

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

	pub fn lex_string(&mut self) -> Token {
		let mut result = String::new();
		let start = self.now();

		let mut q_count = 0;


		while self.has_next() {

			match self.get() {
				'`' => q_count += 1,
				'\'' => q_count -= 1,
				'\n' => panic!(),
				_ => (),
			}

			result.push(self.get());

			self.inc();

			if q_count == 0 {
				break;
			}
		}

		Token::new(TokenType::Str(result), self.line,start, self.now())
	}

	pub fn lex_operator(&mut self) -> Token {
		let acceptable = vec!["+", "-", "/", "*",
												 "=", "%", ">", "<",
												 "+=", "-=", "*=", "/=",
												 "&=", "|=", "^=", ">=",
												 "<=", ">>=", "<<=", ">>",
												 "<<", "&", "^", "==", "%=",
												 "!=", "!", "&&", "**"];

		let mut result = String::new();

		let start = self.now();

		while self.has_next() {

			if !self.operator_start.contains(self.get()) {
				break;
			}

			result.push(self.get());
			self.inc();
		}

		if !acceptable.contains(&result.as_str()) {
			panic!("{}", format!("Error: unrecognized operator: {}", &result.as_str()));
		}

		Token::new(TokenType::Operator(result), self.line, start, self.now())
		
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
			} else if self.ident_start.contains(ch) {
				result.push(self.lex_keyword_or_ident());
			} else if self.operator_start.contains(ch) {
				
				// Sorry for this.
				if ch == '/' {
					if let Some(n) = self.peek() {
						if n == '/' {
							while self.index < self.input.len() && self.get() != '\n' {
								self.inc();
							}
							continue;
						}
					}
				}

				result.push(self.lex_operator());
			} else if ch == '`' {
				result.push(self.lex_string());
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

