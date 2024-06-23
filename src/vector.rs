use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

#[derive(Copy, Clone, Debug)]
/// A generic vector
pub struct VecN<const DIMENSIONS: usize> {
    coords: [f64; DIMENSIONS],
}

impl<const DIMENSIONS: usize> core::default::Default for VecN<DIMENSIONS> {
    fn default() -> Self {
        Self { coords: [0.0; DIMENSIONS] }
    }
}

impl<const DIMENSIONS: usize> core::fmt::Display for VecN<DIMENSIONS> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.coords)
    }
}


impl<const DIMENSIONS: usize> VecN<DIMENSIONS> {
    /// Creates a new vector with given coordinates
    pub const fn new(coords: [f64; DIMENSIONS]) -> Self {
        Self { coords }
    }

    /// Creates a new vector with all coordinates set to the same value
    pub fn from(value: f64) -> Self {
        Self { coords: [value; DIMENSIONS] }
    }

    /// Clamps each coordinate within -`limit` to `limit`
    pub fn limit(&mut self, limit: f64) {
        for coord in self.coords.iter_mut() {
            *coord = coord.clamp(-limit, limit);
        }
    }

    /// Returns the magnitude (length) of the vector
    pub fn magnitude(&self) -> f64 {
        self.coords.iter().map(|&x| x * x).sum::<f64>().sqrt()
    }

    /// Normalizes the vector, making its magnitude 1
    pub fn normalize(&mut self) {
        let mag = self.magnitude();
        if mag != 0.0 {
            for coord in self.coords.iter_mut() {
                *coord /= mag;
            }
        }
    }

    /// Returns the distance between two vectors
    pub fn distance(&self, other: &VecN<DIMENSIONS>) -> f64 {
        self.coords.iter()
            .zip(other.coords.iter())
            .map(|(&a, &b)| (a - b) * (a - b))
            .sum::<f64>()
            .sqrt()
    }

    /// Sets the vector to all zeros
    pub fn clear(&mut self) {
        self.coords = [0.0; DIMENSIONS];
    }

    /// Checks whether any of the values within the vector are NaN
    pub fn is_nan(&self) -> bool {
        self.coords.iter().find(|x| x.is_nan()).is_some()
    }
}

impl<const DIMENSIONS: usize> Add for VecN<DIMENSIONS> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self;
        result += other;
        result
    }
}

impl<const DIMENSIONS: usize> AddAssign for VecN<DIMENSIONS> {
    fn add_assign(&mut self, other: Self) {
        for i in 0..DIMENSIONS {
            self.coords[i] += other.coords[i];
        }
    }
}

impl<const DIMENSIONS: usize> Sub for VecN<DIMENSIONS> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self;
        result -= other;
        result
    }
}

impl<const DIMENSIONS: usize> SubAssign for VecN<DIMENSIONS> {
    fn sub_assign(&mut self, other: Self) {
        for i in 0..DIMENSIONS {
            self.coords[i] -= other.coords[i];
        }
    }
}

impl<const DIMENSIONS: usize> Mul<f64> for VecN<DIMENSIONS> {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self::Output {
        let mut result = self;
        result *= scalar;
        result
    }
}

impl<const DIMENSIONS: usize> MulAssign<f64> for VecN<DIMENSIONS> {
    fn mul_assign(&mut self, scalar: f64) {
        for coord in self.coords.iter_mut() {
            *coord *= scalar;
        }
    }
}

impl<const DIMENSIONS: usize> Div<f64> for VecN<DIMENSIONS> {
    type Output = Self;

    fn div(self, scalar: f64) -> Self::Output {
        let mut result = self;
        result /= scalar;
        result
    }
}

impl<const DIMENSIONS: usize> DivAssign<f64> for VecN<DIMENSIONS> {
    fn div_assign(&mut self, scalar: f64) {
        for coord in self.coords.iter_mut() {
            *coord /= scalar;
        }
    }
}
