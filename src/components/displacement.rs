//! Displacement component
//!
//! This component represents an object

/// Represents a displacement in the game world
#[derive(Debug, Copy, Clone)]
pub struct Displacement {
    /// The x coordinate of displacement
    pub x: f64,
    /// The y coordinate of displacement
    pub y: f64,
}

impl Displacement {
    /// Creates a new displacement component
    pub fn new(x: f64, y: f64) -> Self {
        Displacement {
            x,
            y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_displacement() {
        let d = Displacement::new(0.0, 0.0);
        assert_eq!(d.x, 0.0);
        assert_eq!(d.y, 0.0);
    }

    #[test]
    fn copy_displacement() {
        let mut x = Displacement::new(0.0, 0.0);
        let y = x;
        assert_eq!(x.x, y.x);
        assert_eq!(x.y, y.y);
        x.x = 1.0;
        assert_eq!(x.x, 1.0);
        assert_eq!(y.x, 0.0);
    }

    #[test]
    fn clone_displacement() {
        let mut x = Displacement::new(0.0, 0.0);
        let y = x.clone();
        assert_eq!(x.x, y.x);
        assert_eq!(x.y, y.y);
        x.x = 1.0;
        assert_eq!(x.x, 1.0);
        assert_eq!(y.x, 0.0);
    }
}
