use logos::Logos;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Logos)]
#[repr(u16)]
pub enum SyntaxKind {
    #[regex(r"([0-9]+(\.[0-9]+)?([eE][0-9]+)?)")]
    Number,
    #[token("var")]
    Var,
    #[token("=")]
    Equal,
    #[token("==")]
    EqualTo,
    #[token("!=")]
    NotEqual,
    #[token(">")]
    GreaterThan,
    #[token(">=")]
    GreaterThanOrEqual,
    #[token("<")]
    LessThan,
    #[token("<=")]
    LessThanOrEqual,
    #[token("(")]
    OpeningParenthesis,
    #[token(")")]
    ClosingParenthesis,
    #[token("!")]
    Not,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token(".")]
    Dot,
    #[token("#")]
    Tag,
    #[token("++")]
    Inc,
    #[token("--")]
    Dec,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,
    #[token("&")]
    And,
    #[token("|")]
    Or,
    #[token("^")]
    Xor,
    #[token("%")]
    Mod,
    #[regex(r"[a-zA-Z_$][a-zA-Z\\d_$]*")]
    Identifier,
    #[regex(r#""([^"]|\\(["ntrf\\b/']|x[0-9A-Fa-f]+|[0-7][0-7]?[0-7]?))*""#)]
    String,
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}

#[test]
fn number_test() {
    let mut lex = SyntaxKind::lexer("1.252525225");
    assert_eq!(lex.next(), Some(SyntaxKind::Number));
    println!("{:?}", lex.slice());
}
#[test]
fn word_test() {
    let mut lex = SyntaxKind::lexer("Hi\n\t");
    let next = lex.next();
    assert_eq!(next, Some(SyntaxKind::Identifier));
    println!("{:?}ns", std::time::Instant::now().elapsed().as_nanos())
}
#[test]
fn symbols_test() {
    let mut lex = SyntaxKind::lexer("()!.,+-*/=++--== != <<>>#|&^%");
    assert_eq!(lex.next(), Some(SyntaxKind::OpeningParenthesis));
    assert_eq!(lex.next(), Some(SyntaxKind::ClosingParenthesis));
    assert_eq!(lex.next(), Some(SyntaxKind::Not));
    assert_eq!(lex.next(), Some(SyntaxKind::Dot));
    assert_eq!(lex.next(), Some(SyntaxKind::Comma));
    assert_eq!(lex.next(), Some(SyntaxKind::Plus));
    assert_eq!(lex.next(), Some(SyntaxKind::Minus));
    assert_eq!(lex.next(), Some(SyntaxKind::Asterisk));
    assert_eq!(lex.next(), Some(SyntaxKind::Slash));
    assert_eq!(lex.next(), Some(SyntaxKind::Equal));
    assert_eq!(lex.next(), Some(SyntaxKind::Inc));
    assert_eq!(lex.next(), Some(SyntaxKind::Dec));
    assert_eq!(lex.next(), Some(SyntaxKind::EqualTo));
    assert_eq!(lex.next(), Some(SyntaxKind::NotEqual));
    assert_eq!(lex.next(), Some(SyntaxKind::Shl));
    assert_eq!(lex.next(), Some(SyntaxKind::Shr));
    assert_eq!(lex.next(), Some(SyntaxKind::Tag));
    assert_eq!(lex.next(), Some(SyntaxKind::Or));
    assert_eq!(lex.next(), Some(SyntaxKind::And));
    assert_eq!(lex.next(), Some(SyntaxKind::Xor));
    assert_eq!(lex.next(), Some(SyntaxKind::Mod));
}
#[test]
fn string_test() {
    let mut lex = SyntaxKind::lexer("\"Hello, World\n\t \"");
    assert_eq!(lex.next(), Some(SyntaxKind::String));
    println!("{:?}", lex.slice())
}
