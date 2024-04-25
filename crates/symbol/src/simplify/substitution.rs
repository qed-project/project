use std::collections::HashMap;

use log::trace;

use crate::expr::Expression;

impl Expression {
    pub fn substitute_variables(self, var_table: &HashMap<String, Expression>) -> Expression {
        trace!("substitution: {} :: {:?}", self, var_table);

        match self {
            Self::Sum(lhs, rhs) => {
                let replaced_lhs = lhs.substitute_variables(var_table);
                let replaced_rhs = rhs.substitute_variables(var_table);
                Self::Sum(Box::new(replaced_lhs), Box::new(replaced_rhs))
            }
            Self::Difference(lhs, rhs) => {
                let replaced_lhs = lhs.substitute_variables(var_table);
                let replaced_rhs = rhs.substitute_variables(var_table);
                Self::Difference(Box::new(replaced_lhs), Box::new(replaced_rhs))
            }
            Self::Product(lhs, rhs) => {
                let replaced_lhs = lhs.substitute_variables(var_table);
                let replaced_rhs = rhs.substitute_variables(var_table);
                Self::Product(Box::new(replaced_lhs), Box::new(replaced_rhs))
            }
            Self::Quotient(lhs, rhs) => {
                let replaced_lhs = lhs.substitute_variables(var_table);
                let replaced_rhs = rhs.substitute_variables(var_table);
                Self::Quotient(Box::new(replaced_lhs), Box::new(replaced_rhs))
            }
            Self::Negation(expr) => {
                let replaced_expr = expr.substitute_variables(var_table);
                Self::Negation(Box::new(replaced_expr))
            }
            Self::Power(base, exp) => {
                let replaced_base = base.substitute_variables(var_table);
                let replaced_exp = exp.substitute_variables(var_table);
                Self::Power(Box::new(replaced_base), Box::new(replaced_exp))
            }
            Self::Variable(var) => {
                var_table.get(&var).unwrap().clone()
            }
            Self::Integer(_)
            | Self::RationalNumber(_)
            | Self::ComplexNumber(_) => {
                self
            }
            _ => todo!("replace vars in {self}")
        }
    }
}
