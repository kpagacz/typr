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
            // --- Specific Identifiers / Keywords with Lookahead (Must come BEFORE generic identifiers) ---
            'r' if self.first() == '#' && self.second() == '"' => {
                self.bump(); // eat '#'
                self.bump(); // eat '"'
                while !self.is_eof() {
                    if self.first() == '"' && self.second() == '#' {
                        self.bump();
                        self.bump();
                        break;
                    }
                    self.bump();
                }
                SyntaxKind::STRING
            }
            'a' if self.first() == 's' && self.second() == '!' => {
                self.bump(); // eat 's'
                self.bump(); // eat '!'
                SyntaxKind::AS_EXCL
            }

            // --- Dynamic tokens ---
            c if is_ident_start(c) => {
                self.eat_while(is_ident_continue);
                SyntaxKind::IDENT
            }
            '`' => {
                self.eat_while(|c| c != '`');
                self.bump();
                SyntaxKind::IDENT
            }
            c if c.is_ascii_digit() => {
                self.eat_while(|c| c.is_ascii_digit());

                if self.first() == '.' && self.second().is_ascii_digit() {
                    self.bump(); // consume the '.'
                    self.eat_while(|c| c.is_ascii_digit()); // consume the trailing digits
                }

                SyntaxKind::NUMBER
            }
            '"' | '\'' => {
                let quote_type = first_char;
                while !self.is_eof() {
                    let c = self.bump().unwrap();
                    if c == '\\' {
                        self.bump();
                    } else if c == quote_type {
                        break;
                    }
                }
                SyntaxKind::STRING
            }

            // --- Operators ---
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

            // % Operator Logic
            '%' if self.first() == '%' => {
                self.bump();
                SyntaxKind::MODULO2
            }
            '%' => {
                let lookahead = self.src.clone();
                let mut found_closing_percent = false;

                for c in lookahead {
                    if c == '%' {
                        found_closing_percent = true;
                    }
                    if c == '%' || c.is_whitespace() {
                        break;
                    }
                }

                if found_closing_percent {
                    self.eat_while(|c| c != '%');
                    self.bump();
                    SyntaxKind::CUSTOM
                } else {
                    SyntaxKind::MODULO
                }
            }

            // | Operator Logic
            '|' if self.first() == '>' && self.second() == '>' => {
                self.bump();
                self.bump();
                SyntaxKind::PIPE2
            }
            '|' if self.first() == '>' => {
                self.bump();
                SyntaxKind::PIPE
            }
            '|' if self.first() == '|' => {
                self.bump();
                SyntaxKind::OR2
            }
            '|' => SyntaxKind::OR,

            '$' if self.first() == '$' => {
                self.bump();
                SyntaxKind::DOLLAR2
            }
            '$' => SyntaxKind::DOLLAR,

            '=' if self.first() == '=' => {
                self.bump(); // BUG FIX: Don't forget to consume the second '='!
                SyntaxKind::EQ2
            }
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
            '!' => SyntaxKind::EXCLAMATION,

            '<' if self.first() == '=' => {
                self.bump();
                SyntaxKind::LESSER_OR_EQUAL
            }
            '<' if self.first() == '-' => {
                self.bump();
                SyntaxKind::L_ARROW
            }
            '<' => SyntaxKind::LESSER_THAN,

            '>' if self.first() == '=' => {
                self.bump();
                SyntaxKind::GREATER_OR_EQUAL
            }
            '>' => SyntaxKind::GREATER_THAN,

            '&' if self.first() == '&' => {
                self.bump();
                SyntaxKind::AND2
            }
            '&' => SyntaxKind::AND,

            // --- Punctuation ---
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
            '^' => SyntaxKind::CARET,
            '?' => SyntaxKind::QUESTION_MARK,
            '_' => SyntaxKind::UNDERSCORE,
            '\\' => SyntaxKind::BACKSLASH,

            // --- Trivia ---
            c if c.is_whitespace() => {
                self.eat_while(char::is_whitespace);
                SyntaxKind::WHITESPACE
            }
            '#' => {
                self.eat_while(|c| c != '\n');
                SyntaxKind::COMMENT
            }

            // --- Special ---
            EOF_CHAR => SyntaxKind::EOF,
            _ => SyntaxKind::ERROR,
        }
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex_all(input: &str) -> Vec<(SyntaxKind, &str)> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        let mut offset = 0;

        loop {
            let token = lexer.next_token();
            if token.kind == SyntaxKind::EOF {
                break;
            }

            let text = &input[offset..offset + token.len]; // len is usize here, so no cast needed
            tokens.push((token.kind, text));
            offset += token.len;
        }
        tokens
    }

    #[test]
    fn test_custom_operators() {
        // Normal modulo
        assert_eq!(lex_all("5 % 2")[2], (SyntaxKind::MODULO, "%"));
        // Modulo 2
        assert_eq!(lex_all("%%")[0], (SyntaxKind::MODULO2, "%%"));
        // Custom Operator
        assert_eq!(lex_all("%in%")[0], (SyntaxKind::CUSTOM, "%in%"));
    }

    #[test]
    fn test_pipe_operators() {
        assert_eq!(lex_all("|>")[0], (SyntaxKind::PIPE, "|>"));
        assert_eq!(lex_all("|>>")[0], (SyntaxKind::PIPE2, "|>>"));
    }

    #[test]
    fn test_numbers() {
        assert_eq!(lex_all("42")[0], (SyntaxKind::NUMBER, "42"));
        assert_eq!(lex_all("12.34")[0], (SyntaxKind::NUMBER, "12.34"));

        // Ensure method calls on numbers don't swallow the dot
        let tokens = lex_all("1.to_string");
        assert_eq!(tokens[0], (SyntaxKind::NUMBER, "1"));
        assert_eq!(tokens[1], (SyntaxKind::DOT, "."));
        assert_eq!(tokens[2], (SyntaxKind::IDENT, "to_string"));
    }

    #[test]
    fn test_strings() {
        assert_eq!(lex_all("\"hello\"")[0], (SyntaxKind::STRING, "\"hello\""));
        assert_eq!(lex_all("'world'")[0], (SyntaxKind::STRING, "'world'"));
        assert_eq!(
            lex_all("r#\"hello world\"#")[0],
            (SyntaxKind::STRING, "r#\"hello world\"#")
        );
    }

    #[test]
    fn test_as_excl() {
        assert_eq!(lex_all("as!")[0], (SyntaxKind::AS_EXCL, "as!"));
        assert_eq!(lex_all("astronaut")[0], (SyntaxKind::IDENT, "astronaut"));
    }
}
