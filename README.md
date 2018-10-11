# vkr

An unsafe rust wrapper for the [Vulkan API](https://www.khronos.org/registry/vulkan/).

The wrapper provides:
* Function pointer loaders for the Vulkan core API and all extensions
* Type safety for enums and handles
* Defaults and builders for Vulkan structures to make constructing them less verbose
* Thin wrappers around Vulkan functions to make them more convenient to call from rust code

The wrapper does not provide:
* Parameter validation
* Safe functions

Almost all of the library is generated from the Vulkan API specifications using [vk_parse](https://github.com/krolli/vk-parse) to parse the specifications XML.

## Loaders

The structs `Loader`, `Instance` and `Device` load function pointers for the core API.

```rust
// load the Vulkan shared library
let loader = Loader::new()?;

// create an instance (skip listing layers and extensions for this example)
let instance_create_info = vk::InstanceCreateInfo {
    .. Default::default()
};
let instance = unsafe { loader.create_instance(&instance_create_info, None) }?;

// check what version loaded successfully
println!("instance version: {}", instance.version);
```

Each struct will attempt to load function pointers for all versions of Vulkan.
The `version` field can be read to determine what versions loaded successfully.
Function pointers for versions beyond this will be present but their implementation will `panic!`.

Each Vulkan extension has its own loader that must be created manually for an `Instance` or `Device`.  For example:

```rust
// load functions for the VK_NVX_raytracing extension for this device
// (expects instance to have been created with this extension listed)
let nvx_raytracing = NvxRaytracing::new(&instance, &device)?;

// can now call functions from this extension
let accel = nvx_raytracing.create_acceleration_structure_nvx(&create_info, None)?;
```

## Vulkan Handles

Handle types make use of the `std::ptr::NonNull` and `std::num::NonZeroU64` rust types and must always be valid.

When used as part of other structures, handles will be wrapped in `Option<T>` to allow encoding of VK_NULL_HANDLE. For example:

```rust
pub struct DescriptorImageInfo {
    pub sampler: Option<Sampler>,
    pub image_view: Option<ImageView>,
    pub image_layout: ImageLayout,
}
```

When used as function parameters, the parameter will only be wrapped in `Option<T>` if that parameter is optional.  For example:

```rust
impl KhrSwapchain {
    pub unsafe fn acquire_next_image_khr(
        &self,
        swapchain: vk::SwapchainKHR,        // not optional
        timeout: u64,
        semaphore: Option<vk::Semaphore>,   // optional
        fence: Option<vk::Fence>,           // optional
    ) -> Result<(vk::Result, u32)> {
        ...
    }
}
```

## Function Wrappers

Vulkan functions that return `VkResult`, are usually translated to return `Result<T, vk::Result>` for some `T` return value type.
Where there are multiple success codes, the return type will be `Result<(vk::Result, T), vk::Result>` so that the success code is also returned.

The remaining parameters are translated as follows:

* `Device` or `Instance` handles are passed automatically from the loader where possible
* `VkBool32` becomes `bool`
* Pointer to constant null-terminated `char` data become `&CStr`
* Pointers becomes references (wrapped in `Option` when optional)
* Pointer and length pairs become slices
* Functions that fill an array of unknown size have a `_to_vec` variant to return all values in a `Vec`
* Functions that fill an array of known size have `_array` and `_single` variants that do not allocate from the heap, in addition to a `_to_vec` variant

As an example, for this C function:

```C
VkResult vkAllocateMemory(
    VkDevice device,
    const VkMemoryAllocateInfo* pAllocateInfo,
    const VkAllocationCallbacks* pAllocator,
    VkDeviceMemory* pMemory);
```

The rust wrapper on `Device` is:

```rust
impl Device {
    pub unsafe fn allocate_memory(
        &self,
        p_allocate_info: &vk::MemoryAllocateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::DeviceMemory> {
        ...
    }
}
```

## Default and Builders

All structs implement the `Default` trait to avoid having to specify optional members or members that must take a specific value:

```rust
let vertex_input_state_create_info = vk::PipelineVertexInputStateCreateInfo {
    ..Default::default()
};
```

In addition, most structs also implement the `vkr::Builder` trait to create a builder object, which allows fields to be safely set using slices or `CStr`.

The builder struct implements the `Deref` trait to allow a reference to the builder to be used where a reference to the underlying struct is expected.

```rust
let render_pass_create_info = vk::RenderPassCreateInfo::builder()
    .set_p_attachments(&attachments)
    .set_p_subpasses(&subpass_descriptions)
    .set_p_dependencies(&subpass_dependencies);
let render_pass = unsafe { device.create_render_pass(&render_pass_create_info, None) }?;
```
