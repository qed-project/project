use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{expr::Expression, rule::{load_rules_with_signatures, Rule}, signature::Signature};

static RULES: Lazy<HashMap<Signature, Rule>> = Lazy::new(|| load_rules_with_signatures().unwrap()); 

pub fn simplify(expr: &Expression) -> Expression {
    // step 0.5: return if expression is already simple enough
    match expr {
        Expression::Integer(_)
        | Expression::RationalNumber(_)
        | Expression::ComplexNumber(_)
        | Expression::Variable(_) => return expr.clone(),
        _ => {}
    }

    // step 1: 
    let signature = Signature::from(expr);
    let mut intermediate: Vec<(Signature, Expression)> = vec![(signature, expr.clone())];

    // step 2: check if rules can be applied
    for (_signature, rule) in RULES.iter() {
        // step 2.1: check if the top level expression
        // of the rule is an equality and destruct it
        let Expression::Equality(lhs, rhs) = &rule.expr else {
            continue;
        };

        // step 2.2: check if the expression matches with
        // the left hand side of the rule
        let (matches, var_table) = expr_matches(expr, lhs);
        println!("rule '{}': matches={}", rule.name, matches);

        // step 2.3: apply the rule if it matches
        if matches {
            let new_expr = replace_variables_with_expressions(*rhs.clone(), &var_table);
            println!("new expression: {}", new_expr);
            let new_expr_signature = Signature::from(&new_expr);
            intermediate.push((new_expr_signature, new_expr));
        }
    }
    
    // if no rule could be applied to the top-level
    // expression, try to apply rules to the sub-expressions
    if intermediate.len() == 1 {
        println!("no rule could be applied (to the top-level expression)!");
        // match intermediate.first().unwrap().1 {
        //     _ => {}
        // }
    }

    let mut best_intermediate = intermediate.first()
        .expect("cannot fail, because the inital expression is always in this vector");

    for current_intermediate in &intermediate {
        // compare depth of current and (current) best expressions
        #[allow(clippy::comparison_chain)]
        if best_intermediate.0.depth > current_intermediate.0.depth {
            best_intermediate = current_intermediate;
        } else if best_intermediate.0.depth == current_intermediate.0.depth {
            // TODO: check which expression is semantically prefered
            best_intermediate = current_intermediate;
        }
        // else: skip this because it's depth
        // is bigger than the current best depth
    }
    
    best_intermediate.1.to_owned()
}

fn expr_matches(expr1: &Expression, rule: &Expression) -> (bool, HashMap<String, Expression>) {
    let mut var_table = HashMap::new();

    match (expr1, rule) {
        (Expression::Sum(expr_lhs, expr_rhs), Expression::Sum(rule_lhs, rule_rhs))
        | (Expression::Difference(expr_lhs, expr_rhs), Expression::Difference(rule_lhs, rule_rhs))
        | (Expression::Product(expr_lhs, expr_rhs), Expression::Product(rule_lhs, rule_rhs))
        | (Expression::Quotient(expr_lhs, expr_rhs), Expression::Quotient(rule_lhs, rule_rhs)) => {
            let lhs_matches = expr_matches(expr_lhs, rule_lhs);
            let rhs_matches = expr_matches(expr_rhs, rule_rhs);

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

fn replace_variables_with_expressions(expr: Expression, var_table: &HashMap<String, Expression>) -> Expression {
    match expr {
        Expression::Sum(lhs, rhs) => {
            let replaced_lhs = replace_variables_with_expressions(*lhs.clone(), var_table);
            let replaced_rhs = replace_variables_with_expressions(*rhs.clone(), var_table);
            Expression::Sum(Box::new(replaced_lhs), Box::new(replaced_rhs))
        }
        Expression::Difference(lhs, rhs) => {
            let replaced_lhs = replace_variables_with_expressions(*lhs.clone(), var_table);
            let replaced_rhs = replace_variables_with_expressions(*rhs.clone(), var_table);
            Expression::Difference(Box::new(replaced_lhs), Box::new(replaced_rhs))
        }
        Expression::Product(lhs, rhs) => {
            let replaced_lhs = replace_variables_with_expressions(*lhs.clone(), var_table);
            let replaced_rhs = replace_variables_with_expressions(*rhs.clone(), var_table);
            Expression::Product(Box::new(replaced_lhs), Box::new(replaced_rhs))
        }
        Expression::Quotient(lhs, rhs) => {
            let replaced_lhs = replace_variables_with_expressions(*lhs.clone(), var_table);
            let replaced_rhs = replace_variables_with_expressions(*rhs.clone(), var_table);
            Expression::Quotient(Box::new(replaced_lhs), Box::new(replaced_rhs))
        }
        Expression::Variable(var) => {
            var_table.get(&var).unwrap().clone()
        }
        Expression::Integer(_) => {
            expr
        }
        _ => todo!("replace vars in {expr}")
    }
}
