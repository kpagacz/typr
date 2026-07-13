use crate::components::language::syntax::syntax_kind::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TyprLanguage {}

impl rowan::Language for TyprLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        // This is safe because rowan guarantees it only ever
        // passes back the u16s it was given when building
        // the tree.
        unsafe { std::mem::transmute(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<TyprLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<TyprLanguage>;
pub type SyntaxElement = rowan::SyntaxElement<TyprLanguage>;
