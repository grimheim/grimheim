//! Global component controls
//!
//! This module contains masks for components

bitflags! {
    /// A mask of all the available components
    pub struct Component: u64 {
        /// No components
        const NONE = 0b0000_0000;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(Component::NONE.bits, 0b00000000);
    }
}
