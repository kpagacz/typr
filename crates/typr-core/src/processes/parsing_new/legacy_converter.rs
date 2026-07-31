use crate::components::error_message::help_data::HelpData;
use crate::components::language::operators::Op;
use crate::components::language::syntax::syntax_kind::SyntaxKind;
use crate::components::language::Lang;
use crate::processes::parsing_new::ast::{AstNode, BinaryExpr, Expr, LiteralExpr, ParenExpr};

pub trait ToLegacy {
    fn to_legacy(&self, file_name: &str) -> Lang;
}

impl ToLegacy for Expr {
    fn to_legacy(&self, file_name: &str) -> Lang {
        match self {
            Expr::Literal(lit) => lit.to_legacy(file_name),
            Expr::Binary(bin) => bin.to_legacy(file_name),
            Expr::Paren(paren) => paren.to_legacy(file_name),
        }
    }
}

impl ToLegacy for ParenExpr {
    fn to_legacy(&self, file_name: &str) -> Lang {
        let inner = self
            .inner_expr()
            .expect("ParenExpr missing inner expression")
            .to_legacy(file_name);

        // Find the offset of the inner expression, not the parenthesis itself (which matches old parser behavior)
        let inner_offset = self
            .inner_expr()
            .unwrap()
            .syntax()
            .text_range()
            .start()
            .into();
        let help_data = HelpData::new(inner_offset, file_name.to_string());

        Lang::Scope {
            body: vec![inner],
            help_data,
        }
    }
}

impl ToLegacy for LiteralExpr {
    fn to_legacy(&self, file_name: &str) -> Lang {
        let token = self
            .syntax()
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| !matches!(it.kind(), SyntaxKind::WHITESPACE | SyntaxKind::COMMENT))
            .expect("LiteralExpr missing content token");
        let text = token.text().to_string();

        let offset = token.text_range().start().into();
        let help_data = HelpData::new(offset, file_name.to_string());

        match token.kind() {
            SyntaxKind::TRUE_KW => Lang::Bool {
                value: true,
                help_data,
            },
            SyntaxKind::FALSE_KW => Lang::Bool {
                value: false,
                help_data,
            },
            SyntaxKind::STRING => Lang::Char {
                value: text.replace("\"", ""),
                help_data,
            },
            SyntaxKind::NUMBER => {
                if text.contains('.') {
                    Lang::Number {
                        value: text.parse().unwrap(),
                        help_data,
                    }
                } else {
                    Lang::Integer {
                        value: text.parse().unwrap(),
                        help_data,
                    }
                }
            }
            _ => unimplemented!("Literal not handled: {:?}", token.kind()),
        }
    }
}

impl ToLegacy for BinaryExpr {
    fn to_legacy(&self, file_name: &str) -> Lang {
        let lhs = self
            .lhs()
            .expect("BinaryExpr missing lhs")
            .to_legacy(file_name);
        let rhs = self
            .rhs()
            .expect("BinaryExpr missing rhs")
            .to_legacy(file_name);
        let op_token = self.op_token().expect("BinaryExpr missing operator");

        let offset = op_token.text_range().start().into();
        let help_data = HelpData::new(offset, file_name.to_string());

        let operator = match op_token.kind() {
            SyntaxKind::ADD => Op::Add(help_data.clone()),
            SyntaxKind::MINUS => Op::Minus(help_data.clone()),
            SyntaxKind::MUL => Op::Mul(help_data.clone()),
            SyntaxKind::DIV => Op::Div(help_data.clone()),
            _ => unimplemented!("Operator not handled: {:?}", op_token.kind()),
        };

        // Lang::Operator uses outer help_data, which usually matched the start of the expression in nom.
        let expr_offset = self.syntax().text_range().start().into();
        let expr_help_data = HelpData::new(expr_offset, file_name.to_string());

        Lang::Operator {
            operator,
            lhs: Box::new(rhs), // Legacy parser backwards naming: lhs holds the right side
            rhs: Box::new(lhs), // rhs holds the left side
            help_data: expr_help_data,
        }
    }
}
