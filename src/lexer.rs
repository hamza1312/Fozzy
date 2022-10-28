use logos::Logos;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Logos)]
#[repr(u16)]
pub enum Token {
    #[regex(r"\d+", priority = 2)]
    Number,

    #[regex(r"\w+")]
    Word,

    #[error]
    Error,
}
