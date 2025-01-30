# spark

This library aims to expose [Vulkan](https://www.khronos.org/registry/vulkan/) in [Rust](https://www.rust-lang.org/) with convenient syntax.

Supports Vulkan 1.4.306 and all extensions (apart from `GGP`/`QNX` extensions that use unknown data types).  It should compile on Windows, Linux and Android.

## Design

The library is similar in scope to [`ash`](https://github.com/MaikKlein/ash).  Ash seems to be the most popular low-library for Vulkan in Rust, so if you are looking for something with wide support, then I recommend using [`ash`](https://github.com/MaikKlein/ash) instead.

Since `ash` widely used, I'll just list the ways this library currently differs from `ash`.  These are just alternatives I personally found interesting to explore:

### Helpers To Manage Extensions

The structs `InstanceExtensions` and `DeviceExtensions` can be used to simplify code that manages extensions.
These expose a method per extension that checks for support, which handles checking for dependencies and the core Vulkan version:

```rust
// parse physical device extension properties into DeviceExtensions
let available_extensions = {
    let extension_properties =
        unsafe { instance.enumerate_device_extension_properties_to_vec(physical_device, None) }.unwrap();
    DeviceExtensions::from_properties(core_version, &extension_properties)
};

// check availability
if available_extensions.supports_khr_draw_indirect_count() {
    println!("VK_KHR_draw_indirect_count is present OR core_version >= 1.2");
}
```

These structs also expose a method per extension to enable that extension and all of its dependencies.  Extensions that have been promoted to the given core Vulkan version are skipped.

```rust
let mut extensions = DeviceExtensions::new(core_version);

// enables VK_KHR_draw_indirect_count (unless core_version >= 1.2)
extensions.enable_khr_draw_indirect_count();

// enables all the following:
//  VK_KHR_acceleration_structure
//  VK_EXT_descriptor_indexing (unless core_version >= 1.2)
//  VK_KHR_maintenance3 (unless core_version >= 1.1)
//  VK_EXT_buffer_device_address (unless core_version >= 1.2)
//  VK_KHR_deferred_host_operations
extensions.enable_khr_acceleration_structure();
```

Once all extensions have been enabled, the set can be passed to `Instance` or `Device` creation as a list of names:

```rust
// for passing to device creation
let extension_names = extensions.to_name_vec();
```

### Extensions Are Part Of `Instance` And `Device`

To simplify the use of extensions and core Vulkan versions, the library manages all Vulkan function pointers directly on the `Instance` or `Device`.

When the `Instance` or `Device` is created, the core version and list of extensions is checked, and all Vulkan commands are loaded that are enabled for that combination.  This handles complex cases such as commands that are loaded only when a combination of extensions are present.  The `extensions` struct on `Instance` or `Device` can be used to query which extensions are supported.

```rust
// emit a marker if EXT_debug_utils was loaded
if instance.extensions.supports_ext_debug_utils() {
    let label = vk::DebugUtilsLabelEXT {
        p_label_name: name.as_ptr(),
        ..Default::default()
    };
    instance.cmd_begin_debug_utils_label_ext(cmd, &label);
}
```

### Vulkan Command Aliases Are Eqivalent

Only one function pointer is stored for Vulkan commands that are aliases of each other.  Once loaded, any alias can be used to emit the command, since they all call through to the same function pointer on `Device` or `Instance`.

For example, when the `Device` is created, `vkCmdDrawIndirectCount` is loaded for one of the following cases:
* If the core version is 1.2 or greater (loaded as `vkCmdDrawIndirectCount`)
* If the `VK_KHR_draw_indirect_count` extension is enabled (loaded as the alias `vkCmdDrawIndirectCountKHR`)
* If the `VK_AMD_draw_indirect_count` extension is enabled (loaded as the alias `vkCmdDrawIndirectCountAMD`)

The resulting function pointer is stored as `Device.fp_cmd_draw_indirect_count` regardless of how it was loaded, so client code can use any wrapper function to emit it:

```rust
// all the following are equivalent, they all call through to device.fp_cmd_draw_indirect_count
device.cmd_draw_indirect_count(/*...*/)
device.cmd_draw_indirect_count_khr(/*...*/)
device.cmd_draw_indirect_count_amd(/*...*/)
```

### Non-Zero Handles

This is opinionated, but the library enforces that Vulkan handles must be non-null, by making use of the `NonZeroUsize` and `NonZeroU64` types.  For optional function parameters or struct members, they can be wrapped in `Option` to represent `VK_NULL_HANDLE` directly as `None`.

The parameter type then encodes whether that object is required:

```rust
impl Device {
    /* ... */
    pub unsafe fn acquire_next_image_khr(
        &self,
        swapchain: vk::SwapchainKHR,        // not optional
        timeout: u64,
        semaphore: Option<vk::Semaphore>,   // optional
        fence: Option<vk::Fence>,           // optional
    ) -> Result<(vk::Result, u32)> {
        /* ... */
    }
    /* ... */
}
```

But struct declarations always use `Option` (to be able to have a `Default`), so get a bit more noisy:

```rust
pub struct DescriptorImageInfo {
    pub sampler: Option<Sampler>,
    pub image_view: Option<ImageView>,
    pub image_layout: ImageLayout,
}
```

On balance I think this is worth it and more Rust-y for handles to always be valid.

### Fully Generated

I had a go at generating not only the struct and function pointer types as much as possible (hopefully there will be a standard `vk-sys` for this one day), but also **all** the wrappers that exist to make Vulkan functions more Rust-y on `Instance` and `Device` (and all the struct builders too).

These are generated using [vk_parse](https://github.com/krolli/vk-parse) to parse the Vulkan specifications XML, then taking care to use info in the spec as much as possible, such as:
* All the sensible translations to C from `bool`, rust native types, `CStr`, `Option`, references and slices
* Pair up arrays with lengths (including cases where multiple arrays share a single length)
* Which result codes are considered to be successful for that call

This seems to handle tricky cases reasonable well, like functions that have multiple "success" codes:

```rust
impl Device {
    /* ... */
    pub unsafe fn wait_semaphores_khr(
        &self,
        p_wait_info: &vk::SemaphoreWaitInfoKHR,
        timeout: u64,
    ) -> Result<vk::Result> {
        /*
            returns Ok(SUCCESS), Ok(TIMEOUT) or Err(other)
        */
    }
    /* ... */
}
```

Or functions (in this case a builder) where two arrays must be the same length (so are built together):

```rust
impl<'a> SubmitInfoBuilder<'a> {
    /* ... */
    pub fn p_wait_semaphores(
        mut self,
        p_wait_semaphores: &'a [vk::Semaphore],
        p_wait_dst_stage_mask: &'a [vk::PipelineStageFlags],
    ) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        assert_eq!(self.inner.wait_semaphore_count, p_wait_dst_stage_mask.len() as u32);
        self.inner.p_wait_semaphores = p_wait_semaphores.as_ptr();
        self.inner.p_wait_dst_stage_mask = p_wait_dst_stage_mask.as_ptr();
        self
    }
    /* ... */
}
```

### Zero-Allocation Where Possible

This is maybe overkill, but functions that fill an array of known size have `_array` and `_single` variants that do not allocate from the heap, in addition to a `_to_vec` variant that requires a heap allocation.

```rust
impl Device {
    /* ... */
    pub unsafe fn create_compute_pipelines_to_vec(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<Vec<vk::Pipeline>> {
        /* ... */
    }
    pub unsafe fn create_compute_pipelines_array<const N: usize>(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<[vk::Pipeline; N]> {
        /* ... */
    }
    pub unsafe fn create_compute_pipelines_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &vk::ComputePipelineCreateInfo,
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        /* ... */
    }
    /* ... */
}
```

The `_array` version now makes use of const generics to be fully generic over array length.

## Examples

Examples can be found in the `examples` folder.

### [`compute`](https://github.com/sjb3d/spark/blob/master/examples/compute)

A minimal console application that runs a compute shader to fill some memory.  Shows basic usage of a Vulkan device.

### [`graphics`](https://github.com/sjb3d/spark/blob/master/examples/graphics)

A minimal windowed application that draws a spinning triangle and some UI.
* Uses [`winit`](https://github.com/rust-windowing/winit) for windowing
* Demonstrates [`spark-egui`](https://github.com/sjb3d/spark/tree/master/spark-egui) as a renderer for [`egui`](https://github.com/emilk/egui).

![graphics](https://raw.githubusercontent.com/sjb3d/spark/master/docs/graphics.png)
