use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub enum Number {
  Integer(i64),
  Float(f64),
}

impl Number {
  pub fn to_string(&self) -> String {
    match self {
      Number::Integer(a) => a.to_string(),
      Number::Float(a) => a.to_string(),
    }
  }
  pub fn pow(&self, other: &Number) -> Number {
    let one: f64 = self.into();
    let two: f64 = other.into();
    let result = one.powf(two);
    Number::Float(result)
  }
}

// Impl add for &Number
impl Add for &Number {
  type Output = Number;
  fn add(self, other: &Number) -> Number {
    match (self, other) {
      (Number::Integer(a), Number::Integer(b)) => Number::Integer(a + b),
      (Number::Integer(a), Number::Float(b)) => Number::Float(*a as f64 + b),
      (Number::Float(a), Number::Integer(b)) => Number::Float(a + *b as f64),
      (Number::Float(a), Number::Float(b)) => Number::Float(a + b),
    }
  }
}

// Impl sub for &Number
impl std::ops::Sub for &Number {
  type Output = Number;
  fn sub(self, other: &Number) -> Number {
    match (self, other) {
      (Number::Integer(a), Number::Integer(b)) => Number::Integer(a - b),
      (Number::Integer(a), Number::Float(b)) => Number::Float(*a as f64 - b),
      (Number::Float(a), Number::Integer(b)) => Number::Float(a - *b as f64),
      (Number::Float(a), Number::Float(b)) => Number::Float(a - b),
    }
  }
}

// Impl mul for &Number
impl std::ops::Mul for &Number {
  type Output = Number;
  fn mul(self, other: &Number) -> Number {
    match (self, other) {
      (Number::Integer(a), Number::Integer(b)) => Number::Integer(a * b),
      (Number::Integer(a), Number::Float(b)) => Number::Float(*a as f64 * b),
      (Number::Float(a), Number::Integer(b)) => Number::Float(a * *b as f64),
      (Number::Float(a), Number::Float(b)) => Number::Float(a * b),
    }
  }
}

// Impl div for &Number
impl std::ops::Div for &Number {
  type Output = Number;
  fn div(self, other: &Number) -> Number {
    match (self, other) {
      (Number::Integer(a), Number::Integer(b)) => Number::Integer(a / b),
      (Number::Integer(a), Number::Float(b)) => Number::Float(*a as f64 / b),
      (Number::Float(a), Number::Integer(b)) => Number::Float(a / *b as f64),
      (Number::Float(a), Number::Float(b)) => Number::Float(a / b),
    }
  }
}

// Impl rem for &Number
impl std::ops::Rem for &Number {
  type Output = Number;
  fn rem(self, other: &Number) -> Number {
    match (self, other) {
      (Number::Integer(a), Number::Integer(b)) => Number::Integer(a % b),
      (Number::Integer(a), Number::Float(b)) => Number::Float(*a as f64 % b),
      (Number::Float(a), Number::Integer(b)) => Number::Float(a % *b as f64),
      (Number::Float(a), Number::Float(b)) => Number::Float(a % b),
    }
  }
}

// Impl neg for &Number
impl std::ops::Neg for &Number {
  type Output = Number;
  fn neg(self) -> Number {
    match self {
      Number::Integer(a) => Number::Integer(-a),
      Number::Float(a) => Number::Float(-a),
    }
  }
}

// Impl PartialEq for &Number
impl PartialEq for Number {
  fn eq(&self, other: &Number) -> bool {
    match (self, other) {
      (Number::Integer(a), Number::Integer(b)) => a == b,
      (Number::Integer(a), Number::Float(b)) => *a as f64 == *b,
      (Number::Float(a), Number::Integer(b)) => *a == *b as f64,
      (Number::Float(a), Number::Float(b)) => a == b,
    }
  }
}

// Impl PartialOrd for &Number
impl PartialOrd for Number {
  fn partial_cmp(&self, other: &Number) -> Option<std::cmp::Ordering> {
    match (self, other) {
      (Number::Integer(a), Number::Integer(b)) => a.partial_cmp(b),
      (Number::Integer(a), Number::Float(b)) => (*a as f64).partial_cmp(b),
      (Number::Float(a), Number::Integer(b)) => a.partial_cmp(&(*b as f64)),
      (Number::Float(a), Number::Float(b)) => a.partial_cmp(b),
    }
  }
}

// Impl From<i64> for &Number
impl From<i64> for Number {
  fn from(n: i64) -> Number {
    Number::Integer(n)
  }
}

// Impl From<f64> for &Number
impl From<f64> for Number {
  fn from(n: f64) -> Number {
    Number::Float(n)
  }
}

// Impl From<Number> for i64
impl From<&Number> for i64 {
  fn from(n: &Number) -> i64 {
    match n {
      Number::Integer(n) => *n,
      Number::Float(n) => *n as i64,
    }
  }
}

// Impl From<Number> for f64
impl From<&Number> for f64 {
  fn from(n: &Number) -> f64 {
    match n {
      Number::Integer(n) => *n as f64,
      Number::Float(n) => *n,
    }
  }
}

// Impl From<Number> for String
impl From<Number> for String {
  fn from(n: Number) -> String {
    match n {
      Number::Integer(n) => n.to_string(),
      Number::Float(n) => n.to_string(),
    }
  }
}
