use crate::VecN;

/// Splitable multidimensional bounding box
///
/// https://en.wikipedia.org/wiki/Minimum_bounding_box
///
/// No assertions are done in struct methods; it is up to the caller to make
/// sure the indexes don't ever overflow the box.
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox<const DIMENSIONS: usize> {
    pub min: VecN<DIMENSIONS>,
    pub max: VecN<DIMENSIONS>,
}

impl<const DIMENSIONS: usize> BoundingBox<DIMENSIONS> {
    /// Creates a new `BoundingBox`, setting all dimensions to `min` and `max`
    pub fn from(min: f64, max: f64) -> Self {
        Self {
            min: VecN::from(min),
            max: VecN::from(max),
        }
    }

    /// Returns the center `n` coordinate (average of min and max `n`).
    pub fn center(&self, n: usize) -> f64 {
        (self.min[n] + self.max[n]) / 2.
    }

    /// Returns the difference between maximum and minimum `n` coordinates
    pub fn diff(&self, n: usize) -> f64 {
        self.max[n] - self.min[n]
    }

    /// Returns the index quadrant of a point at position `pos`
    pub fn quadrant(&self, pos: VecN<DIMENSIONS>) -> usize {
        let mut idx = 0;
        for i in 0..DIMENSIONS {
            let flip = (pos[i] >= self.center(i)) as usize;
            idx |= (1 * flip) << i;
        }
        idx
    }

    /// Inverse of the `quadrant()` function: returns the subquadrant of this
    /// bounding box.
    /// `quadrant` must have been returned by the `quadrant()` function such
    /// that its value is always between `0` and `2^DIMENSIONS - 1`.
    pub fn child(&self, quadrant: usize) -> Self {
        let mut min = self.min;
        let mut max = self.max;

        for i in 0..DIMENSIONS {
            if (quadrant & (1 << i)) != 0 {
                min[i] = self.center(i);
            } else {
                max[i] = self.center(i);
            }
        }

        Self { min, max }
    }
}
