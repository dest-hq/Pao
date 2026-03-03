use primit::Color;
use wgpu::{
    CommandEncoderDescriptor, LoadOp, Operations, RenderPassColorAttachment, RenderPassDescriptor,
    StoreOp, TextureViewDescriptor,
};

use crate::context::RenderContext;

pub struct Scene<'a, 'window> {
    background: Color,
    context: &'a RenderContext<'window>,
}

impl<'a, 'window> Scene<'a, 'window> {
    pub fn new(context: &'a RenderContext<'window>) -> Self {
        Self {
            background: Color::rgb8(0, 0, 0),
            context,
        }
    }

    pub fn render(&mut self, background: Color) {
        let frame = self.context.surface.get_current_texture().unwrap();
        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .context
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("pao encoder"),
            });

        {
            let _render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                multiview_mask: None,
                label: Some("pao render pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    depth_slice: None,
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(wgpu::Color {
                            r: background.r as f64 / 255.0,
                            g: background.g as f64 / 255.0,
                            b: background.b as f64 / 255.0,
                            a: background.a as f64 / 255.0,
                        }),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        self.context.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
