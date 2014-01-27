///
/// Description: Lex the source code into tokens
///

pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Assignment,
    Num(u16),
    Let,
    If,
    For,
    While,
    Loop,
    Else,
    Fn,
    String(~str),
    Equal,
    Plus,
    PlusEq,
    Minus,
    MinusEq
}

pub struct Lexer<'a> {
    priv remaining: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            remaining: source.trim()
        }
    }
}

impl<'a> Iterator<Token> for Lexer<'a> {
    fn next(&mut self) -> Option<Token> {
        let len = self.remaining.len();
        if len == 0 {
            return None;
        }
        
        let mut token_end = 1;
        let token = match self.remaining.char_at(0) {
            '(' => LeftParen,
            ')' => RightParen,
            '{' => LeftBrace,
            '}' => RightBrace,
            '=' => {
                if len == 1 {
                    Assignment
                }
                else {
                    match self.remaining.char_at(1) {
                        '=' => { token_end += 1; Equal },
                        _   => Assignment
                    }
                }
            },
            '+' => {
                if len == 1 {
                    Plus
                }
                else {
                    match self.remaining.char_at(1) {
                        '=' => { token_end += 1; PlusEq },
                        _   => Plus
                    }
                }
            },
            '-' => {
                if len == 1 {
                    Minus
                }
                else {
                    match self.remaining.char_at(1) {
                        '=' => { token_end += 1; MinusEq },
                        _   => Minus
                    }
                }
            },
            
            '0'..'9' => {
                token_end = scan_token(self.remaining);
                match from_str(self.remaining.slice_to(token_end)) {
                    Some(n) => Num(n),
                    None    => fail!("Invalid number")
                }
            },
            _ => {
                token_end = scan_token(self.remaining);
                match self.remaining.slice_to(token_end) {
                    "let"   => Let,
                    "if"    => If,
                    "for"   => For,
                    "while" => While,
                    "loop"  => Loop,
                    "else"  => Else,
                    "fn"    => Fn,
                    _       => String(self.remaining.slice_to(token_end).to_owned())
                }
            }
        };
        
        self.remaining = self.remaining.slice_from(token_end).trim_left();
        Some(token)
    }
}

/// Scans till the end of the token returning the index of the end of the token
fn scan_token(string: &str) -> uint {
    static TOKEN_BOUNDS: &'static [char] = &[' ', '\t', '\n', '(', ')', '{', '}', '.', '='];
    match string.find(TOKEN_BOUNDS) {
        Some(n) => n,
        None    => string.len()
    }
}