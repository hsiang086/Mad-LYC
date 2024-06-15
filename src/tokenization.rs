use std::process::exit;

pub enum TokenType {
    Exit,
    IntLit,
    Semi,
}

pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct Tokenizer {
    m_src: String,
    m_idx: usize,
}

impl Tokenizer {
    pub fn new(src: String) -> Tokenizer {
        Tokenizer {
            m_src: src,
            m_idx: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut buffer: String = String::new();
        while self.peak(None).is_some() {
            let c: char = self.peak(None).unwrap();
            if c.is_alphabetic() {
                buffer.push(self.consume());
                while self.peak(None).is_some() {
                    if self.peak(None).unwrap().is_alphanumeric() {
                        buffer.push(self.consume());
                    } else {
                        break;
                    }
                }
                if buffer == "exit" {
                    tokens.push(Token {
                        token_type: TokenType::Exit,
                        value: buffer.clone(),
                    });
                    buffer.clear();
                    continue;
                } else {
                    println!("bad");
                    exit(-1);
                }
            } else if c.is_digit(10) {
                buffer.push(self.consume());
                while self.peak(None).is_some() {
                    if self.peak(None).unwrap().is_digit(10) {
                        buffer.push(self.consume());
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    token_type: TokenType::IntLit,
                    value: buffer.clone(),
                });
                buffer.clear();
                continue;
            } else if c == ';' {
                tokens.push(Token {
                    token_type: TokenType::Semi,
                    value: ";".to_string(),
                });
                self.consume();
                continue;
            } else if c.is_whitespace() {
                self.consume();
                continue;
            } else {
                println!("bad");
                exit(-1);
            }
        }
        self.m_idx = 0;
        
        tokens
    }

    fn peak(&self, ahead: Option<usize>) -> Option<char> {
        let ahead: usize = ahead.unwrap_or(1);
        if self.m_idx + ahead > self.m_src.len() {
            None
        } else {
            Some(self.m_src.chars().nth(self.m_idx).unwrap())
        }
    }

    fn consume(&mut self) -> char {
        let temp: usize = self.m_idx;
        self.m_idx += 1;
        self.m_src.chars().nth(temp).unwrap()
    }
}
