use std::fmt;

use crate::Rectangle;

impl Rectangle {
    pub fn x(&self) -> i32 {
        self.inner.x
    }

    pub fn y(&self) -> i32 {
        self.inner.y
    }

    pub fn width(&self) -> i32 {
        self.inner.width
    }

    pub fn height(&self) -> i32 {
        self.inner.height
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}
