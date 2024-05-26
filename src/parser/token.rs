use std::str::CharIndices;

#[derive(Debug)]
pub enum Token {
    Integer(String),
    Identifier(String),
    Equal,
    Plus,
    Hyphen,
    Asterisk,
    Slash,
    Percent,
    Comma,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBracket,
    ClosingBracket,
}

pub struct Lexer<'id> {
    input: &'id str,
    iter: CharIndices<'id>,
    next_index_char: Option<(usize, char)>,
    pub next_token: Option<Token>,
}

impl<'id> Lexer<'id> {
    pub fn new(input: &'id str) -> Lexer<'id> {
        let mut ret = Lexer {
            input,
            iter: input.char_indices(),
            next_index_char: None,
            next_token: None,
        };
        ret.consume_char();
        ret.consume_token();
        ret
    }
    fn pos(&self) -> usize {
        match self.next_index_char {
            Some((index, _)) => index,
            None => self.input.len(),
        }
    }
    fn consume_char(&mut self) {
        self.next_index_char = self.iter.next();
    }
    fn next_char(&self) -> Option<char> {
        self.next_index_char.map(|(_, ch)| ch)
    }
    pub fn consume_token(&mut self) {
        while self.next_char().is_some_and(|ch| ch.is_ascii_whitespace()) {
            self.consume_char();
        }
        let start = self.pos();
        self.next_token = match self.next_char() {
            None => None,
            Some(first_token) => {
                self.consume_char();
                match first_token {
                    '0'..='9' => {
                        let mut value = first_token.to_string();
                        while let Some(ch) = self.next_char() {
                            match ch {
                                '0'..='9' => value.push(ch),
                                '_' => {}
                                _ => break,
                            }
                            self.consume_char();
                        }
                        Some(Token::Integer(value))
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        while let Some('a'..='z' | 'A'..='Z' | '0'..='9' | '_') = self.next_char() {
                            self.consume_char();
                        }
                        let end = self.pos();
                        match &self.input[start..end] {
                            s => Some(Token::Identifier(s.to_string())),
                        }
                    }
                    '#' => {
                        while self.next_char().is_some() {
                            self.consume_char();
                        }
                        None
                    }
                    '=' => Some(Token::Equal),
                    '+' => Some(Token::Plus),
                    '-' => Some(Token::Hyphen),
                    '*' => Some(Token::Asterisk),
                    '%' => Some(Token::Percent),
                    '/' => Some(Token::Slash),
                    ',' => Some(Token::Comma),
                    '(' => Some(Token::OpeningParenthesis),
                    ')' => Some(Token::ClosingParenthesis),
                    '[' => Some(Token::OpeningBracket),
                    ']' => Some(Token::ClosingBracket),
                    _ => panic!("unexpected character {first_token:?}"),
                }
            }
        }
    }
}
