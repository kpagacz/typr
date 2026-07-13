use crate::{
    components::language::syntax::syntax_kind::SyntaxKind,
    processes::parsing_new::parser::{CompletedMarker, Parser},
};

pub fn parse_expr(p: &mut Parser) {
    expr_bp(p, 0);
}

fn expr_bp(p: &mut Parser, min_bp: u8) {
    let Some(mut lhs) = parse_lhs(p) else {
        return;
    };
    loop {
        let Some((left_bp, right_bp)) = infix_binding_power(p.current()) else {
            break;
        };

        // If the operator has lower binding power than the current context, stop parsing
        if left_bp < min_bp {
            break;
        }

        // We wrap the LHS in the new node binary expression node
        // which happens at the end of the loop,
        // because we already know that LHS is part of
        // a binary expression because of the previous lines.
        // The binary expression node precedes the LHS node
        let marker = lhs.precede(p);

        p.bump(); // Consume the operator
        expr_bp(p, right_bp);

        lhs = marker.complete(p, SyntaxKind::BINARY_EXPR);
    }
}

/// Parses the base elements (Literals, Variables, Prefix operators)
fn parse_lhs(p: &mut Parser) -> Option<CompletedMarker> {
    match p.current() {
        SyntaxKind::NUMBER
        | SyntaxKind::STRING
        | SyntaxKind::TRUE_KW
        | SyntaxKind::FALSE_KW
        | SyntaxKind::NULL_KW
        | SyntaxKind::NA_KW
        | SyntaxKind::DOT3 => {
            let m = p.start();
            p.bump();
            Some(m.complete(p, SyntaxKind::LITERAL_EXPR))
        }
        SyntaxKind::IDENT => {
            let m = p.start();
            p.bump();
            Some(m.complete(p, SyntaxKind::IDENT_EXPR))
        }
        _ => {
            p.error("Expected an expression");
            None
        }
    }
}

fn infix_binding_power(kind: SyntaxKind) -> Option<(u8, u8)> {
    match kind {
        SyntaxKind::ADD | SyntaxKind::MINUS => Some((1, 2)),
        SyntaxKind::MUL | SyntaxKind::DIV => Some((3, 4)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::components::language::syntax::language::SyntaxNode;
    use crate::processes::lexing::input::Input;
    use crate::processes::lexing::lexed_str::LexedStr;
    use crate::processes::parsing_new::build_tree::build_tree;
    use crate::processes::parsing_new::grammar::expr::parse_expr;

    #[test]
    fn test_pratt_parser() {
        // We want to see if `*` binds tighter than `+`
        let code = "1 + 2 * 3";
        let lexed = LexedStr::new(code);
        let input = Input::new(&lexed);
        let mut parser = Parser::new(&input);

        let root = parser.start();
        parse_expr(&mut parser);
        root.complete(&mut parser, SyntaxKind::SOURCE_FILE);
        let tree = build_tree(&lexed, parser.events);
        let typed_node = SyntaxNode::new_root(tree);

        println!("{:#?}", typed_node);
    }
}
