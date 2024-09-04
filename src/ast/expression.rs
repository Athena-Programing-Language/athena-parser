
pub trait Express {
    fn eval(&self) -> f64;
}


struct Variables {
    value: f64,
}

impl Express for Variables {
    fn eval(&self) -> f64 {
        return self.value
    }
}


struct Suming {
    a: Box<dyn Express>,
    b: Box<dyn Express>,
    operation: char,
}

impl Express for Suming {
    fn new( operation: char, a: Box<dyn Express>, b: Box<dyn Express>, ) -> Self {
        Suming { operation, a, b,  }
    }
    fn eval(&self) -> f64 {
        match self.operation {
            '+' => return self.a + self.b,
            '-' => return self.a - self.b,
            '*' => return self.a * self.b,
            '/' => return self.a / self.b,
            _ =>  return eprintln!("non-existent operation"),
        }
    }
}
