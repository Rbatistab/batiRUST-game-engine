#[cfg(test)]
mod math_test {
    use game_engine_lib as lib;
    use lib::math as math;
    use lib::physics as physics;
    use lib::renderers as renderers;
    
    #[test]
    fn integration_test() {
        let lib_test: String        = lib::hello_source_lib();
        let math_test: String       = math::hello_from_math();
        let physics_test: String    = physics::hello_from_physics();
        let renderers_test: String  = renderers::hello_from_renderers();
        
        let integration_string: String  = lib_test 
                                          + "\n" + &math_test
                                          + "\n" + &physics_test
                                          + "\n" + &renderers_test;
        
        let mut integration_string_test: String = String::new();
        integration_string_test   = integration_string_test 
                                    + "Hello from source lib!"
                                    + "\nHello from math module!"
                                    + "\nHello from physics module!"
                                    + "\nHello from renderers module!";

        assert_eq!(integration_string, integration_string_test);
        
    }
}
