<div align="center">

## 🦀 Rust Graphics
Using rust to create a graphical shader rendering pipeline

<img src="graphics.png" alt="graphics" width="200"/>

</div>

## 📬 Goal and purposes

The goal with this project was for me to get some hands on experience interfacing with the GPU through the Rust programming language. It uses rust wgsl and opengl libraries to interface with the GPU. The application sets up a window and renders a triangle with some RGB mixed colors.

## 🛠 Content

**src/main.rs** : 

Application entry point that hosts the event loop. 

**src/shader.wgsl** :

WGSL shader that defines a simple triangle rendering process. It takes vertex indices as input and assigns positions and colors to each vertex of a triangle.

**src/renderpipeline.rs** : 
	
*Sets up our graphical application using winit and wgpu libraries. It takes a window and configures a Vulkan based graphics instance and establishes a rendering pipeline with our shader.*


## 🚀 Usage

Asumes you have rust and cargo installed and configured

```bash
cargo run 
```

