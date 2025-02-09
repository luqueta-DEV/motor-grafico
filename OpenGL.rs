extern crate wgpu;

use wgpu::{Device, Instance, Queue, Adapter, Surface};
use std::sync::Arc;

pub struct GraphicsDevice {
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
    pub adapter: Arc<Adapter>,
    pub surface: Arc<Surface>,
}

impl GraphicsDevice {
    pub fn new() -> Self {
        
        let instance = wgpu::Instance::new(wgpu::Backends::Vulkan);
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
        }).unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ).unwrap();

        
        let surface = Arc::new(wgpu::Surface::create(&instance, None)); 

        GraphicsDevice {
            device: Arc::new(device),
            queue: Arc::new(queue),
            adapter: Arc::new(adapter),
            surface,
        }
    }
}


