use anyhow::anyhow;
use sdl2::{
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    surface::{self},
    sys::{SDL_FillRect, SDL_Rect},
    video::Window,
};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

impl Circle {
    fn new(x: i32, y: i32, radius: i32) -> Self {
        Circle { x, y, radius }
    }
}

fn DrawCircle(
    canvas: &mut Canvas<Window>,
    x: i32,
    y: i32,
    radius: i32,
) -> Result<(), anyhow::Error> {
    let c = Circle::new(x, y, radius);
    let squared_radius = c.radius.pow(2);
    canvas.set_draw_color(Color::RED);
    for x in c.x - c.radius..c.x + c.radius {
        for y in c.y - c.radius..c.y + c.radius {
            let dist_squared = (x - c.x).pow(2) + (y - c.y).pow(2);

            if dist_squared < squared_radius {
                canvas
                    .fill_rect(Rect::new(x, y, 1, 1))
                    .map_err(|e| anyhow!("Error drawing pixel {}", e))?;
            }
        }
    }

    Ok(())
}
fn main() -> Result<(), anyhow::Error> {
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    let window = video
        .window("Raytracing", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    DrawCircle(&mut canvas, (WIDTH / 2) as i32, (HEIGHT / 2) as i32, 200)?;

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
