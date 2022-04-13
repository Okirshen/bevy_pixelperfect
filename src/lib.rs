use bevy::{prelude::*, winit::WinitWindows};
use components::*;
use image::{imageops::FilterType, DynamicImage};
use pixels::{Pixels, SurfaceTexture};
use std::cmp::{max, min};

mod components;

pub mod prelude {
    pub use crate::{components::*, PixelPerfectPlugin};
}

// PixelPerfectPlugin
pub struct PixelPerfectPlugin;

struct PixelsResource {
    pub pixels: Pixels,
}

impl Plugin for PixelPerfectPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(render_system)
            .add_system(scale_system);
    }
}

fn setup_system(mut commands: Commands, windows: Res<Windows>, winit_windows: Res<WinitWindows>) {
    let window_id = windows.get_primary().expect("window not found").id();

    let winit_window = winit_windows
        .get_window(window_id)
        .expect("winit window fetch failed");

    let window_size = winit_window.inner_size();

    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, winit_window);
    let pixels = Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

    commands.insert_resource(PixelsResource { pixels })
}

fn scale_system(mut q: Query<(&mut Image, &Scale)>) {
    for (mut img, scale) in q.iter_mut() {
        if img.data.width() != img.width * scale.0 {
            img.data = DynamicImage::from(img.data.clone())
                .resize(
                    img.width * scale.0,
                    img.height * scale.0,
                    FilterType::Nearest,
                )
                .to_rgba8();
        }
    }
}

fn render_system(
    mut resource: ResMut<PixelsResource>,
    windows: Res<Windows>,
    mut q: QuerySet<(
        QueryState<&Position, With<Camera>>,
        QueryState<(&Position, &Image)>,
    )>,
) {
    let cam_pos = *q.q0().get_single().expect("No camera comonent found");
    let win = windows.get_primary().expect("Failed to get primary window");
    let frame = resource.pixels.get_frame();
    // Top Left Camera Corner
    let corner = Position {
        x: cam_pos.x - win.width() as i16 / 2,
        y: cam_pos.y - win.height() as i16 / 2,
    };
    for (pos, img) in q.q1().iter() {
        let width = img.data.width() as i16;
        let height = img.data.height() as i16;
        if pos.x >= (win.width() as i16 + corner.x)
            || pos.y >= (win.height() as i16 + corner.y)
            || (pos.x + width as i16) <= corner.x
            || (pos.y + height as i16) <= corner.y
        {
            continue;
        }

        let p1 = Position {
            x: max(pos.x, corner.x),
            y: max(pos.y, corner.y),
        };
        let p2 = Position {
            x: min(pos.x + width, corner.x + win.width() as i16),
            y: min(pos.y + height, corner.y + win.height() as i16),
        };

        let (lp, rp) = if p1.x < p2.x { (p1, p2) } else { (p2, p1) };

        let ld = lp - *pos;
        let rd = rp - *pos;
        for y in ld.y..rd.y {
            for x in ld.x..rd.x {
                let i = ((pos.y - corner.y + y) as i32 * win.width() as i32
                    + (pos.x - corner.x + x) as i32) as usize
                    * 4;
                frame[i..i + 4].copy_from_slice(&img.data.get_pixel(x as u32, y as u32).0)
            }
        }
    }
    resource.pixels.render().expect("Failed to render frame");
}
