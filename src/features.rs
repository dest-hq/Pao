/// Custom render feature that can be added to `Canvas`
///
/// With [`RenderFeature`], you can render the custom stuff that you are missing in `Canvas`
pub trait RenderFeature {
    /// Called before rendering to prepare the data
    fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue);

    /// Called during the render pass
    fn render(&mut self, pass: &mut wgpu::RenderPass, device: &wgpu::Device, queue: &wgpu::Queue);
}
