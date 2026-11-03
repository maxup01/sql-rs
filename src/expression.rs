use std::boxed::Box;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Expression {
    Num(i64),
    Sym(String),
    Add(Vec<Expression>),
    Mul(Vec<Expression>),
    Func {
        name: String,
        argument: Box<Expression>,
    },
}
