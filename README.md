<div align="center">

## 🦀 Rust Graphics
Using rust to create a graphical shader rendering pipeline

![Graphics](graphics.png)

</div>

## 🛠 Content

shader.wgsl : This WGSL shader defines a simple triangle rendering process. It takes vertex indices as input and assigns positions and colors to each vertex of a triangle. 

main.rs : Sets up our graphical application using winit and wgpu libraries. It creates a window and configures a Vulkan based graphics instance and establishes a rendering pipeline with our shader.

## 🚀 Usage

Asumes you have rust and cargo installed and configured

```bash
cargo run 
```