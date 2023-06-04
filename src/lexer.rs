#[derive(Debug)]
pub struct LexerToken {
   pub token: String,
   pub symbol_type: SymbolType,
}

#[derive(Debug)]
pub struct Lexer {
   chars: Vec<char>,
   position: usize,
   tokens: Vec<LexerToken>,
}

impl Lexer {
   pub fn new(input: &str) -> Lexer {
      return Lexer {
         chars: input.chars().collect(),
         position: 0,
         tokens: Vec::new(),
      };
   }

   pub fn read(mut self) -> Vec<LexerToken> {
      while let Some(c) = self.chars.get(self.position) {
         if c.is_alphabetic() {
            self.parse_ident();
         } else if c.is_numeric() {
            self.parse_number();
         } else {
            self.parse();
         }
      }

      return self.tokens;
   }

   fn parse(&mut self) {
      let c = self.chars[self.position];

      let token = match c {
         '+' => LexerToken {
            token: c.to_string(),
            symbol_type: SymbolType::Addition,
         },
         '-' => LexerToken {
            token: c.to_string(),
            symbol_type: SymbolType::Subtraction,
         },
         '=' => LexerToken {
            token: c.to_string(),
            symbol_type: SymbolType::Equals,
         },
         '*' => LexerToken {
            token: c.to_string(),
            symbol_type: SymbolType::Multiplication,
         },
         '"' => self.parse_string(),
         '\n' => LexerToken {
            token: c.to_string(),
            symbol_type: SymbolType::NewLine,
         },
         ';' => LexerToken {
            token: c.to_string(),
            symbol_type: SymbolType::EOS,
         },
         ' ' => {
            self.position += 1;
            return;
         }
         _ => todo!("{} not handled", c),
      };

      self.position += 1;
      self.tokens.push(token);
   }

   fn parse_ident(&mut self) {
      let mut ident = "".to_string();

      while let Some(c) = self.chars.get(self.position) {
         if !c.is_alphanumeric() {
            break;
         }
         ident.push(*c);
         self.position += 1;
      }

      self.tokens.push(LexerToken {
         token: ident.to_string(),
         symbol_type: SymbolType::Ident,
      });
   }

   fn parse_number(&mut self) {
      let mut num = "".to_string();
      let mut seen_decimal = false;

      while let Some(c) = self.chars.get(self.position) {
         if *c == '.' && !seen_decimal {
            seen_decimal = true;
         } else if !c.is_numeric() {
            break;
         }
         num.push(*c);
         self.position += 1;
      }

      self.tokens.push(LexerToken {
         token: num.to_string(),
         symbol_type: SymbolType::Integer,
      });
   }

   fn parse_string(&mut self) -> LexerToken {
      let mut string = "".to_string();

      while let Some(c) = self.chars.get(self.position + 1) {
         if *c == '"' {
            break;
         }

         string.push(*c);
         self.position += 1;
      }

      return LexerToken {
         token: string.to_string(),
         symbol_type: SymbolType::String,
      };
   }
}

#[derive(Debug)]
pub enum SymbolType {
   String,
   Addition,
   Subtraction,
   Multiplication,
   Equals,
   NewLine,
   Ident,
   EOS,
   Integer,
}
