use crate::expr::Expression;

pub struct SimplifyContext {
    pub original: Expression,
    pub original_depth: usize,
    pub max_depth_difference_from_original: usize,
    pub generated_expressions: Vec<Expression>
}

impl SimplifyContext {
    pub fn new(original: Expression, original_depth: usize, max_depth_difference_from_original: usize, generated_expressions: Vec<Expression>) -> Self {
        Self { original, original_depth, max_depth_difference_from_original, generated_expressions }
    }
}
