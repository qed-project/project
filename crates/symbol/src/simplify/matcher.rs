use std::collections::HashMap;

use crate::expr::Expression;

impl Expression {
    pub fn matches_with_rule(&self, rule: &Expression) -> (bool, HashMap<String, Expression>) {
        let mut var_table = HashMap::new();
    
        match (self, rule) {
            (Expression::Sum(expr_lhs, expr_rhs), Expression::Sum(rule_lhs, rule_rhs))
            | (Expression::Difference(expr_lhs, expr_rhs), Expression::Difference(rule_lhs, rule_rhs))
            | (Expression::Product(expr_lhs, expr_rhs), Expression::Product(rule_lhs, rule_rhs))
            | (Expression::Quotient(expr_lhs, expr_rhs), Expression::Quotient(rule_lhs, rule_rhs)) => {
                let lhs_matches = expr_lhs.matches_with_rule(rule_lhs);
                let rhs_matches = expr_rhs.matches_with_rule(rule_rhs);
    
                if !(lhs_matches.0 && rhs_matches.0) {
                    return (false, var_table);
                }
    
                var_table.extend(lhs_matches.1);
                var_table.extend(rhs_matches.1);
            }
            (wildcard, Expression::Variable(var_name)) => {
                var_table.insert(var_name.clone(), wildcard.clone());
            }
            _ => return (false, var_table),
        }
        
        (true, var_table)
    }
}
