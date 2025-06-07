use crate::obj::Obj;
use crate::lin::{Vec2, Vec3, Triangle3, Transform};
use crate::renderer::Color;

pub fn world_to_screen_and_depth(point: Vec3, transform: Transform, fov: f32, screen_size: Vec2) -> Vec3 {
    let screen_height = (fov / 2.).to_radians().tan() * 2.;
    let point = transform.apply(point);
    let pixels_per_world = screen_size.y / screen_height / point.z;
    let scaled = Vec2::new(point.x, point.y).mul(pixels_per_world);
    let centered_scaled = scaled.add(screen_size.mul(0.5));

    Vec3::new(centered_scaled.x, centered_scaled.y, point.z)
}

#[derive(Default, Debug)]
pub struct Model {
    triangles: Vec<Triangle3>,
    pub colors: Vec<Color>,
}

impl Model {
    pub fn from_obj(obj: Obj) -> Self {
        let mut model = Model::default();

        for face in obj.faces {
            for window in face[1..].windows(2) {
                model.triangles.push(Triangle3 {
                    a: obj.verts[face[0] - 1],
                    b: obj.verts[window[0] - 1],
                    c: obj.verts[window[1] - 1]
                });
                model.colors.push(Color::random());
            }
        }

        model
    }

    pub fn as_projected_triangles(&self, transform: Transform, screen_size: Vec2) -> Vec<Triangle3> {
        let fov = 60.;
        self.triangles
            .iter()
            .map(|triangle3| Triangle3 {
                a: world_to_screen_and_depth(triangle3.a, transform, fov, screen_size),
                b: world_to_screen_and_depth(triangle3.b, transform, fov, screen_size),
                c: world_to_screen_and_depth(triangle3.c, transform, fov, screen_size)
            })
            .collect()
    }
}
