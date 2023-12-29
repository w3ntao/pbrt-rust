use crate::pbrt::*;

#[derive(Clone, Debug, PartialEq)]
enum ParserToken {
    Illegal,
    EOF,
    RightBracket,
    WorldBegin,
    AttributeBegin,
    AttributeEnd,

    Keyword(String),
    Number(String),
    String(String),
    Variable((String, String)),
    List(Vec<String>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    AttributeBegin,
    AttributeEnd,
    WorldBegin,

    Keyword(String),
    Number(String),
    String(String),
    Bool(bool),
    Variable((String, String)),
    List(Vec<String>),
}

impl Token {
    pub fn convert_to_float(&self) -> f64 {
        return match self {
            Token::Number(num) => match num.parse::<f64>() {
                Ok(x) => x,
                Err(_) => {
                    panic!("fail to convert {:?} into f64", self);
                }
            },
            _ => {
                panic!("expect Token::String, get {:?}", self);
            }
        };
    }

    pub fn convert_to_string(&self) -> String {
        return match self {
            Token::String(str) => str.clone(),
            _ => {
                panic!("expect Token::String, get {:?}", self);
            }
        };
    }
}

impl From<ParserToken> for Token {
    fn from(value: ParserToken) -> Self {
        return match value {
            ParserToken::AttributeBegin => Token::AttributeBegin,
            ParserToken::AttributeEnd => Token::AttributeEnd,
            ParserToken::WorldBegin => Token::WorldBegin,

            ParserToken::Keyword(kw) => Token::Keyword(kw),

            ParserToken::Number(num) => Token::Number(num),
            ParserToken::String(str) => Token::String(str),
            ParserToken::Variable(var) => Token::Variable(var),
            ParserToken::List(ls) => Token::List(ls),

            _ => {
                panic!("{:?} can't convert to Token", value);
            }
        };
    }
}
fn parse_identifier(identifier: &str) -> ParserToken {
    return match identifier {
        "WorldBegin" => ParserToken::WorldBegin,
        "AttributeBegin" => ParserToken::AttributeBegin,
        "AttributeEnd" => ParserToken::AttributeEnd,
        "false" | "true" => ParserToken::String(identifier.to_string()),
        _ => ParserToken::Keyword(identifier.to_string()),
    };
}

pub fn parse_pbrt_into_token(file_path: &str) -> Vec<Token> {
    let mut lexer = {
        let binding = match read_to_string(file_path) {
            Ok(content) => content,
            Err(msg) => {
                panic!("fail to read file `{}`:\n`{}`", file_path, msg)
            }
        };
        let input = binding.as_str();
        Lexer::new(input)
    };

    let mut tokens = vec![];

    lexer.read_char();
    loop {
        let token = lexer.next_token();
        if token == ParserToken::EOF {
            break;
        }
        if token == ParserToken::Illegal {
            panic!(
                "\nparsing {} fails at line {}",
                file_path, lexer.line_number
            );
        }

        tokens.push(token);
    }

    return tokens
        .iter()
        .map(|t| Token::from(t.clone()))
        .collect::<Vec<Token>>();
}

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub current_char: Option<char>,
    pub line_number: usize,
}

fn is_letter(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
}

fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            current_char: None,
            line_number: 1,
        }
    }

    pub fn read_char(&mut self) {
        self.current_char = if self.position >= self.input.len() {
            None
        } else {
            Some(self.input[self.position])
        };

        self.position += 1;
    }

    fn skip_space(&mut self) {
        loop {
            match self.current_char {
                None => {
                    return;
                }
                Some(ch) => {
                    if ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r' {
                        if ch == '\n' {
                            self.line_number += 1;
                        }
                        self.read_char();
                        continue;
                    }
                    return;
                }
            }
        }
    }

    fn skip_comment(&mut self) {
        loop {
            self.read_char();
            if self.current_char == Some('\n') {
                self.line_number += 1;
                return;
            }
        }
    }

    fn read_number(&mut self) -> String {
        let last_position = self.position - 1;

        loop {
            match self.current_char {
                None => {
                    break;
                }
                Some(ch) => {
                    if ch == '-' || ch == '.' || is_digit(ch) {
                        self.read_char();
                        continue;
                    }
                    break;
                }
            };
        }

        return self.input[last_position..(self.position - 1)]
            .iter()
            .collect();
    }

    fn read_identifier(&mut self) -> String {
        let last_position = self.position - 1;
        while self.current_char.is_some() && is_letter(self.current_char.unwrap()) {
            self.read_char();
        }

        return self.input[last_position..(self.position - 1)]
            .iter()
            .collect();
    }

    fn read_quoted_string(&mut self) -> ParserToken {
        let last_position = self.position - 1;

        self.read_char(); // consume the first quote
        while self.current_char != Some('"') {
            self.read_char();
        }
        self.read_char(); // consume the last quote

        let string_without_quote: String = self.input[(last_position + 1)..(self.position - 2)]
            .iter()
            .collect();

        if !string_without_quote.contains(' ') {
            return ParserToken::String(string_without_quote);
        }

        let split_char = string_without_quote.split(' ');
        let mut parts = vec![];
        for p in split_char {
            parts.push(p.to_string());
        }
        assert_eq!(parts.len(), 2);
        return ParserToken::Variable((parts[0].clone(), parts[1].clone()));
    }

    fn read_list(&mut self) -> Vec<String> {
        self.read_char(); // consume `[`

        let mut list = vec![];
        loop {
            let token = self.next_token();
            let val = match token {
                ParserToken::RightBracket => {
                    break;
                }
                ParserToken::Number(num) => num,
                ParserToken::String(str) => str,

                _ => {
                    panic!("line {}: get {:?}", self.line_number, token);
                }
            };

            list.push(val);
        }

        return list;
    }

    fn next_token(&mut self) -> ParserToken {
        self.skip_space();
        while self.current_char == Some('#') {
            self.skip_comment();
            self.skip_space();
        }

        let tok = match self.current_char {
            None => ParserToken::EOF,
            Some(current_char) => match current_char {
                '"' => self.read_quoted_string(),
                '[' => ParserToken::List(self.read_list()),
                ']' => ParserToken::RightBracket,
                _ => {
                    if is_letter(current_char) {
                        return parse_identifier(&self.read_identifier());
                    }

                    if current_char == '-' || is_digit(current_char) {
                        return ParserToken::Number(self.read_number());
                    }

                    println!(
                        "line {}: illegal char: `{}`",
                        self.line_number, current_char
                    );

                    return ParserToken::Illegal;
                }
            },
        };
        self.read_char();
        return tok;
    }
}
