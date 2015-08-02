//! Square root functionality for integral types
//! 
//! These all return f64 for extra precision. Feel free to down-cast if needed

///A trait that allows calculation of square root into a Float(64)
pub trait SquareRoot64 {
    ///Calculates the square root of a number
    fn sqrt(self) -> f64;
}

impl SquareRoot64 for i8 {

    ///Calculates the square root of an i8, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for u8 {

    ///Calculates the square root of a u8, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for i16 {

    ///Calculates the square root of an i16, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for u16 {

    ///Calculates the square root of a u16, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for i32 {

    ///Calculates the square root of an i32, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for u32 {

    ///Calculates the square root of a u32, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for i64 {

    ///Calculates the square root of an i64, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for u64 {

    ///Calculates the square root of a u64, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for isize {

    ///Calculates the square root of an isize, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

impl SquareRoot64 for usize {

    ///Calculates the square root of a usize, returning a f64
    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}

#[cfg(test)]
mod test {

    use super::SquareRoot64;

    ///Test square root functions on i8s
    #[test]
    fn test_i8_sqrt() {
        let value: i8 = 55;
        assert!(value.sqrt().abs_sub(7.4161984871f64) <= 0.001);
    }

    ///Test square root functions on u8s
    #[test]
    fn test_u8_sqrt() {
        let value: u8 = 214;
        assert!(value.sqrt().abs_sub(14.6287388383f64) <= 0.001);
    }


    ///Test square root functions on i16s
    #[test]
    fn test_i16_sqrt() {
        let value: i16 = 25000;
        assert!(value.sqrt().abs_sub(158.113883008f64) <= 0.001);
    }

    ///Test square root functions on u16s
    #[test]
    fn test_u16_sqrt() {
        let value: u16 = 50000;
        assert!(value.sqrt().abs_sub(223.60679775f64) <= 0.001);
    }

    ///Test square root functions on i32s
    #[test]
    fn test_i32_sqrt() {
        let value: i32 = 100;
        assert!(value.sqrt().abs_sub(10f64) <= 0.001);
    }

    ///Test square root functions on u32s
    #[test]
    fn test_u32_sqrt() {
        let value: u32 = 4000000000;
        assert!(value.sqrt().abs_sub(63245.5532034f64) <= 0.001);
    }

    ///Test square root functions on i64s
    #[test]
    fn test_i64_sqrt() {
        let value: i64 = 7777777777777777;
        assert!(value.sqrt().abs_sub(88191710.3688f64) <= 0.001); //Oh fuck this
    }

    ///Test square root functions on u64s
    #[test]
    fn test_u64_sqrt() {
        let value: u64 = 777777777777777777;
        assert!(value.sqrt().abs_sub(881917103.688f64) <= 0.001); //Oh fuck this
    }

    ///Test square root functions on isizes
    #[test]
    fn test_isize_sqrt() {
        let value: isize = 7777777777777777;
        assert!(value.sqrt().abs_sub(88191710.3688f64) <= 0.001); //Oh fuck this
    }

    ///Test square root functions on usizes
    #[test]
    fn test_usize_sqrt() {
        let value: usize = 777777777777777777;
        assert!(value.sqrt().abs_sub(881917103.688f64) <= 0.001); //Oh fuck this
    }

}