pub fn hello_from_math() -> String {
    format!("Hello from math module!")
}

#[cfg(test)]
mod math_test {
    use crate::math;

    #[test]
    fn hello_from_math_test() {
        assert_eq!(math::hello_from_math(), "Hello from math module!");
    }
}
