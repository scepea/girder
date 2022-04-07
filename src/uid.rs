use std::{collections::HashSet,sync::atomic::{AtomicUsize, Ordering},hash::Hash};

struct Entity {
    id: usize
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
        Entity{id: COUNTER.fetch_add(1, Ordering::Relaxed)}
    }
}

pub fn example() {
    let mut entities: HashSet<Entity> = HashSet::new();

    let entity1 = Entity::new();
    entities.insert(entity1);

    let entity2 = Entity::new();
    entities.insert(entity2);

    let entity3 = Entity::new();
    entities.insert(entity3);

    for entity in entities {
        println!("{}", entity.id);
    }
}