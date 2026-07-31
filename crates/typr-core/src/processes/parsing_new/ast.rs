use crate::components::language::syntax::language::{SyntaxNode, SyntaxToken};
use crate::components::language::syntax::syntax_kind::SyntaxKind;

pub trait AstNode {
    fn can_cast(kind: SyntaxKind) -> bool;
    fn cast(node: SyntaxNode) -> Option<Self>
    where
        Self: Sized;
    fn syntax(&self) -> &SyntaxNode;
}

pub struct LiteralExpr(pub SyntaxNode);

impl AstNode for LiteralExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::LITERAL_EXPR
    }
    fn cast(node: SyntaxNode) -> Option<Self> {
        if Self::can_cast(node.kind()) {
            Some(Self(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

pub struct BinaryExpr(pub SyntaxNode);

impl AstNode for BinaryExpr {
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SyntaxKind::BINARY_EXPR
    }
    fn cast(node: SyntaxNode) -> Option<Self> {
        if Self::can_cast(node.kind()) {
            Some(Self(node))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.0
    }
}

impl BinaryExpr {
    pub fn lhs(&self) -> Option<Expr> {
        self.syntax().children().find_map(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.syntax().children().filter_map(Expr::cast).nth(1)
    }

    pub fn op_token(&self) -> Option<SyntaxToken> {
        self.syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| {
                matches!(
                    it.kind(),
                    SyntaxKind::ADD | SyntaxKind::MINUS | SyntaxKind::MUL | SyntaxKind::DIV
                )
            })
    }
}

pub enum Expr {
    Literal(LiteralExpr),
    Binary(BinaryExpr),
}

impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        LiteralExpr::can_cast(kind) || BinaryExpr::can_cast(kind)
    }
    fn cast(node: SyntaxNode) -> Option<Self> {
        if LiteralExpr::can_cast(node.kind()) {
            Some(Expr::Literal(LiteralExpr(node)))
        } else if BinaryExpr::can_cast(node.kind()) {
            Some(Expr::Binary(BinaryExpr(node)))
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::Literal(it) => it.syntax(),
            Expr::Binary(it) => it.syntax(),
        }
    }
}
