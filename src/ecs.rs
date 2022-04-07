use std::{collections::{HashMap, HashSet}, any::{Any,TypeId}, hash::Hash, sync::atomic::{AtomicUsize, Ordering}};

use ecs_derive::Component;

pub trait Component {
    fn as_any(&self) -> &dyn Any;
}

#[derive(Component)]
struct NameComponent{
    name: String
}

struct Entity {
    id: usize,
    components: HashMap<TypeId, Box<dyn Component>>
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entity {}

impl Hash for Entity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Entity {

    fn new() -> Entity {
        static COUNTER:AtomicUsize = AtomicUsize::new(1);
        COUNTER.fetch_add(1, Ordering::Relaxed);

        Entity{id: COUNTER.fetch_add(1, Ordering::Relaxed), components: HashMap::new()}
    }
    
    fn add_component<T: 'static + Component>(self: &mut Self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    fn get_component<T: 'static + Component>(self: &Self) -> Option<&T>{
        self.components.get(&TypeId::of::<T>()).expect("msg").as_any().downcast_ref::<T>()
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