pub fn hello_from_physics() -> String {
    format!("Hello from physics module!")
}

#[cfg(test)]
mod physics_test {
    use crate::physics;

    #[test]
    fn hello_from_physics_test() {
        assert_eq!(physics::hello_from_physics(), "Hello from physics module!");
    }
}
