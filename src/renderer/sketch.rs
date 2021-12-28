use crate::prelude::*;

pub struct Sketch {
    pub pipeline: wgpu::RenderPipeline,
    pub form_idx: usize,
}

impl Sketch {
    pub fn new(renderer: &Renderer, shade_idx: usize, form_idx: usize) -> Self {
        let shade = renderer
            .shades
            .get(shade_idx)
            .expect("shader index invalid");
        let form = renderer.form(form_idx);

        let render_pipeline_layout =
            renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let create_pipeline = |attr: &[wgpu::VertexBufferLayout]| {
            renderer
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shade.shader,
                        entry_point: "vs_main",
                        buffers: attr,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shade.shader,
                        entry_point: "fs_main",
                        targets: &[wgpu::ColorTargetState {
                            format: renderer.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::all(),
                        }],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                        polygon_mode: wgpu::PolygonMode::Fill,
                        // Requires Features::DEPTH_CLAMPING
                        unclipped_depth: false,
                        // Requires Features::CONSERVATIVE_RASTERIZATION
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                })
        };

        let pipeline = match form {
            Form::SimpleRange { .. } => create_pipeline(&[]),
            Form::Vertices(buf) => {
                let layout = wgpu::VertexBufferLayout {
                    array_stride: buf.array_stride,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: buf.attributes.as_slice(),
                };
                create_pipeline(&[layout])
            }
            Form::IndexedVertices(buf) => {
                let layout = wgpu::VertexBufferLayout {
                    array_stride: buf.array_stride,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: buf.attributes.as_slice(),
                };
                create_pipeline(&[layout])
            }
        };

        Self { pipeline, form_idx }
    }
}
