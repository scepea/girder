pub fn service_function(echo: String) -> String{
	service_function_implementation(echo, another_module::hello_dependency, another_module::echo_dependency)
}

fn service_function_implementation(echo: String, first_dependency: fn() -> String, second_dependency: fn(String) -> String) -> String{
	let mut result = String::from("");
    result.push_str(&first_dependency());
    result.push_str(", ");
    result.push_str(&second_dependency(echo));
    result.push_str("!");
    result
}

#[cfg(test)]
#[test]
fn unit_test() {
    let echo = String::from("Test");
    let result: String = service_function_implementation(
        echo,
        || -> String {String::from("Unit")}, 
        |x| -> String {x}
    );
    assert_eq!("Unit, Test!", result);
}

mod another_module {
    use crate::advanced_dependency_injection::service_function;

    #[cfg(test)]
    #[test]
    fn integration_test() {
        use crate::advanced_dependency_injection::service_function_implementation;

        let result: String = service_function(String::from("World"));
        assert_eq!("Hello, World!", result);
    }

    pub fn hello_dependency() -> String {
        String::from("Hello")
    }

    pub fn echo_dependency(echo: String) -> String {
        echo
    }

}