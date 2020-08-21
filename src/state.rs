use winit::window::Window;
use winit::event::WindowEvent;
use wgpu::{Instance, BackendBit, Operations, Device, ShaderModuleSource, PrimitiveTopology};
use wgpu::LoadOp::Clear;
use winit::dpi::PhysicalPosition;
use std::borrow::Cow;

pub struct State {
    instance: wgpu::Instance,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    render_pipeline: wgpu::RenderPipeline,

    size: winit::dpi::PhysicalSize<u32>,
    cursor_position: Option<PhysicalPosition<f64>>,
}

// fn load_shaders(device: &Device) {
//     // let vs_src = include_str!("shader.vert");
//     // let fs_src = include_str!("shader.frag");
//     //
//     // let mut compiler = shaderc::Compiler::new().unwrap();
//     // let vs_spirv = compiler.compile_into_spirv(vs_src, shaderc::ShaderKind::Vertex, "shader.vert", "main", None).unwrap();
//     // let fs_spirv = compiler.compile_into_spirv(fs_src, shaderc::ShaderKind::Fragment, "shader.frag", "main", None).unwrap();
//     //
//     // // let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
//     // // let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();
//     // let vs_module = device.create_shader_module(ShaderModuleSource);
// }

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

        let vs_module = device.create_shader_module(ShaderModuleSource::Wgsl(Cow::from(include_str!("shader.vert"))));
        let fs_module = device.create_shader_module(ShaderModuleSource::Wgsl(Cow::from(include_str!("shader.frag"))));
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: None,
            primitive_topology: PrimitiveTopology::PointList,
            color_states: &[],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: Default::default(),
                vertex_buffers: &[],
            },
            sample_count: 0,
            sample_mask: 0,
            alpha_to_coverage_enabled: false,
        });

        Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            render_pipeline,
            size,
            cursor_position: None,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    // input() won't deal with GPU code, so it can be synchronous
    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = Some(*position);
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) {
        let frame = self.swap_chain.get_current_frame()
            .expect("Timeout getting texture");

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        ops: Operations {
                            load: Clear(
                                match self.cursor_position {
                                    Some(pos) => wgpu::Color {
                                        r: pos.x / self.size.width as f64,
                                        g: pos.y / self.size.height as f64,
                                        b: 0.3,
                                        a: 1.0,
                                    },
                                    None => wgpu::Color {
                                        r: 0.1,
                                        g: 0.2,
                                        b: 0.3,
                                        a: 1.0,
                                    }
                                }
                            ),
                            store: true,
                        },
                    }
                ],
                depth_stencil_attachment: None,
            });
        }

        // TODO: How can we avoid this allocation it does not like using an array for some reason.
        self.queue.submit(vec![
            encoder.finish()
        ]);
        // self.queue.submit(&[
        //     encoder.finish()
        // ]);
    }
}
