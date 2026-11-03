use std::boxed::Box;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expression {
    Num(i64),
    Sym(String),
    Add {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Mul {
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Func {
        name: String,
        argument: Box<Expression>,
    },
}
