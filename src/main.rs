use std::sync::Arc;

use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    device::{Device, DeviceCreateInfo, Queue, QueueCreateInfo},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::StandardMemoryAllocator,
    VulkanLibrary,
};

use bytemuck::{Pod, Zeroable};

struct VkContext {
    device: Arc<Device>,
    queue: Arc<Queue>,
    memory_allocator: StandardMemoryAllocator,
}

impl VkContext {
    fn new() -> VkContext {

        log::info!("Creating Vulkan context");

        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let instance = Instance::new(library, InstanceCreateInfo::default())
            .expect("failed to create instance");

        let physical = instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");

        let queue_family_index = physical
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_, q)| q.queue_flags.graphics)
            .expect("couldn't find a graphical queue family")
            as u32;

        let (device, mut queues) = Device::new(
            physical,
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        let memory_allocator = StandardMemoryAllocator::new_default(device.clone());

        VkContext {
            device,
            queue: queues.next().unwrap(),
            memory_allocator,
        }
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
struct Particle {
    position: [f32; 2],
    color: [f32; 3],
}

impl Particle {
    fn new() -> Particle {
        Particle {
            position: [0.0, 0.0],
            color: [1.0, 1.0, 1.0],
        }
    }
}

struct World {
    buffer: Arc<CpuAccessibleBuffer<[Particle]>>,
}

impl World {
    fn new_random(ctx: &VkContext) -> World {
        let iter = (0..128).map(|_| Particle::new());

        let buffer = CpuAccessibleBuffer::from_iter(
            &ctx.memory_allocator,
            BufferUsage {
                uniform_buffer: true,
                ..Default::default()
            },
            false,
            iter,
        )
        .unwrap();

        World { buffer }
    }

    fn dump_data(&self, path: &str) {

    }
}

fn main() {
    env_logger::init();
    let ctx = VkContext::new();
    let world = World::new_random(&ctx);
}
