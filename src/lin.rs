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
            x: -self.y,
            y: self.x
        }
    }

    pub fn perp_cc(&self) -> Self {
        Self {
            x: self.y,
            y: -self.x
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

fn signed_area(start: Vec2, end: Vec2, point: Vec2) -> f32 {
    let from_start = point.sub(start);
    let side = end.sub(start).perp();

    from_start.dot(side)
}

impl Triangle2 {
    pub fn depth_at(&self, point: Vec2) -> Option<Vec3> {
        let area_a = signed_area(self.a, self.b, point);
        let area_b = signed_area(self.b, self.c, point);
        let area_c = signed_area(self.c, self.a, point);

        if area_a <= 0. || area_b <= 0. || area_c <= 0. {
            // cull backfaces
            return None;
        }

        let total_area = area_a + area_b + area_c;
        if total_area <= 0. {
            return None;
        }

        let inv_ta = total_area.recip();
        let a_weight = area_a * inv_ta;
        let b_weight = area_b * inv_ta;
        let c_weight = area_c * inv_ta;

        Some(Vec3::new(a_weight, b_weight, c_weight))
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

    pub fn recip(&self) -> Self {
        Self {
            x: self.x.recip(),
            y: self.y.recip(),
            z: self.z.recip()
        }
    }

    pub fn dot(&self, other: Self) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn trunc(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Triangle3 {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3
}

impl Triangle3 {
    pub fn trunc(&self) -> Triangle2 {
        Triangle2 {
            a: self.a.trunc(),
            b: self.b.trunc(),
            c: self.c.trunc()
        }
    }
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
