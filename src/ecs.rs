use std::{collections::{HashMap}, any::{TypeId}};

use ecs_derive::Component;

pub trait Component{
    fn id(self: &Self) -> u64;
}

#[derive(Component)]
struct DemoComponent{
    name: String
}

struct Entity {
    components: HashMap<TypeId, Box<dyn Component>>
}

impl Entity {

    fn new() -> Entity {
        Entity{components: HashMap::new()}
    }
    
    fn add_component<T: 'static + Component>(self: &mut Self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

}

pub fn example() {
    let mut e = Entity::new();
    e.add_component(DemoComponent{name: String::from("Mr Demo")});
}