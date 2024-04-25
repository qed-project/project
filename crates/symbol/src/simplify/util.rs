use crate::expr::Expression;

impl Expression {
    pub fn get_max_depth(&self) -> usize {
        match self {
            Expression::Integer(_)
            | Expression::RationalNumber(_)
            | Expression::ComplexNumber(_)
            | Expression::Variable(_) => 1,
            Expression::Sum(lhs, rhs)
            | Expression::Difference(lhs, rhs)
            | Expression::Product(lhs, rhs)
            | Expression::Quotient(lhs, rhs)
            | Expression::Equality(lhs, rhs)
            | Expression::Power(lhs, rhs) => {
                let lhs_depth = lhs.get_max_depth();
                let rhs_depth = rhs.get_max_depth();
    
                lhs_depth.max(rhs_depth) + 1
            }
            Expression::Negation(expr) => expr.get_max_depth() + 1,
    
            _ => todo!("get_depth: {self}")
        }
    }
    
}