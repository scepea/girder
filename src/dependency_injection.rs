// Binding
pub fn service() -> impl Service{
    FooService {}
}

// Abstraction
pub trait Service {
    fn foobar(&self) -> &str;
}

// Implementations
struct FooService {}

impl Service for FooService {
    fn foobar(&self) -> &str {
        "foo"
    }
}
struct BarService {}

impl Service for BarService {
    fn foobar(&self) -> &str {
        "bar"
    }
}