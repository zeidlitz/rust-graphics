mod renderpipeline;
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap(); 
    window.set_title("Rust Graphics");
    env_logger::init();
    pollster::block_on( renderpipeline::render(event_loop, &window));    
}