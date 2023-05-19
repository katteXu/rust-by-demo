mod checked {

    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeLogarithm,
        NegativeSquareRoot,
    }

    pub type MathResult = Result<f64, MathError>;

    pub fn div(x: f64, y: f64) -> MathResult {
        if y == 0.0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(x / y)
        }
    }

    pub fn sqrt(x: f64) -> MathResult {
        println!("x === >{}", x);
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    pub fn ln(x: f64) -> MathResult {
        println!("x === > {}", x);
        if x < 0.0 {
            Err(MathError::NegativeLogarithm)
        } else {
            Ok(x.ln())
        }
    }

    fn op_(x: f64, y: f64) -> MathResult {
        let ratio = div(x, y)?;
        let ln = ln(ratio)?;

        sqrt(ln)
    }

    pub fn op(x: f64, y: f64) {
        match op_(x, y) {
            Ok(value) => println!("{}", value),
            Err(why) => match why {
                MathError::DivisionByZero => println!("division by zero"),
                MathError::NegativeLogarithm => println!("logarithm of negative number"),
                MathError::NegativeSquareRoot => println!("square root of negative number"),
            },
        }
    }
}

fn main() {
    checked::op(100.0, 1001.0);
}
