use std::f64::consts::PI;

use anyhow::anyhow;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    sys::{SDL_Color, SDL_Delay},
    video::Window,
};

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const RAYS_NUMBER: usize = 100;
struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

struct Ray {
    pub start_x: i32,
    pub start_y: i32,
    pub angle: f64,
}

impl Circle {
    fn new(x: i32, y: i32, radius: i32) -> Self {
        Circle { x, y, radius }
    }

    fn DrawCircle(&self, canvas: &mut Canvas<Window>) -> Result<(), anyhow::Error> {
        let squared_radius = self.radius.pow(2);
        canvas.set_draw_color(Color::WHITE);
        for x in self.x - self.radius..self.x + self.radius {
            for y in self.y - self.radius..self.y + self.radius {
                let dist_squared = (x - self.x).pow(2) + (y - self.y).pow(2);

                if dist_squared < squared_radius {
                    canvas
                        .fill_rect(Rect::new(x, y, 1, 1))
                        .map_err(|e| anyhow!("Error drawing pixel {}", e))?;
                }
            }
        }

        Ok(())
    }

    fn init_rays(&self, rays: &mut Vec<Ray>) -> Result<(), anyhow::Error> {
        for i in 0..RAYS_NUMBER {
            let angle = (i as f64 / RAYS_NUMBER as f64) * 2 as f64 * PI;

            println!("{}", angle);

            rays.push(Ray {
                start_x: self.x,
                start_y: self.y,
                angle,
            });
        }
        Ok(())
    }
}

fn fill_rays(
    canvas: &mut Canvas<Window>,
    rays: &Vec<Ray>,
    color: u32,
) -> Result<(), anyhow::Error> {
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
    let mut circle = Circle::new(0, 0, 80);
    let shadow_circle = Circle::new(650, 300, 140);
    let erase_rect = Rect::new(0, 0, WIDTH, HEIGHT);
    let mut rays = vec![];
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    circle.init_rays(&mut rays)?;

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::MouseMotion { x, y, .. } => {
                    circle.x = x;
                    circle.y = y;
                    circle.init_rays(&mut rays)?;
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::BLACK);
        let _ = canvas.fill_rect(erase_rect);

        circle.DrawCircle(&mut canvas)?;
        shadow_circle.DrawCircle(&mut canvas)?;

        canvas.present();
    }

    Ok(())
}
