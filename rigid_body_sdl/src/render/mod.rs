#![allow(dead_code)]
use rigid_body_core::render::{
    PIXEL_FORMAT,
    Renderer,
    RendererImpl,
};
use sdl2::{
    IntegerOrSdlError,
    pixels::PixelFormatEnum,
    render::{
	Canvas,
	TextureCreator,
	WindowCanvas,
    },
    surface::Surface,
    Sdl,
    video::WindowContext,
    VideoSubsystem,
};

type SurfaceCanvas<'a> = Canvas<Surface<'a>>;
pub type StrResult<T> = Result<T, String>;

pub struct RendererSDL {
    renderer_impl: RendererImpl,
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
	let window = video.window(window_name, window_size.0, window_size.1)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
	let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	Ok(Self {
	    renderer_impl: RendererImpl::new(window_size),
	    texture_creator: canvas.texture_creator(),
	    canvas,
	    _video: video,
	})
    }

    pub fn set_window_size(
	&mut self, window_size: (u32, u32),
    ) -> Result<(), IntegerOrSdlError> {
	self.canvas.window_mut().set_size(window_size.0, window_size.1)?;
	self.renderer_impl.set_window_size(window_size);
	Ok(())
    }

    pub fn present(&mut self) -> StrResult<()> {
	let (width, height) = self.canvas.window().size();
	self.canvas.copy(
	    &self.texture_creator
		.create_texture_from_surface(
		    Surface::from_data(
			self.renderer_impl.get_data_mut(),
			width, height,
			width*PIXEL_FORMAT as u32,
			PixelFormatEnum::RGB24,
		    )?,
		).map_err(|e| e.to_string())?,
	    None, None, 
	)?;
	self.canvas.present();
	Ok(())
    }
}

impl Renderer for RendererSDL {
    fn get(&self) -> &RendererImpl {
	&self.renderer_impl
    }
    fn get_mut(&mut self) -> &mut RendererImpl {
	&mut self.renderer_impl
    }
}
