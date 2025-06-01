use crate::lin::Vec3;

#[derive(Default, Debug)]
pub struct Obj {
    pub verts: Vec<Vec3>,
    pub faces: Vec<Vec<usize>>
}

impl Obj {
    pub fn parse(file: String) -> anyhow::Result<Self> {
        let mut obj = Self::default();

        for line in file.lines() {
            let line = line
                .split_once('#')
                .map(|l| l.0)
                .unwrap_or(line)
                .trim();

            if line.is_empty() {
                continue;
            }

            let tokens = line.split_whitespace().collect::<Vec<_>>();

            match tokens[0] {
                "v" => obj.verts.push(Vec3 {
                    x: tokens[1].parse()?,
                    y: tokens[2].parse()?,
                    z: tokens[3].parse()?,
                }),
                "f" => obj.faces.push(tokens[1..].iter().map(|tok| {
                    tok.split('/').next().unwrap().parse()
                }).collect::<Result<Vec<usize>, _>>()?),
                "vt" | "vn" | "vp" | "l" | "s" | "o" | "g" => {},
                token => anyhow::bail!("unknown token: {token}")
            }
        }

        Ok(obj)
    }
}
