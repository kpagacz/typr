use std::str::Chars;

use crate::components::language::syntax::{SyntaxKind, Token};

pub const EOF_CHAR: char = '\0';

pub struct Lexer<'src> {
    src: Chars<'src>,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self { src: input.chars() }
    }

    pub fn is_eof(&self) -> bool {
        self.src.as_str().is_empty()
    }

    fn first(&self) -> char {
        self.src.clone().next().unwrap_or(EOF_CHAR)
    }

    fn second(&self) -> char {
        let mut it = self.src.clone();
        it.next();
        it.next().unwrap_or(EOF_CHAR)
    }

    fn bump(&mut self) -> Option<char> {
        self.src.next()
    }

    fn eat_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.first()) && !self.is_eof() {
            self.bump();
        }
    }

    pub fn next_token(&mut self) -> Token {
        let bytes_before = self.src.as_str().len();
        let kind = self.advance_token();
        let bytes_after = self.src.as_str().len();

        // Fix: bytes_before is larger than bytes_after!
        let len = bytes_before - bytes_after;

        Token { kind, len }
    }

    fn advance_token(&mut self) -> SyntaxKind {
        let Some(first_char) = self.bump() else {
            return SyntaxKind::EOF;
        };

        match first_char {
            // Dynamic tokens
            c if is_ident_start(c) => {
                self.eat_while(is_ident_continue);
                SyntaxKind::IDENT
            }
            c if c.is_ascii_digit() => {
                self.eat_while(|c| c.is_ascii_digit() || c == '.');
                SyntaxKind::NUMBER
            }
            '"' => {
                self.eat_while(|c| c != '"');
                SyntaxKind::STRING
            }

            // Operators
            '+' if self.first() == '+' => {
                self.bump();
                SyntaxKind::ADD2
            }
            '+' => SyntaxKind::ADD,
            '-' if self.first() == '-' => {
                self.bump();
                SyntaxKind::MINUS2
            }
            '-' if self.first() == '>' => {
                self.bump();
                SyntaxKind::R_ARROW
            }
            '-' => SyntaxKind::MINUS,
            '*' if self.first() == '*' => {
                self.bump();
                SyntaxKind::MUL2
            }
            '*' => SyntaxKind::MUL,
            '/' if self.first() == '/' => {
                self.bump();
                SyntaxKind::DIV2
            }
            '/' => SyntaxKind::DIV,
            '@' if self.first() == '@' => {
                self.bump();
                SyntaxKind::AT2
            }
            '@' if self.first() == '{' => {
                self.bump();
                SyntaxKind::L_VECTORIAL
            }
            '@' => SyntaxKind::AT,
            '%' if self.first() == '%' => {
                self.bump();
                SyntaxKind::MODULO2
            }
            '%' => SyntaxKind::CUSTOM,
            '%' => SyntaxKind::MODULO,
            '|' if self.first() == '>' => {
                self.bump();
                SyntaxKind::PIPE
            }
            '|' if self.first() == '>' && self.second() == '>' => {
                self.bump();
                self.bump();
                SyntaxKind::PIPE2
            }
            '$' if self.first() == '$' => {
                self.bump();
                SyntaxKind::DOLLAR2
            }
            '$' => SyntaxKind::DOLLAR,
            '=' if self.first() == '=' => SyntaxKind::EQ2,
            '=' => SyntaxKind::EQ,
            '.' if self.first() == '.' && self.second() == '.' => {
                self.bump();
                self.bump();
                SyntaxKind::DOT3
            }
            '.' if self.first() == '.' => {
                self.bump();
                SyntaxKind::DOT2
            }
            '.' => SyntaxKind::DOT,
            '!' if self.first() == '=' => {
                self.bump();
                SyntaxKind::NOT_EQ
            }
            '<' if self.first() == '=' => {
                self.bump();
                SyntaxKind::LESSER_OR_EQUAL
            }
            '>' if self.first() == '=' => {
                self.bump();
                SyntaxKind::GREATER_OR_EQUAL
            }
            '<' if self.first() == '-' => {
                self.bump();
                SyntaxKind::L_ARROW
            }
            '<' => SyntaxKind::LESSER_THAN,
            '>' => SyntaxKind::GREATER_THAN,
            '&' if self.first() == '&' => {
                self.bump();
                SyntaxKind::AND2
            }
            '&' => SyntaxKind::AND,
            '|' if self.first() == '|' => {
                self.bump();
                SyntaxKind::OR2
            }
            '|' => SyntaxKind::OR,
            'a' if self.first() == 's' && self.second() == '!' => {
                self.bump();
                self.bump();
                SyntaxKind::AS_EXCL
            }

            // Punctuation
            '{' => SyntaxKind::L_CURLY,
            '}' if self.first() == '@' => {
                self.bump();
                SyntaxKind::R_VECTORIAL
            }
            '}' => SyntaxKind::R_CURLY,
            '[' => SyntaxKind::L_BRACK,
            ']' => SyntaxKind::R_BRACK,
            '(' => SyntaxKind::L_PAREN,
            ')' => SyntaxKind::R_PAREN,
            ';' => SyntaxKind::SEMICOLON,
            ':' if self.first() == ':' => {
                self.bump();
                SyntaxKind::COLON2
            }
            ':' => SyntaxKind::COLON,
            ',' => SyntaxKind::COMMA,
            '!' => SyntaxKind::EXCLAMATION,
            '^' => SyntaxKind::CARET, // ^
            '?' => SyntaxKind::QUESTION_MARK,
            '_' => SyntaxKind::UNDERSCORE,
            '\\' => SyntaxKind::BACKSLASH,

            // Trivia
            c if c.is_whitespace() => {
                self.eat_while(char::is_whitespace);
                SyntaxKind::WHITESPACE
            }
            '#' => {
                self.eat_while(|c| c != '\n');
                SyntaxKind::COMMENT
            }

            // Special
            EOF_CHAR => SyntaxKind::EOF,
            _ => SyntaxKind::ERROR,
        }
    }
}

// Helper functions for Identifiers
fn is_ident_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
