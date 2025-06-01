use super::*;

#[repr(C)]
struct UnixSize {
    rows: c_ushort,
    cols: c_ushort,
    x: c_ushort,
    y: c_ushort,
}


pub struct TerminalRenderer {
    pub rows: u32,
    pub cols: u32,
    codes: String,
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        println!("\x1b[?1049l"); // switch buffer back
    }
}

impl TerminalRenderer {
    // https://stackoverflow.com/a/74186686
    const BRIGHTNESS_MAP: &'static [u8] = b" ` .-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

    pub fn new(cols: u32, rows: u32) -> Self {
        Self { rows, cols, codes: String::new() }
    }

    pub fn fit(&mut self) -> anyhow::Result<()> {
        let mut us = UnixSize {
            rows: 0,
            cols: 0,
            x: 0,
            y: 0,
        };
        let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut us) };
        if r != 0 {
            anyhow::bail!("could not fit terminal");
        }
        self.rows = us.rows as u32;
        self.cols = us.cols as u32;

        Ok(())
    }

    pub fn push_code(&mut self, s: impl AsRef<str>) {
        self.codes.push_str(s.as_ref());
    }

    pub fn init(&mut self) {
        println!("\x1b[?1049h");
        self.clear();
        self.push_code("\x1b[?25l"); // hide cursor
    }
}

impl Renderer for TerminalRenderer {
    fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.cols || y >= self.rows {
            return;
        }

        //let ch = Self::BRIGHTNESS_MAP[(color.percieved_luminance() * (Self::BRIGHTNESS_MAP.len() as f32 - 1.)) as usize];
        self.push_code(format!("\x1b[{};{}f\x1b[38;2;{};{};{}m█\x1b[H", y + 1, x + 1, color.r, color.g, color.b)); // go to position and draw char
    }

    fn size(&self) -> (u32, u32) {
        (self.cols, self.rows)
    }

    fn clear(&mut self) {
        self.push_code("\x1b[2J\x1b[H");
    }

    fn commit(&mut self) -> anyhow::Result<()> {
        println!("{}", std::mem::take(&mut self.codes));
        Ok(())
    }
}
