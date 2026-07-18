
 pub struct Calculator {
    num1: f64,
    num2: f64,
}


impl Calculator {
   pub fn add(&self) -> f64 {
        self.num1 + self.num2
    }

    pub fn sub(&self) -> f64 {
        self.num1 - self.num2
    }
    pub fn mul(&self) -> f64 {
        self.num1 * self.num2
    }
    pub fn div(&self) -> f64 {
        self.num1 / self.num2
    }
    pub fn new(num1: f64, num2: f64) -> Self {
        Self { num1, num2 }
    }
}
