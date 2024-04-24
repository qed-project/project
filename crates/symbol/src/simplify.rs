use std::collections::HashMap;

use once_cell::sync::Lazy;

use crate::{expr::Expression, rule::{load_rules_with_signatures, Rule}, signature::Signature};

static RULES: Lazy<HashMap<Signature, Rule>> = Lazy::new(|| load_rules_with_signatures().unwrap()); 

pub fn simplify(expr: &Expression) -> Expression {
    let expr_signature = Signature::from(expr);
    
    let look_for = match expr {
        Expression::Equality(_,_) => "=",
        _ => "="
    };

    for (signature, rule) in RULES.iter() {
        // step 1: find all rules which could be applied
        if signature.root == look_for {
            // step 1.5: check if LHS of expr and rule match
            let rule_lhs_signature = Signature::from(&rule.expr);
            
            if rule_lhs_signature != expr_signature {
                continue;
            }
            
            // step 2: apply rule
            let new_expr = rule.apply(expr);

            println!("applied '{}': '{}'", rule.name, new_expr);
        }

        // // step 3: store intermediate value (expression and name of the rule)
        // intermediate_exprs.append((rule.name, new_expr));
    }

    expr.clone()
}
