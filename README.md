# vkr

This library aims to expose [Vulkan](https://www.khronos.org/registry/vulkan/) in [Rust](https://www.rust-lang.org/) with convenient syntax.

Supports Vulkan 1.2.133 and all extensions (apart from `GGP` extensions that use unknown data types).

## Design

It ended up very similar in design and scope to [`ash`](https://github.com/MaikKlein/ash).  Ash seems to be the most popular low-library for Vulkan in Rust, so if you are looking for something with wide support, then I recommend using [`ash`](https://github.com/MaikKlein/ash) instead.

Since `ash` widely used, I'll just list the ways this library currently differs from `ash`.  These are just alternatives I personally found interesting to explore:

### Extensions Are (Optional) Parts Of `Instance` And `Device`

When you create an `Instance` or `Device`, the library checks the Vulkan version and array of extension names, and loads all the function pointers that are referenced by that combination.  The `Instance` or `Device` object have an `extensions` member variable that can be inspected to check which extensions were loaded, and all Vulkan functions are accessible from either the `Instance` or `Device`.

```rust
// emit marker if we have EXT_debug_utils loaded
if instance.extensions.ext_debug_utils {
    let label = vk::DebugUtilsLabelEXT {
        p_label_name: name.as_ptr(),
        ..Default::default()
    };
    instance.cmd_begin_debug_utils_label_ext(cmd, &label);
}
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
    pub unsafe fn create_compute_pipelines_single(
        &self,
        pipeline_cache: Option<vk::PipelineCache>,
        p_create_infos: &[vk::ComputePipelineCreateInfo],
        p_allocator: Option<&vk::AllocationCallbacks>,
    ) -> Result<vk::Pipeline> {
        /* ... */
    }
    /* ... */
}
```

The `_array` version is implemented using a trait up to arrays of length 8, but should be possible to make fully generic once *const generics* are part of stable Rust.

## Examples

Examples can be found in the `examples` folder.

### [`compute`](https://github.com/sjb3d/vkr/blob/master/examples/src/bin/compute.rs)

A minimal console application that runs a compute shader to fill some memory.  Shows basic usage of a Vulkan device.

### `imgui` (soon)

A renderer implementation for [`imgui-rs`](https://github.com/Gekkio/imgui-rs) (which wraps the amazing [Dear ImGui](https://github.com/ocornut/imgui)) can be found in [`vkr-imgui`](https://github.com/sjb3d/vkr/tree/master/vkr-imgui) now.  An example that demonstrates this will be added soon.
