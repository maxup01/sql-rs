use std::boxed::Box;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Num(i64),
    Sym(String),
    Add(Vec<Expression>),
    Mul(Vec<Expression>),
    Func {
        name: String,
        argument: Box<Expression>,
    },
}

impl Expression {
    pub fn parse(expression: &str) -> Result<Self, crate::error::ParseError> {
        Self::parentheses_correct(expression).map_err(|err| {
            crate::error::ParseError::new(crate::error::ParseErrorKind::UnbalancedParentheses, err)
        })?;

        Ok(Expression::Num(0))
    }

    fn parentheses_correct(expression: &str) -> Result<(), String> {
        let mut depth = 0i32;

        for (i, c) in expression.chars().enumerate() {
            match c {
                '(' => depth += 1,
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        return Err(format!("unexpected ')' at position {i}"));
                    }
                }
                _ => {}
            }
        }

        if depth != 0 {
            return Err(format!("{depth} unclosed '('"));
        }

        Ok(())
    }
}

enum Token {
    Num(i64),
    Sym(String),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}
