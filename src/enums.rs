pub enum Component {
    Name(String),
    Physics{mass: i32, velocity: i32},
    Job{execute: fn() -> i32},
}

pub fn string_it(component: Component) -> String {
    match component {
        Component::Name(x) => x, // Echo the name
        Component::Physics { mass, velocity } => (mass*velocity).to_string(), // Calculate momentum
        Component::Job { execute } => execute().to_string(), // Execute job
    }
}

pub fn example() {
    println!("{}", string_it(Component::Name(String::from("Foobar"))));
    println!("{}", string_it(Component::Physics { mass: 3, velocity: 4 }));
    println!("{}", string_it(Component::Job { execute: || -> i32 {5} }));
}