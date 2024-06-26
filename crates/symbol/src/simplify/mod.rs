mod constant;
mod matcher;
mod substitution;
mod context;
mod util;

use iter_tools::Itertools;
use log::{debug, trace};
use once_cell::sync::Lazy;

use crate::{expr::Expression, rule::{load_rules, Rule}};

use self::context::SimplifyContext;

static RULES: Lazy<Vec<Rule>> = Lazy::new(|| load_rules().unwrap()); 

impl Expression {
    pub fn simplify(&self) -> Expression {
        trace!("simplifying");

        const MAX_STEPS: usize = 4;
        let mut context = SimplifyContext::new(self.clone(), self.get_max_depth(), 2, vec![self.clone()]);
        let mut expressions = self.simplify_step(&mut context, MAX_STEPS);
    
        debug!("\nall modified expressions:\n");
    
        expressions.iter()
            .for_each(|e| println!("- `{e}`"));
    
        println!();
    
        expressions.swap_remove(0)
    }

    pub(crate) fn simplify_step(&self, context: &mut SimplifyContext, max_steps: usize) -> Vec<Expression> {
        trace!("simplify_step");
        
        // step 1) 
        //   simplify all constant expressions
        //   (i.e. expressions with just numbers)
        let expr = self.simplify_constants();
        debug!("simplified expression: {}", expr);
    
        // step 2)
        //   return if expression is already simple enough
        match expr {
            Expression::Integer(_)
            | Expression::RationalNumber(_)
            | Expression::ComplexNumber(_)
            | Expression::Variable(_) => return vec![expr],
            _ => {}
        }
        
        trace!("looping over rules");
        // step 3)
        //   check if rules can be applied
        //   and store each intermediary expression
        let mut intermediate: Vec<Expression> = vec![expr.clone()];
        
        for rule in RULES.iter() {
            // step 2.1: check if the top level expression
            // of the rule is an equality and destruct it
            let Expression::Equality(lhs, rhs) = &rule.expr else {
                continue;
            };
    
            trace!("matching rule");
    
            // step 2.2: check if the expression matches with
            // the left hand side of the rule
            let (matches, var_table) = expr.matches_with_rule(lhs);
            // step 2.3: apply the rule if it matches
            if matches {
                trace!("applying rule from the LHS");
                let new_expr = rhs.clone().substitute_variables(&var_table);
                println!("new expr: {}", new_expr);
                intermediate.push(new_expr);
            }
    
            let (matches, var_table) = expr.matches_with_rule(rhs);
            if matches {
                trace!("applying rule from the RHS");
                let new_expr = lhs.clone().substitute_variables(&var_table);
                println!("new expr: {}", new_expr);
                intermediate.push(new_expr);
            }
        }
    
        for i in &intermediate {
            if !context.generated_expressions.contains(i) {
                context.generated_expressions.push(i.clone())
            }

            if i.get_max_depth() - context.original_depth >= context.max_depth_difference_from_original {
                // TODO: remove i from intermediate
            }
        }

        trace!("finished rule checking");
        
        if max_steps == 1 {
            return intermediate.into_iter()
                .unique()
                .collect();
        }
    
        // TODO: try to apply rules to sub-expressions
    
        if intermediate.len() == 1 {
            // We already tried to apply all rules.
            // We dont need to do it again!
            intermediate
        } else {
            intermediate.into_iter()
                .flat_map(|e| e.simplify_step(context, max_steps - 1))
                .unique()
                .collect()
        }
    }
}

// // if no rule could be applied to the top-level
//     // expression, try to apply rules to the sub-expressions
//     if intermediate.len() == 1 {
//         println!("no rule could be applied (to the top-level expression)!");
//         #[allow(clippy::single_match)]
//         match &intermediate.first().unwrap() {
//             // Expression::Sum(lhs, rhs) => {
//             //     let lhs_simplified = try_simplify(lhs, max_steps - 1)
//             //         .into_iter()
//             //         .map(|e| Expression::Sum(Box::new(e), rhs.clone()))
//             //         .collect::<Vec<_>>();

//             //     let rhs_simplified = try_simplify(rhs, max_steps - 1)
//             //         .into_iter()
//             //         .map(|e| Expression::Sum(lhs.clone(), Box::new(e)))
//             //         .collect::<Vec<_>>();

//             //     intermediate.extend(lhs_simplified);
//             //     intermediate.extend(rhs_simplified);
//             // }
//             Expression::Difference(lhs, rhs) => {
//                 let lhs_simplified = simplify_step(lhs, max_steps - 1)
//                     .into_iter()
//                     .map(|e| Expression::Difference(Box::new(e), rhs.clone()))
//                     .collect::<Vec<_>>();

//                 let rhs_simplified = simplify_step(rhs, max_steps - 1)
//                     .into_iter()
//                     .map(|e| Expression::Difference(lhs.clone(), Box::new(e)))
//                     .collect::<Vec<_>>();

//                 intermediate.extend(lhs_simplified);
//                 intermediate.extend(rhs_simplified);
//             }
//             // Expression::Product(lhs, rhs) => {
//             //     let lhs_simplified = try_simplify(lhs, max_steps - 1)
//             //         .into_iter()
//             //         .map(|e| Expression::Product(Box::new(e), rhs.clone()))
//             //         .collect::<Vec<_>>();

//             //     let rhs_simplified = try_simplify(rhs, max_steps - 1)
//             //         .into_iter()
//             //         .map(|e| Expression::Product(lhs.clone(), Box::new(e)))
//             //         .collect::<Vec<_>>();

//             //     intermediate.extend(lhs_simplified);
//             //     intermediate.extend(rhs_simplified);
//             // }
//             // Expression::Quotient(lhs, rhs) => {
//             //     let lhs_simplified = try_simplify(lhs, max_steps - 1)
//             //         .into_iter()
//             //         .map(|e| Expression::Quotient(Box::new(e), rhs.clone()))
//             //         .collect::<Vec<_>>();

//             //     let rhs_simplified = try_simplify(rhs, max_steps - 1)
//             //         .into_iter()
//             //         .map(|e| Expression::Quotient(lhs.clone(), Box::new(e)))
//             //         .collect::<Vec<_>>();

//             //     intermediate.extend(lhs_simplified);
//             //     intermediate.extend(rhs_simplified);
//             // }
//             _ => {}
//         }
//     }
