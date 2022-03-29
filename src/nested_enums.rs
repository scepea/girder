use std::collections::HashSet;
use std::fmt::Debug;
use Components::PhysicsComponent;
use PhysicsComponents::RigidBody;
use PhysicsComponents::MeshCollider;

#[derive(PartialEq, Eq, Hash, Debug)]
enum SpriteComponents {
    Sprite,
    Atlas
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum PhysicsComponents {
    RigidBody,
    MeshCollider,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Components {
    SpriteComponent(SpriteComponents),
    PhysicsComponent(PhysicsComponents),
}

pub fn example() {
    let mut components:HashSet<Components> = HashSet::new();
    components.insert(Components::SpriteComponent(SpriteComponents::Sprite));
    components.insert(Components::SpriteComponent(SpriteComponents::Atlas));
    components.insert(PhysicsComponent(RigidBody));
    components.insert(PhysicsComponent(MeshCollider));
    for x in components {
        println!("{:?}", x);
    }
}