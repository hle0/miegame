use image::{Pixel, RgbaImage};
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::{Texture, TextureCreator};

pub trait ToTexture {
    fn to_texture<'a, U>(&self, creator: &'a TextureCreator<U>) -> Texture<'a>;
}

impl ToTexture for RgbaImage {
    fn to_texture<'a, U>(&self, creator: &'a TextureCreator<U>) -> Texture<'a> {
        let width = self.width();
        let height = self.height();

        let texture = {
            let mut texture = creator
                .create_texture_streaming(Some(PixelFormatEnum::RGBA8888), width, height)
                .unwrap();

            texture
                .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    for x in 0usize..(width as usize) {
                        for y in 0usize..(height as usize) {
                            let offset = 4 * x + y * pitch;
                            buffer[offset..offset + 4].copy_from_slice(
                                &self.get_pixel(x as u32, y as u32).to_rgba().0[..],
                            );
                        }
                    }
                })
                .unwrap();

            texture
        };

        texture
    }
}
