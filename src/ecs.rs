use std::{collections::{HashMap, HashSet}, any::{Any,TypeId}, hash::Hash};

use ecs_derive::Component;

pub trait Component: 'static{
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
    
    fn add_component<T: Component>(&mut self, component: T) {
        self.components.insert(TypeId::of::<T>(), Box::new(component));
    }

    fn remove_component<T: Component>(&mut self) {
        self.components.remove(&TypeId::of::<T>());
    }

    fn get_component<T: Component>(&self) -> Option<&T>{
        match self.components.get(&TypeId::of::<T>()) {
            Some(x) => x.as_any().downcast_ref::<T>(),
            None => None
        }
    }

    pub fn has_component<T: Component>(&self) -> bool{
        match self.components.get(&TypeId::of::<T>()) {
            Some(x) => true,
            None => false
        }
    }

}

struct World {
    entities: HashSet<Entity>
}

impl World {
    pub fn new() -> World{
        World{entities: HashSet::new()}
    }

    pub fn query(&self, query: Query) -> HashSet<&Entity> {
        let mut result: HashSet<&Entity> = HashSet::new();
        for entity in &self.entities {
            if query.include.iter().all(|k| entity.components.contains_key(k)) {
                result.insert(entity);
            }
        }
        result
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.insert(entity);
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.remove(entity);
    }
}

struct Query {
    include: HashSet<TypeId>
}

impl Query {
    pub fn new() -> Query {
        Query { include: HashSet::new() }
    }

    pub fn with<T: Component>(mut self) -> Self{
        self.include.insert(TypeId::of::<T>());
        self
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

    let mut entity = Entity::new();
    entity.add_component(NameComponent{name: String::from("Chaff")});
    println!("We made an entity with the name: {}", entity.get_component::<NameComponent>().expect("").name);
    entity.remove_component::<NameComponent>();
    match entity.get_component::<NameComponent>() {
        Some(_) => panic!("The entity shouldn't have a name component anymore!"),
        None => println!("Now the entity is now nameless, as expected.")
    }
}

#[cfg(test)]
mod test {
    use super::{World, Component, Entity, Query};
    use std::any::Any;

    #[derive(Component)]
    struct StringComponent{
        name: String
    }

    #[derive(Component)]
    struct IntComponent{
        number: u32
    }
    
    #[test]
    fn test_single_result(){
        // Given
        let mut world = World::new();
        let mut e1 = Entity::new();
        let mut e2 = Entity::new();

        let sc1 = StringComponent{name: String::from("First Entity")};
        let sc2 = StringComponent{name: String::from("Second Entity")};

        let ic = IntComponent{number: 1138};

        let query = Query::new().with::<StringComponent>().with::<IntComponent>();
        // When
        e1.add_component(sc1);
        e2.add_component(sc2);
        e2.add_component(ic);

        world.add_entity(e1);
        world.add_entity(e2);

        let actual = world.query(query);
        
        // Then
        assert_eq!(1, actual.len());
        for result in actual {
            assert_eq!(String::from("Second Entity"), result.get_component::<StringComponent>().expect("").name);
            assert_eq!(1138, result.get_component::<IntComponent>().expect("").number);
        }
    }

    #[test]
    fn test_multiple_result(){
        // Given
        let mut world = World::new();
        let mut e1 = Entity::new();
        let mut e2 = Entity::new();

        let sc1 = StringComponent{name: String::from("First Entity")};
        let sc2 = StringComponent{name: String::from("Second Entity")};

        let ic = IntComponent{number: 1138};

        let query = Query::new().with::<StringComponent>();
        // When
        e1.add_component(sc1);
        e2.add_component(sc2);
        e2.add_component(ic);

        world.add_entity(e1);
        world.add_entity(e2);

        let actual = world.query(query);
        
        // Then
        assert_eq!(2, actual.len());
        for result in actual {
            if result.has_component::<IntComponent>() {
                assert_eq!(String::from("Second Entity"), result.get_component::<StringComponent>().expect("").name);
                assert_eq!(1138, result.get_component::<IntComponent>().expect("").number);
            } else {
                assert_eq!(String::from("First Entity"), result.get_component::<StringComponent>().expect("").name);
            }
        }
    }

}