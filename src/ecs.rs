use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::atomic::{AtomicUsize, Ordering},
};

pub trait Component: 'static {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Entity {
    id: usize,
}

impl Entity {
    fn new() -> Entity {
        static COUNTER: AtomicUsize = AtomicUsize::new(1);
        Entity {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

struct World {
    entities: HashSet<Entity>,
    components: HashMap<TypeId, HashMap<Entity, Box<dyn Component>>>,
}

impl World {
    pub fn new() -> World {
        World {
            entities: HashSet::new(),
            components: HashMap::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = Entity::new();
        self.entities.insert(entity);
        entity
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        for (_, x) in &mut self.components {
            x.remove(&entity);
        }
        self.entities.remove(&entity);
    }

    pub fn add_component<T: Component>(&mut self, entity: Entity, component: T) {
        let set = self.components.get_mut(&TypeId::of::<T>());
        if set.is_some() {
            set.unwrap().insert(entity, Box::new(component));
        } else {
            let mut map: HashMap<Entity, Box<dyn Component>> = HashMap::new();
            map.insert(entity, Box::new(component));
            self.components.insert(TypeId::of::<T>(), map);
        }
    }

    pub fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        match self.components.get(&TypeId::of::<T>()) {
            Some(x) => match x.get(&entity) {
                Some(x) => x.as_any().downcast_ref::<T>(),
                None => None,
            },
            None => None,
        }
    }

    pub fn get_component_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T> {
        match self.components.get_mut(&TypeId::of::<T>()) {
            Some(x) => match x.get_mut(&entity) {
                Some(x) => x.as_any_mut().downcast_mut::<T>(),
                None => None,
            },
            None => None,
        }
    }

    pub fn remove_component<T: Component>(&mut self, entity: Entity) {
        let x = self.components.get_mut(&TypeId::of::<T>());
        if x.is_some() {
            x.unwrap().remove(&entity);
        }
    }

    pub fn query(&self, query: Query) -> HashSet<Entity> {
        let mut result: HashSet<Entity> = HashSet::new();

        result.extend(self.entities.iter()); //[TODO] this is slow.

        for x in query.include {
            let components_of_type = self.components.get(&x);
            if components_of_type.is_some() {
                let entities_with_component: HashSet<&Entity> =
                    components_of_type.unwrap().keys().collect();
                result.retain(|x| entities_with_component.contains(x));
            } else {
                result.clear();
            }
        }

        result
    }
}

struct Query {
    include: HashSet<TypeId>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            include: HashSet::new(),
        }
    }

    pub fn with<T: Component>(mut self) -> Self {
        self.include.insert(TypeId::of::<T>());
        self
    }
}

#[cfg(test)]
mod test {
    use ecs_derive::Component;

    use super::{Component, Query, World};
    use std::{any::Any};

    #[derive(Component)]
    struct ClassComponent {
        name: String,
    }

    #[derive(Component, Debug, PartialEq, Eq)]
    struct IdComponent {
        number: u32,
    }

    #[derive(Component)]
    struct MarkerComponent {}

    #[test]
    fn test_query_unused_component() {
        // Given
        let mut world = World::new();

        let negative_entity = world.new_entity();

        // When
        let actual = world.query(Query::new().with::<MarkerComponent>());

        // Then
        assert_eq!(None, actual.get(&negative_entity));
    }

    #[test]
    fn test_result_singleton() {
        // Given
        let mut world = World::new();

        let negative_entity = world.new_entity();

        let positive_entity = world.new_entity();
        world.add_component(positive_entity, MarkerComponent {});

        // When
        let actual = world.query(Query::new().with::<MarkerComponent>());

        // Then
        assert_eq!(&positive_entity, actual.get(&positive_entity).unwrap());
        assert_eq!(None, actual.get(&negative_entity));
    }

    #[test]
    fn test_result_set() {
        // Given
        let mut world = World::new();

        let negative_entity = world.new_entity();

        let positive_entity_1 = world.new_entity();
        world.add_component(positive_entity_1, MarkerComponent {});

        let positive_entity_2 = world.new_entity();
        world.add_component(positive_entity_2, MarkerComponent {});

        // When
        let actual = world.query(Query::new().with::<MarkerComponent>());

        // Then
        assert_eq!(&positive_entity_1, actual.get(&positive_entity_1).unwrap());
        assert_eq!(&positive_entity_2, actual.get(&positive_entity_2).unwrap());
        assert_eq!(None, actual.get(&negative_entity));
    }

    #[test]
    fn test_query_no_results() {
        // Given
        let mut world = World::new();

        let negative_entity_1 = world.new_entity();
        world.add_component(negative_entity_1, MarkerComponent {});
        world.add_component(negative_entity_1, IdComponent { number: 1 });

        let negative_entity_2 = world.new_entity();
        world.add_component(negative_entity_2, MarkerComponent {});
        world.add_component(
            negative_entity_2,
            ClassComponent {
                name: String::from(""),
            },
        );

        // When
        let actual = world.query(Query::new().with::<IdComponent>().with::<ClassComponent>());

        // Then
        assert_eq!(None, actual.get(&negative_entity_1));
        assert_eq!(None, actual.get(&negative_entity_2));
    }

    #[test]
    fn test_read_component() {
        // Given
        let mut world = World::new();

        let positive_entity = world.new_entity();
        let id = 1138;
        world.add_component(positive_entity, IdComponent{number: id});


        // When
        let actual = world.get_component::<IdComponent>(positive_entity);

        // Then
        assert_eq!(id, actual.unwrap().number);
    }

    #[test]
    fn test_update_component() {
        // Given
        let mut world = World::new();

        let positive_entity = world.new_entity();
        let old_id = 1138;
        world.add_component(positive_entity, IdComponent{number: old_id});

        let new_id = 8311;
        
        // When
        world.get_component_mut::<IdComponent>(positive_entity).unwrap().number = new_id;
        let actual = world.get_component::<IdComponent>(positive_entity);

        // Then
        assert_eq!(new_id, actual.unwrap().number);
    }

    #[test]
    fn test_delete_component() {
            // Given
            let mut world = World::new();

            let positive_entity = world.new_entity();
            let id = 1;
            world.add_component(positive_entity, IdComponent{number: id});
    
            // When
            world.remove_component::<IdComponent>(positive_entity);
            let actual = world.get_component::<IdComponent>(positive_entity);
    
            // Then
            assert_eq!(None, actual);
    }

    #[test]
    fn test_delete_entity() {
            // Given
            let mut world = World::new();

            let positive_entity = world.new_entity();
            world.add_component(positive_entity, IdComponent{number: 1});

            let negative_entity = world.new_entity();
            let expected_id = 2;
            world.add_component(negative_entity, IdComponent{number: expected_id});
    
            // When
            world.remove_entity(positive_entity);
            let positive_actual = world.get_component::<IdComponent>(positive_entity);
            let negative_actual = world.get_component::<IdComponent>(negative_entity);

            // Then
            assert_eq!(world.entities.len(), 1);
            assert_eq!(None, positive_actual);
            assert_eq!(expected_id, negative_actual.unwrap().number);
    }
}
