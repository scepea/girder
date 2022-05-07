use std::{collections::{HashMap, HashSet}, any::{Any,TypeId}, hash::Hash, sync::atomic::{AtomicUsize, Ordering}};

pub trait Component: 'static{
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Entity {
    id: usize
}

impl Entity {
    fn new() -> Entity {
        static COUNTER:AtomicUsize = AtomicUsize::new(1);
        Entity{id: COUNTER.fetch_add(1, Ordering::Relaxed)}
    }
}

struct World {
    entities: HashSet<Entity>,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Component>>>
}

impl World {
    pub fn new() -> World{
        World{entities: HashSet::new(), components: HashMap::new()}
    }

    pub fn new_entity(&mut self) -> Entity{
        let entity = Entity::new();
        self.entities.insert(entity);
        entity
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        self.entities.remove(&entity);
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let set = self.components.get_mut(&TypeId::of::<T>());
        if set.is_some(){
            set.unwrap().insert(entity, Box::new(component));
        } else {
            let mut map:HashMap<Entity, Box<dyn Component>> = HashMap::new();
            map.insert(entity, Box::new(component));
            self.components.insert(TypeId::of::<T>(), map);
        }
        
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T>{
        match self.components.get(&TypeId::of::<T>()) {
            Some(x) => match x.get(&entity) {
                Some (x) => x.as_any().downcast_ref::<T>(),
                None => None
            },
            None => None
        }
    }

    pub fn query(&self, query: Query) -> HashSet<Entity> {
        let mut result: HashSet<Entity> = HashSet::new();
        
        result.extend(self.entities.iter()); //[TODO] this is slow.

        for x in query.include {
            let components_of_type = self.components.get(&x);
            if components_of_type.is_some() {
                let entities_with_component: HashSet<&Entity> = components_of_type.unwrap().keys().collect();
                result.retain(|x| entities_with_component.contains(x));
            }
        }
        
        result
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
    use ecs_derive::Component;

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

        let first_entity = world.new_entity();
        world.add_component(first_entity, StringComponent{name: String::from("First Entity")});
        
        let second_entity = world.new_entity();
        world.add_component(second_entity, StringComponent{name: String::from("Second Entity")});
        world.add_component(second_entity, IntComponent{number: 1138});
        
        // When
        let actual = world.query(Query::new().with::<StringComponent>().with::<IntComponent>());
        
        // Then
        assert_eq!(1, actual.len());
        for result_entity in actual {
            assert_eq!(String::from("Second Entity"), world.get_component::<StringComponent>(result_entity).expect("").name);
            assert_eq!(1138, world.get_component::<IntComponent>(result_entity).expect("").number);
        }
    }

    #[test]
    fn test_multiple_result(){
        // Given
        let mut world = World::new();

        let first_entity = world.new_entity();
        world.add_component(first_entity, StringComponent{name: String::from("First Entity")});
        
        let second_entity = world.new_entity();
        world.add_component(second_entity, StringComponent{name: String::from("Second Entity")});
        world.add_component(second_entity, IntComponent{number: 1138});            
        
        // When
        let actual = world.query(Query::new().with::<StringComponent>());
        
        // Then
        assert_eq!(2, actual.len());
        for result_entity in actual {
            let int_component = world.get_component::<IntComponent>(result_entity);

            if int_component.is_some() {
                assert_eq!(String::from("Second Entity"), world.get_component::<StringComponent>(result_entity).expect("").name);
                assert_eq!(1138, int_component.expect("").number);
            } else {
                assert_eq!(String::from("First Entity"), world.get_component::<StringComponent>(result_entity).expect("").name);
            }
        }
    }

}