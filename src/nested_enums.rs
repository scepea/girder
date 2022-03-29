use std::collections::HashSet;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash, Debug)]
enum PhysicsComponents {
    RigidBody,
    MeshCollider,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum SpriteComponents {
    Sprite,
    Atlas
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Components {
    PhysicsComponent(PhysicsComponents),
    SpriteComponent(SpriteComponents),
}

pub fn example() {
    let mut components:HashSet<Components> = HashSet::new();
    components.insert(Components::PhysicsComponent(PhysicsComponents::RigidBody));
    components.insert(Components::PhysicsComponent(PhysicsComponents::MeshCollider));
    components.insert(Components::SpriteComponent(SpriteComponents::Sprite));
    components.insert(Components::SpriteComponent(SpriteComponents::Atlas));
    for x in components {
        println!("{:?}", x);
    }
}