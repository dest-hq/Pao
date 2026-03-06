pub trait RenderFeature {
    fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue);

    fn render(&mut self, pass: &mut wgpu::RenderPass);
}
