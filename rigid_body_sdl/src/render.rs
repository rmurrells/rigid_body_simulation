#![allow(dead_code)]
use crate::StrResult;
use rigid_body_core::render::PIXEL_FORMAT;
pub use rigid_body_core::render::{Color, RenderOption};
use sdl2::{
    pixels::PixelFormatEnum,
    render::{Canvas, TextureCreator, WindowCanvas},
    surface::Surface,
    video::WindowContext,
    IntegerOrSdlError, Sdl, VideoSubsystem,
};

type SurfaceCanvas<'a> = Canvas<Surface<'a>>;

pub struct RendererSDL {
    texture_creator: TextureCreator<WindowContext>,
    canvas: WindowCanvas,
    _video: VideoSubsystem,
}

impl RendererSDL {
    pub fn new(
        context: &Sdl,
        window_name: &str,
        window_size: (u32, u32),
    ) -> Result<Self, String> {
        let video = context.video()?;
        let window = video
            .window(window_name, window_size.0, window_size.1)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Self {
            texture_creator: canvas.texture_creator(),
            canvas,
            _video: video,
        })
    }

    pub fn set_window_size(
        &mut self,
        window_size: (u32, u32),
    ) -> Result<(), IntegerOrSdlError> {
        self.canvas
            .window_mut()
            .set_size(window_size.0, window_size.1)?;
        Ok(())
    }

    pub fn present(&mut self, pixel_buffer: &mut [u8]) -> StrResult<()> {
        let (width, height) = self.canvas.window().size();
        self.canvas.copy(
            &self
                .texture_creator
                .create_texture_from_surface(Surface::from_data(
                    pixel_buffer,
                    width,
                    height,
                    width * PIXEL_FORMAT as u32,
                    PixelFormatEnum::RGBA32,
                )?)
                .map_err(|e| e.to_string())?,
            None,
            None,
        )?;
        self.canvas.present();
        Ok(())
    }
}
