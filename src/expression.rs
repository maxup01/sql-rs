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

    /// Binding power when printed: higher binds tighter
    fn prec(&self) -> u8 {
        match self {
            Expression::Add(_) => 1,
            Expression::Mul(_) => 2,
            Expression::Num(n) if *n < 0 => 2, // -3 prints like -1*3
            Expression::Pow(..) if self.as_denominator().is_some() => 2, // prints as 1/x
            Expression::Pow(..) => 3,
            _ => 4, // Num, Sym, Func: atoms
        }
    }

    /// If the term is negative, return its positive part: -3*x → 3*x, -1*x → x
    fn split_neg(&self) -> Option<Expression> {
        match self {
            Expression::Num(n) if *n < 0 => Some(Expression::Num(-n)),
            Expression::Mul(fs) => match fs.first() {
                Some(Expression::Num(-1)) if fs.len() == 2 => Some(fs[1].clone()),
                Some(Expression::Num(-1)) => Some(Expression::Mul(fs[1..].to_vec())),
                Some(Expression::Num(n)) if *n < 0 => {
                    let mut fs = fs.clone();
                    fs[0] = Expression::Num(-n);
                    Some(Expression::Mul(fs))
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format_expression(self))
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
