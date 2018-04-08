//! Representation of the world

use super::ecs::component::Component;
use super::components::displacement::Displacement;

const ENTITY_COUNT: usize = 512;

/// The world
pub struct World {
    mask: [Component; ENTITY_COUNT],

    displacement: [Displacement; ENTITY_COUNT],
}

impl Default for World {
    fn default() -> Self {
        World {
            mask: [Component::NONE; ENTITY_COUNT],
            displacement: [Displacement::new(0.0, 0.0); ENTITY_COUNT],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn default_world() {
        let w = World::default();
        for e in w.mask.iter() {
            assert_eq!(*e, Component::NONE);
        }
        for d in w.displacement.iter() {
            assert_eq!(d.x, 0.0);
            assert_eq!(d.y, 0.0);
        }
    }
}
