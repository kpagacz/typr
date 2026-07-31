use nom_locate::LocatedSpan;
use typr_core::processes::parsing::elements::parse_elements;
use typr_core::processes::lexing::lexed_str::LexedStr;
use typr_core::processes::lexing::input::Input;
use typr_core::processes::parsing_new::parser::Parser;
use typr_core::processes::parsing_new::grammar::expr::parse_expr;
use typr_core::processes::parsing_new::build_tree::build_tree;
use typr_core::components::language::syntax::syntax_kind::SyntaxKind;
use typr_core::components::language::syntax::language::SyntaxNode;
use typr_core::processes::parsing_new::ast::{AstNode, Expr};
use typr_core::processes::parsing_new::legacy_converter::ToLegacy;

fn verify_parser_parity(code: &str, file_name: &str) {
    let span = LocatedSpan::new_extra(code, file_name.to_string());
    let (remaining, old_ast) = parse_elements(span).expect("Old parser failed to parse the expression");
    assert!(remaining.fragment().trim().is_empty(), "Old parser did not consume the entire input! Remaining: {}", remaining.fragment());

    let lexed = LexedStr::new(code);
    let input = Input::new(&lexed);
    let mut parser = Parser::new(&input);
    let root = parser.start();
    
    parse_expr(&mut parser);
    root.complete(&mut parser, SyntaxKind::SOURCE_FILE);
    
    let tree = build_tree(&lexed, parser.events.clone());
    let syntax_node = SyntaxNode::new_root(tree);
    
    let expr_node = syntax_node.descendants().find_map(Expr::cast)
        .expect("New parser did not produce a valid Expr");
    
    let new_ast = expr_node.to_legacy(file_name);

    assert_eq!(
        old_ast, new_ast,
        "\n\n PARSER MISMATCH in file: {} \n\nOld AST:\n{:#?}\n\nNew AST:\n{:#?}\n\n",
        file_name, old_ast, new_ast
    );
}

macro_rules! migration_test {
    ($test_name:ident, $file_path:expr) => {
        #[test]
        fn $test_name() {
            let code = include_str!($file_path);
            verify_parser_parity(code, $file_path);
        }
    };
}

migration_test!(test_expr_literal, "parser_migration_cases/expr_literal.ty");
migration_test!(test_expr_binary, "parser_migration_cases/expr_binary.ty");
