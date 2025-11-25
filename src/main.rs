pub mod color;
pub mod graphics;

use color::RGBA8;
use graphics::*;

fn dump_image(data: &Image<u32>) {
    let (width, height) = data.size();
    let mut image = bmp::Image::new(width as u32, height as u32);

    for (x, y) in image.coordinates() {
        let color = match data.at(x as usize, y as usize) {
            Some(value) => RGBA8::from(value.clone()),
            None => RGBA8::default(),
        };

        image.set_pixel(
            x,
            y,
            bmp::Pixel {
                r: color.r,
                g: color.g,
                b: color.b,
            },
        );
    }

    image.save("dump.bmp").unwrap();
}

fn main() {
    let rast = Rasterizer {
        // uh
    };

    let width = 1600;
    let height = 900;

    let mut fb = Framebuffer {
        color: vec![Image::new(width, height)],
        depth: None,

        width: width,
        height: height,
    };

    rast.clear_framebuffer(
        &mut fb,
        &ClearValue {
            color: 0x00FF00FF,
            depth: 1.0,
        },
    )
    .expect("Failed to clear framebuffer!");

    dump_image(&fb.color[0]);
}
