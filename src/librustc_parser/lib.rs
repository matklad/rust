macro_rules! T {
    (_) => (SyntaxKind::Underscore);
}


pub fn parse_rename(source: &mut dyn Iterator<Item=Token<'_>> ,sink: &mut dyn FnMut(Event)) {
    let tok = match source.next() {
        Some(it) => it,
        None => return
    };
    if !tok.is_keyword("as") {
        return;
    }
    sink(Event::Start(SyntaxKind::Rename));
    sink(Event::Advance(Advance::Remap(SyntaxKind::AsKw)));

    if tok.kind == T![_] || tok.is_ident() {
        sink(Event::Advance(Advance::Bump));
    } else {
        sink(Event::Error);
    }
    sink(Event::Finish(SyntaxKind::Rename));

}

pub struct Token<'a> {
    pub kind: SyntaxKind,
    pub text: &'a str,
}

pub enum Event {
    Start(SyntaxKind),
    Advance(Advance),
    Finish(SyntaxKind),
    Error,
}

pub enum Advance {
    Remap(SyntaxKind),
    Bump,
}

#[derive(Eq, PartialEq)]
pub enum SyntaxKind {
    // Tokens
    AsKw,
    Ident,
    Underscore,
    // Nodes
    Rename
}

impl Token<'_> {
    fn is_keyword(&self, kw: &str) -> bool {
        self.kind == SyntaxKind::Ident && self.text == kw
    }
    fn is_ident(&self) -> bool {
        self.kind == SyntaxKind::Ident
    }
}
