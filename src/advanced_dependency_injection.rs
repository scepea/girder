pub fn injector() -> String{
	implementation(another_module::hello_dependency, another_module::world_dependency)
}

fn implementation(first_dependency: fn() -> String, second_dependency: fn() -> String) -> String{
	let mut result = String::from("");
    result.push_str(&first_dependency());
    result.push_str(", ");
    result.push_str(&second_dependency());
    result.push_str("!");
    result
}

#[test]
fn unit_test() {
    let result: String = implementation(
        || -> String {String::from("Unit")}, 
        || -> String {String::from("Test")}
    );
    assert_eq!("Unit, Test!", result);
}

mod another_module {
    use crate::advanced_dependency_injection::injector;

    #[test]
    fn integration_test() {
        let result: String = injector();
        assert_eq!("Hello, World!", result);
    }

    pub fn hello_dependency() -> String {
        String::from("Hello")
    }

    pub fn world_dependency() -> String {
        String::from("World")
    }

}