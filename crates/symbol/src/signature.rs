
use crate::expr::Expression;

/// Signature of an expression for easier and
/// faster pattern matching
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Signature {
    /// longest path to a leaf expression
    pub depth: usize,
    /// first type of expression
    pub root: String,
    /// the expression itself
    pub expr: Expression
}

impl From<Expression> for Signature {
    fn from(value: Expression) -> Self {
        Self::from_expr(&value)
    }
}

impl From<&Expression> for Signature {
    fn from(value: &Expression) -> Self {
        Self::from_expr(value)
    }
}

impl Signature {
    fn from_expr(expr: &Expression) -> Self {
        let expr = expr.to_owned();

        let root = match expr {
            Expression::Integer(_) => "N",
            Expression::RationalNumber(_) => "N",
            Expression::ComplexNumber(_) => "N",
            Expression::Variable(_) => "V",
            Expression::Sum(_, _) => "+",
            Expression::Difference(_, _) => "-",
            Expression::Product(_, _) => "*",
            Expression::Quotient(_, _) => "/",
            Expression::Equality(_, _) => "=",
            _ => todo!("{expr}")
        }.to_string();

        let depth = get_depth(&expr);

        Self { depth, root, expr }
    }
}

fn get_depth(expr: &Expression) -> usize {
    match expr {
        Expression::Integer(_)
        | Expression::RationalNumber(_)
        | Expression::ComplexNumber(_)
        | Expression::Variable(_) => 1,
        Expression::Sum(lhs, rhs)
        | Expression::Difference(lhs, rhs)
        | Expression::Product(lhs, rhs)
        | Expression::Quotient(lhs, rhs)
        | Expression::Equality(lhs, rhs) => {
            let lhs_depth = get_depth(lhs);
            let rhs_depth = get_depth(rhs);

            lhs_depth.max(rhs_depth) + 1
        }

        _ => todo!("get_depth: {expr}")
    }
}
