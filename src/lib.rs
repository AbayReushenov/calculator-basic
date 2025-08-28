pub mod ops {
    pub fn add(a: f64, b: f64) -> f64 {
        a + b
    }
    pub fn sub(a: f64, b: f64) -> f64 {
        a - b
    }
    pub fn mul(a: f64, b: f64) -> f64 {
        a * b
    }
    pub fn div(a: f64, b: f64) -> Result<f64, &'static str> {
        if b == 0.0 {
            Err("division by zero")
        } else {
            Ok(a / b)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn add_ok() {
            assert_eq!(add(2.0, 3.0), 5.0);
        }
        #[test]
        fn div_by_zero_err() {
            assert!(div(1.0, 0.0).is_err());
        }
    }
}
