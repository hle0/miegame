use crate::procedural::texture::ToTexture;
use image::{Rgba, RgbaImage};
use rand::random;
use sdl2::pixels::PixelFormatEnum::RGBA8888;
use sdl2::surface::Surface;

#[test]
fn test_convert_texture() {
    let width = 16;
    let height = 16;

    let surface = Surface::new(width, height, RGBA8888).unwrap();
    let mut canvas = surface.into_canvas().unwrap();
    let texture_creator = canvas.texture_creator();

    let image = RgbaImage::from_fn(width, height, |_, _| Rgba::from(random::<[u8; 4]>()));

    let texture = image.to_texture(&texture_creator);

    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    canvas.surface_mut().with_lock(|buffer: &[u8]| {
        image.enumerate_pixels().for_each(|(x, y, pixel)| {
            let offset = 4 * (x + y * width) as usize;
            assert_eq!(buffer[offset..offset + 4], pixel.0[..]);
        });
    });
}
