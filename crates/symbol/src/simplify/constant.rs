use log::trace;

use crate::expr::{Expression, ExpressionKind};

impl Expression {
    pub fn simplify_constants(&self) -> Self {
        trace!("simplify_constants");

        match self {
            Self::Integer(_) => self.clone(),
            Self::RationalNumber(_) => self.clone(), // TODO: convert to integer if possible
            Self::ComplexNumber(_) => self.clone(), // TODO: convert to rational if imaginary part is zero
            Self::Variable(_) => self.clone(),
            Self::Sum(lhs, rhs) => {
                match (lhs.kind(), rhs.kind()) {
                    (ExpressionKind::Number(n1), ExpressionKind::Number(n2)) => {
                        Expression::ComplexNumber(n1 + n2)
                    }
                    _ => self.clone()
                }
            },
            _ => self.clone()
        }
    }
}
