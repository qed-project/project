use crate::expr::Expression;

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Integer(int) => write!(f, "{}", int),
            Expression::Variable(var) => write!(f, "{}", var),
            Expression::Sum(lhs, rhs) => write!(f, "({}+{})", lhs, rhs),
            Expression::Difference(lhs, rhs) => write!(f, "({}-{})", lhs, rhs),
            Expression::Product(lhs, rhs) => write!(f, "({}*{})", lhs, rhs),
            Expression::Quotient(lhs, rhs) => write!(f, "({}/{})", lhs, rhs),
            Expression::Equality(lhs, rhs) => write!(f, "{}={}", lhs, rhs),
            Expression::Negation(expr) => write!(f, "(-{})", expr),
            _ => todo!("expression display: {:?}", self)
        }
    }
}
