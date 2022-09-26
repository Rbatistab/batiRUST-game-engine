pub fn hello_from_renderers() -> String {
    format!("Hello from renderers module!")
}

#[cfg(test)]
mod renderers_test {
    use crate::renderers;

    #[test]
    fn hello_from_renderers_test() {
        assert_eq!(renderers::hello_from_renderers(), "Hello from renderers module!");
    }
}
