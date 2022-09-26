pub mod math;
pub mod physics;
pub mod renderers;

pub fn hello_source_lib() -> String {
    format!("Hello from source lib!")
}

#[test]
fn hello_source_lib_test() {
    assert_eq!(hello_source_lib(), "Hello from source lib!");
}