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

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        (std::ptr::addr_of!(*self) as usize) == (std::ptr::addr_of!(*other) as usize)
    }
}

impl Eq for Entity {}

impl Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (std::ptr::addr_of!(self) as usize).hash(state)
    }
}

impl Entity {

    fn new() -> Entity {
        Entity{components: HashMap::new()}
    }
    
    fn add_component<T: 'static + Component>(self: &mut Self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    fn get_component<T: 'static + Component>(self: &Self) -> Option<&T>{
        match self.components.get(&TypeId::of::<T>()) {
            Some(x) => x.as_any().downcast_ref::<T>(),
            None => None
        }
    }

}

pub fn example() {
    let mut entities: HashSet<Entity> = HashSet::new();

    let mut entity1 = Entity::new();
    entity1.add_component(NameComponent{name: String::from("Primus")});
    entities.insert(entity1);

    let mut entity2 = Entity::new();
    entity2.add_component(NameComponent{name: String::from("Secundus")});
    entities.insert(entity2);

    let mut entity3 = Entity::new();
    entity3.add_component(NameComponent{name: String::from("Tertius ")});
    entities.insert(entity3);

    for entity in entities {
        println!("{}", entity.get_component::<NameComponent>().expect("").name);
    }
}