use std::{collections::{HashMap, HashSet}, any::{Any,TypeId}, hash::Hash, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use ecs_derive::Component;

pub trait Component: 'static{
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
        Entity{id: COUNTER.fetch_add(1, Ordering::Relaxed), components: HashMap::new()}
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
    entities: HashMap<usize, Entity>
}

impl World {
    pub fn new() -> World{
        World{entities: HashMap::new()}
    }

    pub fn query(&self, query: Query) -> HashSet<&Entity> {
        let mut result: HashSet<&Entity> = HashSet::new();
        for (id, entity) in &self.entities {
            if query.include.iter().all(|k| entity.components.contains_key(k)) {
                result.insert(entity);
            }
        }
        result
    }

    pub fn new_entity(&mut self) -> usize{
        let entity = Entity::new();
        let id = entity.id;
        self.entities.insert(id, entity);
        id
    }

    pub fn remove_entity(&mut self, entity: &usize) {
        self.entities.remove(entity);
    }

    pub fn get_entity_mut(&mut self, id: &usize) -> Option<&mut Entity>{
        self.entities.get_mut(id)
    }

    pub fn get_entity(&self, id: &usize) -> Option<&Entity> {
        self.entities.get(id)
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

#[cfg(test)]
mod test {
    use super::{World, Component, Query};
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

        let first_entity_id = world.new_entity();
        let first_entity = world.get_entity_mut(&first_entity_id).unwrap();
        first_entity.add_component(StringComponent{name: String::from("First Entity")});
        
        let second_entity_id = world.new_entity();
        let second_entity = world.get_entity_mut(&second_entity_id).unwrap();
        second_entity.add_component(StringComponent{name: String::from("Second Entity")});
        second_entity.add_component(IntComponent{number: 1138});
        
        // When
        let actual = world.query(Query::new().with::<StringComponent>().with::<IntComponent>());
        
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

        let first_entity_id = world.new_entity();
        let first_entity = world.get_entity_mut(&first_entity_id).unwrap();
        first_entity.add_component(StringComponent{name: String::from("First Entity")});
        
        let second_entity_id = world.new_entity();
        let second_entity = world.get_entity_mut(&second_entity_id).unwrap();
        second_entity.add_component(StringComponent{name: String::from("Second Entity")});
        second_entity.add_component(IntComponent{number: 1138});                        
        
        // When
        let actual = world.query(Query::new().with::<StringComponent>());
        
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