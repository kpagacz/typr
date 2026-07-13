use crate::{
    components::language::syntax::syntax_kind::SyntaxKind, processes::lexing::input::Input,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    Start { kind: SyntaxKind },
    Token,
    Finish,
    Error(String),
}

pub struct Parser<'src> {
    input: &'src Input,
    pos: usize,
    pub events: Vec<Event>,
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src Input) -> Self {
        Self {
            input,
            pos: 0,
            events: Vec::new(),
        }
    }

    pub fn current(&self) -> SyntaxKind {
        self.input.kind(self.pos)
    }

    pub fn nth(&self, n: usize) -> SyntaxKind {
        self.input.kind(self.pos + n)
    }

    pub fn at(&self, kind: SyntaxKind) -> bool {
        self.current() == kind
    }

    pub fn bump(&mut self) {
        if self.current() == SyntaxKind::EOF {
            return;
        }

        self.events.push(Event::Token);
        self.pos += 1;
    }

    pub fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.current() == kind {
            self.bump();
            true
        } else {
            false
        }
    }

    pub fn error(&mut self, message: impl Into<String>) {
        self.events.push(Event::Error(message.into()));
    }

    pub fn start(&mut self) -> Marker {
        let event_pos = self.events.len();
        self.events.push(Event::Start {
            kind: SyntaxKind::ERROR,
        });
        Marker { pos: event_pos }
    }

    pub fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            true
        } else {
            self.error(format!("Expected {:?}", kind));
            false
        }
    }
}

pub struct Marker {
    pos: usize,
}

impl Marker {
    pub fn complete(self, parser: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
        match &mut parser.events[self.pos] {
            Event::Start { kind: slot } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        parser.events.push(Event::Finish);
        CompletedMarker { pos: self.pos }
    }
}

pub struct CompletedMarker {
    pos: usize,
}

impl CompletedMarker {
    pub fn precede(&self, parser: &mut Parser) -> Marker {
        parser.events.insert(
            self.pos,
            Event::Start {
                kind: SyntaxKind::ERROR,
            },
        );
        Marker { pos: self.pos }
    }
}
