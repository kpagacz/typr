use crate::{components::language::syntax::SyntaxKind, processes::lexing::lexer::Lexer};

pub struct LexedStr<'src> {
    pub text: &'src str,
    pub kinds: Vec<SyntaxKind>,
    pub starts: Vec<usize>,
}

impl<'src> LexedStr<'src> {
    pub fn new(text: &'src str) -> Self {
        let mut lexer = Lexer::new(text);
        let mut kinds = Vec::new();
        let mut starts = Vec::new();
        let mut offset = 0;

        loop {
            let token = lexer.next_token();
            starts.push(offset);

            let final_kind = if token.kind == SyntaxKind::IDENT {
                let token_text = &text[offset..offset + token.len];
                SyntaxKind::from_keyword(token_text).unwrap_or(SyntaxKind::IDENT)
            } else {
                token.kind
            };
            kinds.push(final_kind);
            offset += token.len;
            if final_kind == SyntaxKind::EOF {
                starts.push(offset);
                break;
            }
        }

        Self {
            text,
            kinds,
            starts,
        }
    }

    pub fn kind(&self, index: usize) -> SyntaxKind {
        self.kinds.get(index).copied().unwrap_or(SyntaxKind::EOF)
    }

    pub fn text_for_token(&self, index: usize) -> &'src str {
        // Safety: `starts` should have enough elements because
        // we pushed an extra offset when the lexer hit the end of the file
        let start = self.starts[index];
        let end = self.starts[index + 1];
        &self.text[start..end]
    }

    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
