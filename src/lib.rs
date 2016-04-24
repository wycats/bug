pub trait Function {
    fn arity(self) -> u8;
}

impl Function for fn() -> u8 {
    fn arity(self) -> u8 {
        0
    }
}

impl Function for fn(u8) -> u8 {
    fn arity(self) -> u8 {
        1
    }
}

impl Function for fn(u8, u8) -> u8 {
    fn arity(self) -> u8 {
        2
    }
}


pub fn arity<F: Function>(f: F) -> u8 {
    return f.arity();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arity() {
        assert_eq!(arity(zero), 0);
        assert_eq!(arity(one), 1);
        assert_eq!(arity(two), 2);
    }

    fn zero() -> u8 { 42 }
    fn one(_: u8) -> u8 { 42 }
    fn two(_: u8, _: u8) -> u8 { 42 }
}
