pub fn injector(echo: String) -> String{
	implementation(echo, another_module::hello_dependency, another_module::echo_dependency)
}

fn implementation(echo: String, first_dependency: fn() -> String, second_dependency: fn(String) -> String) -> String{
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
    let result: String = implementation(
        echo,
        || -> String {String::from("Unit")}, 
        |x| -> String {x}
    );
    assert_eq!("Unit, Test!", result);
}

mod another_module {
    use crate::advanced_dependency_injection::injector;

    #[cfg(test)]
    #[test]
    fn integration_test() {
        let result: String = injector(String::from("World"));
        assert_eq!("Hello, World!", result);
    }

    pub fn hello_dependency() -> String {
        String::from("Hello")
    }

    pub fn echo_dependency(echo: String) -> String {
        echo
    }

}