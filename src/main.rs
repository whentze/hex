use std::io::{self, BufWriter, Write};
use termion::{
    color::{self, Bg, Fg, Rgb},
    cursor,
};

/// A hexagon given by its staggered coordinates
#[derive(Copy, Clone)]
struct Hex {
    x: u32,
    y: u32,
}

impl Hex {
    /// the Hex at the given terminal cell
    fn at_cell(col: u32, row: u32) -> Self {
        Self {
            x: col / 3,
            y: (row + (col / 3).rem_euclid(2)) / 2,
        }
    }
    /// the center of this Hex, in terminal coords
    fn center(&self) -> (u32, u32) {
        (self.x * 3, self.y * 2 + (self.x + 1).rem_euclid(2))
    }
}

fn color(c: u32, r: u32, t: u32) -> Rgb {
    let (c, r) = Hex::at_cell(c, r).center();

    Rgb(
        ((((4 * c + 14 * r + t) as f32) / 80.0).sin() * 256.0) as u8,
        0,
        ((((12 * c + 6 * r + t) as f32) / 50.0).sin() * 256.0) as u8,
    )
}

fn main() -> Result<(), io::Error> {
    let (width, height) = (80, 24);

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut buf = BufWriter::with_capacity(
        640_000, /* ought to be enough for anybody */
        &mut stdout,
    );

    for t in 0.. {
        for row in 0..height {
            for col in 0..width {
                write!(
                    buf,
                    "{}{}{}",
                    Bg(color(col, row, t)),
                    Fg(color(col + 1, row, t)),
                    match (col + 3 * row) % 6 {
                        2 => "◥",
                        5 => "◢",
                        _ => " ",
                    }
                )?;
            }
            writeln!(buf, "{}{}", Fg(color::Reset), Bg(color::Reset))?;
        }
        write!(buf, "{}", cursor::Up(height as u16))?;
        std::thread::sleep(std::time::Duration::from_millis(20));
        buf.flush()?;
    }

    Ok(())
}
