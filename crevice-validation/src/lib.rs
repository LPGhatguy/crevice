#![cfg(test)]

use std::borrow::Cow;

use crevice::std140::{AsStd140, Std140};
use futures::executor::block_on;
use mint::Vector2;
use wgpu::util::DeviceExt;

macro_rules! run_test {
    ($shader_path:literal, $ty:ident {
        $(
            $key:ident : $value:expr,
        )+
    }) => {
        let (device, queue) = setup();

        let input = $ty {
            $($key: $value,)+
        };

        let output = round_trip(
            &device,
            &queue,
            include_str!($shader_path),
            &input,
        );

        let expected = input.as_std140();

        $(assert_eq!(output.$key, expected.$key);)+
    }
}

#[test]
fn vec2() {
    #[derive(AsStd140)]
    struct TestData {
        two: Vector2<f32>,
    }

    run_test!(
        "../shaders/vec2.comp",
        TestData {
            two: Vector2 { x: 1.0, y: 2.0 },
        }
    );
}

fn setup() -> (wgpu::Device, wgpu::Queue) {
    // Instantiates instance of WebGPU
    let instance = wgpu::Instance::new(wgpu::Backends::all());

    // `request_adapter` instantiates the general connection to the GPU
    let adapter =
        block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).unwrap();

    // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
    //  `features` being the available features.
    block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::downlevel_defaults(),
        },
        None,
    ))
    .unwrap()
}

fn round_trip<T: AsStd140>(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    glsl_shader: &str,
    value: &T,
) -> <T as AsStd140>::Std140Type {
    let shader = compile(glsl_shader);

    let mut data = Vec::new();
    data.extend_from_slice(value.as_std140().as_bytes());

    let input_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Input Buffer"),
        contents: &data,
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    let output_gpu_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: data.len() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });

    let output_cpu_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Output Buffer"),
        size: data.len() as wgpu::BufferAddress,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let cs_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Owned(shader)),
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &cs_module,
        entry_point: "main",
    });

    // Instantiates the bind group, once again specifying the binding of buffers.
    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output_gpu_buffer.as_entire_binding(),
            },
        ],
    });

    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.dispatch(1, 1, 1);
    }

    encoder.copy_buffer_to_buffer(
        &output_gpu_buffer,
        0,
        &output_cpu_buffer,
        0,
        data.len() as wgpu::BufferAddress,
    );

    queue.submit([encoder.finish()]);

    let output_slice = output_cpu_buffer.slice(..);
    let output_future = output_slice.map_async(wgpu::MapMode::Read);

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::Wait);

    block_on(output_future).unwrap();

    let output = output_slice.get_mapped_range();
    let result = bytemuck::from_bytes::<<T as AsStd140>::Std140Type>(&output).clone();

    drop(output);
    output_cpu_buffer.unmap();

    result
}

fn compile(glsl_source: &str) -> String {
    let mut parser = naga::front::glsl::Parser::default();

    let module = parser
        .parse(
            &naga::front::glsl::Options {
                stage: naga::ShaderStage::Compute,
                defines: Default::default(),
            },
            glsl_source,
        )
        .unwrap();

    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::default(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)
    .unwrap();

    let wgsl = naga::back::wgsl::write_string(&module, &info).unwrap();

    wgsl
}
