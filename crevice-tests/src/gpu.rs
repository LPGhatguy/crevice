use std::borrow::Cow;

use crevice::glsl::GlslStruct;
use crevice::std140::{AsStd140, Std140};
use futures::executor::block_on;
use wgpu::util::DeviceExt;

const BASE_SHADER: &str = "
#version 450

{struct_definition}

layout({layout}, set = 0, binding = 0) readonly buffer INPUT {
    {struct_name} in_data;
};

layout({layout}, set = 0, binding = 1) buffer OUTPUT {
    {struct_name} out_data;
};

void main() {
    out_data = in_data;
}";

pub fn assert_round_trip<T: std::fmt::Debug + PartialEq + AsStd140 + GlslStruct>(value: T) {
    let output = round_trip(&value);

    if value != output {
        println!(
            "Value did not round-trip through wgpu successfully.\n\
            Expected: {:?}\n\
            Actual:   {:?}\n\n\
            GLSL definition: {}",
            value,
            output,
            T::glsl_definition()
        );

        panic!("wgpu round-trip failure for {}", T::NAME);
    }
}

fn round_trip<T: AsStd140 + GlslStruct>(value: &T) -> T {
    let (device, queue) = setup();

    let glsl_shader = BASE_SHADER
        .replace("{struct_name}", T::NAME)
        .replace("{struct_definition}", &T::glsl_definition())
        .replace("{layout}", "std140");

    let shader = match compile(&glsl_shader) {
        Ok(shader) => shader,
        Err(err) => {
            eprintln!("Bad shader: {}", glsl_shader);
            panic!("{}", err);
        }
    };

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

    device.poll(wgpu::Maintain::Wait);
    block_on(output_future).unwrap();

    let output = output_slice.get_mapped_range();
    let result = bytemuck::from_bytes::<<T as AsStd140>::Output>(&output).clone();

    drop(output);
    output_cpu_buffer.unmap();

    T::from_std140(result)
}

fn setup() -> (wgpu::Device, wgpu::Queue) {
    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let adapter =
        block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).unwrap();

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

fn compile(glsl_source: &str) -> anyhow::Result<String> {
    let mut parser = naga::front::glsl::Parser::default();

    let module = parser
        .parse(
            &naga::front::glsl::Options {
                stage: naga::ShaderStage::Compute,
                defines: Default::default(),
            },
            glsl_source,
        )
        .map_err(|err| anyhow::format_err!("{:?}", err))?;

    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::default(),
        naga::valid::Capabilities::all(),
    )
    .validate(&module)?;

    let wgsl = naga::back::wgsl::write_string(&module, &info)?;

    Ok(wgsl)
}
