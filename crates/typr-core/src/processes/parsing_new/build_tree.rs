use rowan::{GreenNode, GreenNodeBuilder};

use crate::{
    components::language::syntax::syntax_kind::SyntaxKind,
    processes::{lexing::lexed_str::LexedStr, parsing_new::parser::Event},
};

pub fn build_tree(lexed: &LexedStr, events: Vec<Event>) -> GreenNode {
    let mut builder = GreenNodeBuilder::new();
    let mut lex_idx = 0;

    let eat_trivia = |builder: &mut GreenNodeBuilder, lex_idx: &mut usize| {
        while *lex_idx < lexed.len() {
            let kind = lexed.kind(*lex_idx);
            if kind == SyntaxKind::WHITESPACE || kind == SyntaxKind::COMMENT {
                let text = lexed.text_for_token(*lex_idx);
                builder.token(rowan::SyntaxKind(kind as u16), text);
                *lex_idx += 1;
            } else {
                break;
            }
        }
    };

    for event in events {
        match event {
            Event::Start { kind } => {
                builder.start_node(rowan::SyntaxKind(kind as u16));
            }
            Event::Token => {
                eat_trivia(&mut builder, &mut lex_idx);
                let kind = lexed.kind(lex_idx);
                let text = lexed.text_for_token(lex_idx);
                builder.token(rowan::SyntaxKind(kind as u16), text);
                lex_idx += 1;
            }
            Event::Finish => builder.finish_node(),
            Event::Error(_) => todo!(),
        }
    }

    eat_trivia(&mut builder, &mut lex_idx);
    builder.finish()
}
