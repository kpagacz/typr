use crate::{components::language::syntax::SyntaxKind, processes::lexing::lexed_str::LexedStr};

pub struct Input {
    pub tokens: Vec<SyntaxKind>,
}

impl Input {
    pub fn new(lexed_str: &LexedStr) -> Self {
        let mut tokens = lexed_str.kinds.clone();
        tokens.retain(|kind| *kind != SyntaxKind::WHITESPACE && *kind != SyntaxKind::COMMENT);
        Self { tokens }
    }

    pub fn kind(&self, index: usize) -> SyntaxKind {
        self.tokens.get(index).copied().unwrap_or(SyntaxKind::EOF)
    }
}
