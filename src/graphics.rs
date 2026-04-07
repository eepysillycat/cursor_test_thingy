use std::sync::Arc;

use anyhow::bail;
use glam::UVec2;
use wgpu::{
    Color, CommandEncoderDescriptor, CompositeAlphaMode, CurrentSurfaceTexture, Device,
    DeviceDescriptor, Instance, InstanceDescriptor, LoadOp, Operations, PresentMode, Queue,
    RenderPassColorAttachment, RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface,
    SurfaceConfiguration, TextureFormat, TextureUsages, TextureViewDescriptor,
};
use winit::{event_loop::OwnedDisplayHandle, window::Window};

pub struct Graphics {
    instance: Instance,
    device: Device,
    queue: Queue,
    surface: Option<Surface<'static>>,
    configured_size: Option<UVec2>,
}

impl Graphics {
    pub async fn new(display: OwnedDisplayHandle) -> anyhow::Result<Self> {
        let instance = Instance::new(InstanceDescriptor::new_with_display_handle(Box::new(
            display,
        )));

        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await?;

        let (device, queue) = adapter.request_device(&DeviceDescriptor::default()).await?;

        Ok(Self {
            instance,
            device,
            queue,
            surface: None,
            configured_size: None,
        })
    }

    pub fn attach_window(&mut self, window: Arc<Window>) -> anyhow::Result<()> {
        let surface = self.instance.create_surface(window)?;

        self.surface = Some(surface);

        Ok(())
    }

    pub fn draw(&mut self, size: UVec2) -> anyhow::Result<()> {
        let surface = match &self.surface {
            Some(surface) => surface,
            None => bail!("window should be attached first"),
        };

        if self.configured_size != Some(size) {
            surface.configure(
                &self.device,
                &SurfaceConfiguration {
                    usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                    format: TextureFormat::Bgra8Unorm,
                    width: size.x,
                    height: size.y,
                    present_mode: PresentMode::AutoVsync,
                    desired_maximum_frame_latency: 2,
                    alpha_mode: CompositeAlphaMode::Auto,
                    view_formats: vec![],
                },
            );

            self.configured_size = Some(size);
        }

        let surface_texture = match surface.get_current_texture() {
            CurrentSurfaceTexture::Success(texture)
            | CurrentSurfaceTexture::Suboptimal(texture) => texture,

            CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return Ok(()),

            CurrentSurfaceTexture::Outdated
            | CurrentSurfaceTexture::Lost
            | CurrentSurfaceTexture::Validation => bail!("failed to acquire next surface texture"),
        };

        let surface_texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &surface_texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color {
                            r: 0.5,
                            g: 0.25,
                            b: 0.5,
                            a: 1.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
        }

        self.queue.submit([encoder.finish()]);

        surface_texture.present();

        Ok(())
    }
}
