use rawr_xd::renderer::{Renderer, fb, terminal};
use rawr_xd::model::Model;
use rawr_xd::obj::Obj;
use rawr_xd::lin::{Vec3, Transform};

use std::error::Error;
use std::env::args;
use std::fs::read_to_string;
use std::f32::consts::TAU;

fn main() -> Result<(), Box<dyn Error>> {
    let Some(renderer_type) = args().nth(1) else { panic!("provide an argument to determine the type of renderer") };

    let mut renderer: Box<dyn Renderer> = match renderer_type.as_str() {
        "terminal" => {
            let mut renderer = terminal::TerminalRenderer::new(0, 0);
            renderer.fit()?;
            renderer.init();

            Box::new(renderer)
        }
        "fb" => Box::new(fb::MinifbRenderer::new(640, 480)?),
        _ => panic!("invalid renderer type (available: fb, terminal)")
    };

    let monke = Obj::parse(read_to_string("obj/monke.obj")?)?;
    let model = Model::from_obj(monke);

    let mut angle = 0.;

    loop {
        angle = (angle + 0.007) % TAU;

        renderer.clear();
        renderer.draw_model(&model, Transform { yaw: angle, pitch: 0., roll: 0., translation: Vec3::new(0., 0., 3.) });

        renderer.commit()?;
    }
}
