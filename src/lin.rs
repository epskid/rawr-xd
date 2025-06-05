fn on_right_side(start: Vec2, end: Vec2, point: Vec2) -> bool {
    let from_start = point.sub(start);
    let side = end.sub(start).perp();

    from_start.dot(side) >= 0.0
}

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn perp(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x
        }
    }

    pub fn perp_cc(&self) -> Self {
        Self {
            x: -self.y,
            y: self.x
        }
    }

    pub fn transpose(&self) -> Self {
        Self {
            x: self.y,
            y: self.x
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }

    pub fn sub(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }

    pub fn mul(&self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle2 {
    pub a: Vec2,
    pub b: Vec2,
    pub c: Vec2
}

impl Triangle2 {
    pub fn point_inside(&self, point: Vec2) -> bool {
        let right_of_a = on_right_side(self.a, self.b, point);
        let right_of_b = on_right_side(self.b, self.c, point);
        let right_of_c = on_right_side(self.c, self.a, point);

        // cull backfaces
        right_of_a && right_of_b && right_of_c
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }

    pub fn sub(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }

    pub fn mul(&self, scale: f32) -> Self {
        Self {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle3 {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3
}

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,
    pub translation: Vec3
}

impl Transform {
    fn apply_rotation(&self, point: Vec3) -> Vec3 {
        let (a, b, c) = (self.yaw, self.pitch, self.roll);
        // general rotation matrix created from multiplying all rotation matrices
        let (row_one, row_two, row_three) = (
            Vec3::new(a.cos() * b.cos(), (a.cos() * b.sin() * c.sin()) - (a.sin() * c.cos()), (a.cos() * b.sin() * c.cos()) + (a.sin() * c.sin())),
            Vec3::new(-b.sin(), b.cos() * c.sin(), b.cos() * c.cos()),
            Vec3::new(a.sin() * b.cos(), (a.sin() * b.sin() * c.sin()) + (a.cos() * c.cos()), (a.sin() * b.sin() * c.cos()) - (a.cos() * c.sin())),
        );

        Vec3::new(
            point.dot(row_one),
            point.dot(row_two),
            point.dot(row_three),
        )
    }

    pub fn apply(&self, point: Vec3) -> Vec3 {
        self.apply_rotation(point).add(self.translation)
    }
}
