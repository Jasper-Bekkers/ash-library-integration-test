use ash::{vk, Entry, Instance, Device};
use ash::version::{EntryV1_0, EntryV1_1, InstanceV1_0, DeviceV1_0};
use ash::vk_make_version;
use std::ffi::CString;
use ash::vk::Handle;
use vk_mem::*;


fn app_init(entry: &ash::Entry) -> (Instance, vk::PhysicalDevice, Device) {
    let app_name = CString::new("Test").unwrap();

    let appinfo = vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(0)
        .engine_name(&app_name)
        .engine_version(0)
        .api_version(vk_make_version!(1, 1, 0));


    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&appinfo)
        .enabled_layer_names(&[])
        .enabled_extension_names(&[]);

    let app_instance: Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Instance creation error")
    };
    
    let pdevices = unsafe {
        app_instance
            .enumerate_physical_devices()
            .expect("Physical device error")
    };

    let priorities = [1.0];
    let queue_family_index = 0;

    let queue_info = [vk::DeviceQueueCreateInfo::builder()
        .queue_family_index(queue_family_index)
        .queue_priorities(&priorities)
        .build()];

    let device_create_info = vk::DeviceCreateInfo {
        p_queue_create_infos: queue_info.as_ptr(),
        queue_create_info_count: queue_info.len() as u32,
        ..Default::default()
    };

    let app_device = unsafe {
        app_instance.create_device(pdevices[0], &device_create_info, None).unwrap()
    };

    (app_instance, pdevices[0], app_device)
}

fn lib_init_2(raw_instance: u64, raw_pdevice: u64, raw_device: u64) {
    let entry = Entry::new().unwrap();
    let lib_instance = unsafe {
        entry.create_instance_from_raw_handle(raw_instance).unwrap()
    };

    let lib_device = unsafe {
        lib_instance.create_device_from_raw_handle(raw_device).unwrap()
    };

    let create_info = vk::CommandPoolCreateInfo::builder().queue_family_index(0).build();

    let pool = unsafe {
        lib_device.create_command_pool(
            &create_info,
            None
        ).unwrap()
    };

    dbg!(pool);

    let cmd_create_info = vk::CommandBufferAllocateInfo::builder()
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_pool(pool)
        .command_buffer_count(1)
        .build();

    let cmd = unsafe {
        lib_device.allocate_command_buffers(&cmd_create_info)
    };

    dbg!(lib_device.handle());
    dbg!(cmd);

    let alloc_create_info = AllocatorCreateInfo {
        physical_device_raw: raw_pdevice,
        device_raw: raw_device,
        instance_raw: raw_instance,
        flags: AllocatorCreateFlags::NONE,
        preferred_large_heap_block_size: 0,
        frame_in_use_count: 1,
        heap_size_limits: None,
    };

    let mut allocator = Allocator::new(&alloc_create_info).unwrap();

    let allocation_info = vk_mem::AllocationCreateInfo {
        required_flags: ash::vk::MemoryPropertyFlags::HOST_VISIBLE.as_raw(),
        preferred_flags: (ash::vk::MemoryPropertyFlags::HOST_COHERENT
            | ash::vk::MemoryPropertyFlags::HOST_CACHED).as_raw(),
        flags: vk_mem::AllocationCreateFlags::MAPPED,
        ..Default::default()
    };

    let buffer_create = vk::BufferCreateInfo::builder()
        .size(16 * 1024)
        .usage(
            ash::vk::BufferUsageFlags::VERTEX_BUFFER
                | ash::vk::BufferUsageFlags::TRANSFER_DST,
        )
        .build();

    let (buffer, allocation, allocation_info) = allocator
        .create_buffer(
            &unsafe { std::mem::transmute(buffer_create) },
            &allocation_info,
        )
        .unwrap();
    assert_ne!(allocation_info.get_mapped_data(), std::ptr::null_mut());
    dbg!(allocation_info.get_mapped_data());
    allocator.destroy_buffer(buffer, &allocation).unwrap();

}

fn main() {
    let entry = Entry::new().unwrap();
    let (app_instance, app_pdevice, app_device) = app_init(&entry);

    lib_init_2(app_instance.handle().as_raw(), app_pdevice.as_raw(), app_device.handle().as_raw());
}
