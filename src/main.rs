mod model;
mod lin;
mod renderer;
mod obj;

use renderer::Renderer;

fn main() -> anyhow::Result<()> {
    let cube = obj::Obj::parse(r##"
v -1.0 -1.0 -1.0  # 1 a
v -1.0 1.0 -1.0  # 2 b
v 1.0 1.0 -1.0  # 3 c
v 1.0 -1.0 -1.0  # 4 d
v -1.0 -1.0 1.0  # 5 e
v -1.0 1.0 1.0  # 6 f
v 1.0 1.0 1.0  # 7 g
v 1.0 -1.0 1.0  # 8 h

# Normal vectors
# One for each face. Shared by all vertices in that face.
vn  1.0  0.0  0.0  # 1 cghd
vn -1.0  0.0  0.0  # 2 aefb
vn  0.0  1.0  0.0  # 3 gcbf
vn  0.0 -1.0  0.0  # 4 dhea
vn  0.0  0.0  1.0  # 5 hgfe
vn  0.0  0.0 -1.0  # 6 cdab

# Faces v/vt/vn
#   3-------2
#   | -     |
#   |   #   |  Each face = 2 triangles (ccw)
#   |     - |            = 1-2-3 + 1-3-4
#   4-------1

# Face 1: cghd = cgh + chd
f 3//1 7//1 8//1
f 3//1 8//1 4//1

# Face 2: aefb = aef + afb
f 1//2 5//2 6//2
f 1//2 6//2 2//2

# Face 3: gcbf = gcb + gbf
f 7//3 3//3 2//3
f 7//3 2//3 6//3

# Face 4: dhea = dhe + dea
f 4//4 8//4 5//4
f 4//4 5//4 1//4

# Face 5: hgfe = hgf + hfe
f 8//5 7//5 6//5
f 8//5 6//5 5//5

# Face 6: cdab = cda + cab
f 3//6 4//6 1//6
f 3//6 1//6 2//6
"##.into())?;
    let model = model::Model::from_obj(cube);

    let mut renderer = renderer::fb::MinifbRenderer::new(640, 480)?;
    //let mut renderer = renderer::terminal::TerminalRenderer::new(0, 0);
    //renderer.fit()?;
    //renderer.init();

    let mut yaw = 0.;

    loop {
        yaw = (yaw + 0.1) % 6.28;

        for (triangle, color) in model.as_triangle2s(lin::Transform { yaw, pitch: yaw, translation: lin::Vec3::new(0., 0., 0.) }, &renderer).into_iter().zip(model.colors.iter()) {
            renderer.draw_triangle(triangle, *color);
        }

        renderer.commit()?;
        renderer.clear();

        std::thread::sleep(std::time::Duration::from_millis(30));
    }
}
