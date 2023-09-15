use crate::pbrt::*;

#[derive(Copy, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> From<Point3<T>> for Vector3<T> {
    fn from(p: Point3<T>) -> Self {
        return Self {
            x: p.x,
            y: p.y,
            z: p.z,
        };
    }
}

impl From<Normal3f> for Vector3f {
    fn from(n: Normal3f) -> Self {
        return Self {
            x: n.x,
            y: n.y,
            z: n.z,
        };
    }
}

impl From<Vector3<Float>> for Vector3<Interval> {
    fn from(value: Vector3<Float>) -> Self {
        return Vector3::<Interval>::new(
            Interval::from(value.x),
            Interval::from(value.y),
            Interval::from(value.z),
        );
    }
}

impl From<Point3<Float>> for Vector3<Interval> {
    fn from(value: Point3<Float>) -> Self {
        return Vector3::<Interval>::new(
            Interval::from(value.x),
            Interval::from(value.y),
            Interval::from(value.z),
        );
    }
}

impl<T: Copy + Add<Output = T> + Mul<Output = T>> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        return Self { x, y, z };
    }

    pub fn permute(&self, p: [usize; 3]) -> Vector3<T> {
        return Vector3::<T> {
            x: self[p[0]],
            y: self[p[1]],
            z: self[p[2]],
        };
    }

    pub fn length_squared(&self) -> T {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}

impl Vector3<Float> {
    pub fn nan() -> Self {
        return Self {
            x: Float::NAN,
            y: Float::NAN,
            z: Float::NAN,
        };
    }

    pub fn abs(&self) -> Vector3<Float> {
        return Vector3::<Float> {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        };
    }

    pub fn dot(&self, v: Self) -> Float {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }

    pub fn length(&self) -> Float {
        return Float::sqrt(self.length_squared());
    }

    pub fn normalize(&self) -> Vector3f {
        return *self / self.length();
    }

    pub fn cross(&self, rhs: Self) -> Self {
        return Self {
            x: difference_of_products(self.y, rhs.z, self.z, rhs.y),
            y: difference_of_products(self.z, rhs.x, self.x, rhs.z),
            z: difference_of_products(self.x, rhs.y, self.y, rhs.x),
        };
    }

    pub fn face_forward(&self, v: Self) -> Self {
        return if self.dot(v) >= 0.0 { *self } else { -*self };
    }

    pub fn max_component_index(&self) -> usize {
        return if self.x > self.y {
            if self.x > self.z {
                0
            } else {
                2
            }
        } else {
            if self.y > self.z {
                1
            } else {
                2
            }
        };
    }

    pub fn max_component_value(&self) -> Float {
        return self.x.max(self.y).max(self.z);
    }

    pub fn coordinate_system(&self) -> (Vector3f, Vector3f) {
        let sign = (1.0 as Float).copysign(self.z);

        let a = -1.0 / (sign + self.z);
        let b = self.x * self.y * a;

        let v2 = Vector3f::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x);
        let v3 = Vector3f::new(b, sign + self.y * self.y * a, -self.y);

        return (v2, v3);
    }

    pub fn softmax_color(&self) -> RGBColor {
        let base: Float = 10.0;
        let x = base.powf(self.x);
        let y = base.powf(self.y);
        let z = base.powf(self.z);

        let sum = x + y + z;
        return RGBColor {
            r: x / sum,
            g: y / sum,
            b: z / sum,
        };
    }
}

impl Vector3<Interval> {
    pub fn new_with_error(v: Vector3f, error: Vector3f) -> Self {
        return Vector3fi {
            x: Interval::from_value_and_error(v.x, error.x),
            y: Interval::from_value_and_error(v.y, error.y),
            z: Interval::from_value_and_error(v.z, error.z),
        };
    }

    pub fn is_exact(&self) -> bool {
        return self.x.width() == 0.0 && self.y.width() == 0.0 && self.z.width() == 0.0;
    }

    pub fn error(&self) -> Vector3<Float> {
        return Vector3::<Float> {
            x: self.x.width() / 2.0,
            y: self.y.width() / 2.0,
            z: self.z.width() / 2.0,
        };
    }

    pub fn length(&self) -> Interval {
        return self.length_squared().sqrt();
    }
}

impl<T: Display> Display for Vector3<T> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<Vector3<Interval>> for Vector3<Float> {
    fn from(value: Vector3<Interval>) -> Self {
        return Self {
            x: value.x.midpoint(),
            y: value.y.midpoint(),
            z: value.z.midpoint(),
        };
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("Vector3: illegal index: {}", index);
            }
        };
    }
}

impl<T: Copy + Add<T, Output = T>> Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        return Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl<T: Neg<Output = T>> Neg for Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Self::Output {
        return Vector3::<T> {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl<T: Copy + Mul<T, Output = T>> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        return Vector3::<T> {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl Mul<Vector3<Float>> for Float {
    type Output = Vector3<Float>;

    fn mul(self, rhs: Vector3<Float>) -> Self::Output {
        return rhs * self;
    }
}

impl Div<Float> for Vector3<Float> {
    type Output = Vector3<Float>;

    fn div(self, rhs: Float) -> Self::Output {
        let one_over_rhs = 1.0 / rhs;
        return Vector3::<Float> {
            x: self.x * one_over_rhs,
            y: self.y * one_over_rhs,
            z: self.z * one_over_rhs,
        };
    }
}