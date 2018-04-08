//! Representation of the world

use super::component::Component;

const ENTITY_COUNT: usize = 512;

/// The world
pub struct World {
    mask: [Component; ENTITY_COUNT],
}

impl Default for World {
    fn default() -> Self {
        World {
            mask: [Component::NONE; ENTITY_COUNT],
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
    }
}
