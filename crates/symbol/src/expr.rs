pub type Integer = num::bigint::BigInt;
pub type Rational = num::rational::Ratio<Integer>;
pub type Complex = num::complex::Complex<Integer>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expression {
    Variable(String),
    // Function(String, Vec<Self>, Box<Self>),
    Integer(Integer),
    RationalNumber(Rational),
    ComplexNumber(Complex),

    Sum(Box<Self>, Box<Self>),
    Difference(Box<Self>, Box<Self>),
    Product(Box<Self>, Box<Self>),
    Quotient(Box<Self>, Box<Self>),
    Power(Box<Self>, Box<Self>),

    // Boolean(bool),
    Negation(Box<Self>),
    Equality(Box<Self>, Box<Self>),
    // NotEqual(Box<Self>, Box<Self>),
    LessThan(Box<Self>, Box<Self>),
    LessThanOrEqual(Box<Self>, Box<Self>),
    GreatherThan(Box<Self>, Box<Self>),
    GreatherThanOrEqual(Box<Self>, Box<Self>),
    // Not(Box<Self>),
    // And(Box<Self>, Box<Self>),
    // Or(Box<Self>, Box<Self>),

    // Vector,
    // Matrix,
}

impl std::ops::Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Expression::Negation(Box::new(self))
    }
}

impl std::ops::Not for Expression {
    type Output = Self;

    fn not(self) -> Self::Output {
        Expression::Negation(Box::new(self))
    }
}

impl std::ops::Add for Expression {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::Sum(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::Sub for Expression {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Expression::Difference(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::Mul for Expression {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Expression::Product(Box::new(self), Box::new(rhs))
    }
}

impl std::ops::Div for Expression {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Expression::Quotient(Box::new(self), Box::new(rhs))
    }
}

impl Expression {
    pub fn typ(&self) -> ExpressionType {
        todo!()
    }

    pub fn precedence(&self) -> usize {
        todo!()
    }

    pub fn associativity(&self) -> Associativity {
        todo!()
    }
}

pub enum ExpressionType {
    Number,
    Logical,
    Function,
    Other,
}

pub enum Associativity {
    Left,
    Right,
    Both,
}
