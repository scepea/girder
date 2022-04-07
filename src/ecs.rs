use std::{collections::{HashMap, HashSet}, any::{Any,TypeId}, hash::Hash};

use ecs_derive::Component;

pub trait Component {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Component)]
struct NameComponent{
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

    fn get_component<T: 'static + Component>(self: &Self) -> Option<&T>{
        self.components.get(&TypeId::of::<T>()).expect("msg").as_any().downcast_ref::<T>()
    }

}

pub fn example() {
    let mut entity = Entity::new();
    entity.add_component(NameComponent{name: String::from("Primus")});
    println!("{}", entity.get_component::<NameComponent>().expect("").name);
}