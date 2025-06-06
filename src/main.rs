pub mod model;
pub mod lin;
pub mod renderer;
pub mod obj;

use renderer::Renderer;

fn main() -> anyhow::Result<()> {
    let cube = obj::Obj::parse(std::fs::read_to_string("obj/monke.obj")?)?;
    let model = model::Model::from_obj(cube);

    let mut renderer = renderer::fb::MinifbRenderer::new(640, 480)?;
    //let mut renderer = renderer::terminal::TerminalRenderer::new(0, 0);
    //renderer.fit()?;
    //renderer.init();

    let mut angle = 0.;

    loop {
        angle = (angle + 0.1) % std::f32::consts::TAU;

        renderer.clear();
        renderer.draw_model(&model, lin::Transform { yaw: angle, pitch: 0., roll: 0., translation: lin::Vec3::new(0., 0., 0.) });

        renderer.commit()?;

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
