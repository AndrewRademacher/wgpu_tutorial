use winit::window::Window;
use winit::event::WindowEvent;
use wgpu::{Instance, BackendBit};

pub struct State {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            features: Default::default(),
            limits: Default::default(),
            shader_validation: false,
        }, None).await.unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        unimplemented!()
    }

    // input() won't deal with GPU code, so it can be synchronous
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        unimplemented!()
    }

    pub fn update(&mut self) {
        unimplemented!()
    }

    pub fn render(&mut self) {
        unimplemented!()
    }
}
