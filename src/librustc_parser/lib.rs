fn parse_rename(source: &mut dyn Iterator<Item=Token> ,sink: &mut dyn FnMut(Event)) {

}

struct Token {
    kind: SyntaxKind,
}

enum Event {
    Start(SyntaxKind),
    Advance(Advance),
    Finish(SyntaxKind),
}

struct Advance;

enum SyntaxKind {
    AS_KW,
    IDENT,
}
