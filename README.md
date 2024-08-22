<div align="center">

## 🦀 Rust Graphics
Using rust to create a graphical shader rendering pipeline

</div>

##  Goal & purpose

The goal with this project was for me to get some hands on experience interfacing with the GPU through the Rust programming language. It uses rust wgsl and opengl libraries to interface with the GPU. The application sets up a window and renders a triangle with some RGB mixed colors.

## Implementations

One limitation at this moment is that the render pipeline will asume a Vulkan based backend

```rust
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        dx12_shader_compiler: Default::default(),
```

This could be improved by adding some gpu backend discovery logic and asuming a default instead. 

We also need to load a renderer, and here it asumes a "shader.wgsl" to be found in the project and tries to load it, this aswell could be done more dynamically.

```rust
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
```

The shader.wgsl simply imposes a "triangle" shape with  a set of 2 and 3 dimensinal vectors.
The 2 dimensional vectors representing x, y coordinates and where each "corner" of the triangle will start and stop. The 3 dimensional vectors represent the colors to be rendered at each position with RGB.

```wgsl
    var pos = array<vec2<f32>,3>(
        vec2<f32>(0.0, 0.5),
        vec2<f32>(-0.5,-0.5),
        vec2<f32>(0.5,-0.5)
    );
    var color = array<vec3<f32>,3>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0)
    );
```


## Usage

Asumes you have rust and cargo installed and configured

```bash
cargo run 
```

