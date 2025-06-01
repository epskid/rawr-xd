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

        //(right_of_a == right_of_b) && (right_of_b == right_of_c)
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
    pub translation: Vec3
}

fn transform(i: Vec3, j: Vec3, k: Vec3, point: Vec3) -> Vec3 {
    i.mul(point.x)
        .add(j.mul(point.y)
            .add(k.mul(point.z)))
}

impl Transform {
    fn get_basis(&self) -> (Vec3, Vec3, Vec3) {
        let yaw = (Vec3::new(self.yaw.cos(), 0., self.yaw.sin()), Vec3::new(0., 1., 0.), Vec3::new(-self.yaw.sin(), 0., self.yaw.cos()));
        let pitch = (Vec3::new(1., 0., 0.), Vec3::new(0., self.pitch.cos(), -self.pitch.sin()), Vec3::new(0., self.pitch.sin(), self.pitch.cos()));

        (
            transform(yaw.0, yaw.1, yaw.2, pitch.0),
            transform(yaw.0, yaw.1, yaw.2, pitch.1),
            transform(yaw.0, yaw.1, yaw.2, pitch.2),
        )
    }

    pub fn apply(&self, point: Vec3) -> Vec3 {
        let (i, j, k) = self.get_basis();

        transform(i, j, k, point).add(self.translation)
    }
}
