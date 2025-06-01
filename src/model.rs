use crate::obj::Obj;
use crate::lin::{Vec2, Vec3, Triangle2, Triangle3, Transform};
use crate::renderer::{Renderer, Color};

pub fn world_to_screen(point: Vec3, transform: Transform, renderer: &impl Renderer) -> Vec2 {
    let point = transform.apply(point);
    let size = renderer.size();
    let pixels_per_world = size.1 as f32 / 3.;
    let scaled = Vec2::new(point.x, point.y).mul(pixels_per_world);

    scaled.add(Vec2::new(size.0 as f32 / 2., size.1 as f32 / 2.))
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
            for window in face.windows(3) {
                model.triangles.push(Triangle3 {
                    a: obj.verts[window[0] - 1],
                    b: obj.verts[window[1] - 1],
                    c: obj.verts[window[2] - 1]
                });
                model.colors.push(Color::random());
            }
        }

        model
    }

    pub fn as_triangle2s(&self, transform: Transform, renderer: &impl Renderer) -> Vec<Triangle2> {
        self.triangles
            .iter()
            .map(|triangle3| Triangle2 {
                a: world_to_screen(triangle3.a, transform, renderer),
                b: world_to_screen(triangle3.b, transform, renderer),
                c: world_to_screen(triangle3.c, transform, renderer)
            })
            .collect()
    }
}
