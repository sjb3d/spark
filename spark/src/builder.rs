//! Generated from vk.xml version 1.4.318

#![allow(clippy::wrong_self_convention, clippy::unnecessary_cast)]

use super::vk;
use std::{
    ffi::CStr,
    marker::PhantomData,
    ops::Deref,
    os::raw::{c_char, c_void},
    ptr,
};

pub trait Builder<'a> {
    type Type;
    fn builder() -> Self::Type;
}

unsafe fn insert_next(head: *mut vk::BaseOutStructure, other: *mut vk::BaseOutStructure) {
    assert!((*other).p_next.is_null());
    (*other).p_next = (*head).p_next;
    (*head).p_next = other as *mut _;
}

#[repr(transparent)]
#[derive(Default)]
pub struct ApplicationInfoBuilder<'a> {
    inner: vk::ApplicationInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ApplicationInfo {
    type Type = ApplicationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ApplicationInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_application_name(mut self, p_application_name: Option<&'a CStr>) -> Self {
        self.inner.p_application_name = p_application_name.map_or(ptr::null(), |r| r.as_ptr());
        self
    }
    pub fn application_version(mut self, application_version: u32) -> Self {
        self.inner.application_version = application_version;
        self
    }
    pub fn p_engine_name(mut self, p_engine_name: Option<&'a CStr>) -> Self {
        self.inner.p_engine_name = p_engine_name.map_or(ptr::null(), |r| r.as_ptr());
        self
    }
    pub fn engine_version(mut self, engine_version: u32) -> Self {
        self.inner.engine_version = engine_version;
        self
    }
    pub fn api_version(mut self, api_version: vk::Version) -> Self {
        self.inner.api_version = api_version;
        self
    }
}
impl<'a> Deref for ApplicationInfoBuilder<'a> {
    type Target = vk::ApplicationInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct AllocationCallbacksBuilder<'a> {
    inner: vk::AllocationCallbacks,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AllocationCallbacks {
    type Type = AllocationCallbacksBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AllocationCallbacksBuilder<'a> {
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
    pub fn pfn_allocation(mut self, pfn_allocation: vk::FnAllocationFunction) -> Self {
        self.inner.pfn_allocation = Some(pfn_allocation);
        self
    }
    pub fn pfn_reallocation(mut self, pfn_reallocation: vk::FnReallocationFunction) -> Self {
        self.inner.pfn_reallocation = Some(pfn_reallocation);
        self
    }
    pub fn pfn_free(mut self, pfn_free: vk::FnFreeFunction) -> Self {
        self.inner.pfn_free = Some(pfn_free);
        self
    }
    pub fn pfn_internal_allocation(
        mut self,
        pfn_internal_allocation: Option<vk::FnInternalAllocationNotification>,
    ) -> Self {
        self.inner.pfn_internal_allocation = pfn_internal_allocation;
        self
    }
    pub fn pfn_internal_free(mut self, pfn_internal_free: Option<vk::FnInternalFreeNotification>) -> Self {
        self.inner.pfn_internal_free = pfn_internal_free;
        self
    }
}
impl<'a> Deref for AllocationCallbacksBuilder<'a> {
    type Target = vk::AllocationCallbacks;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceQueueCreateInfoBuilder<'a> {
    inner: vk::DeviceQueueCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceQueueCreateInfo {
    type Type = DeviceQueueCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DeviceQueueCreateInfoNext {}
impl<'a> DeviceQueueCreateInfoBuilder<'a> {
    pub fn insert_next<T: DeviceQueueCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceQueueCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
    pub fn p_queue_priorities(mut self, p_queue_priorities: &'a [f32]) -> Self {
        self.inner.queue_count = p_queue_priorities.len() as u32;
        self.inner.p_queue_priorities = p_queue_priorities.as_ptr();
        self
    }
}
impl<'a> Deref for DeviceQueueCreateInfoBuilder<'a> {
    type Target = vk::DeviceQueueCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceCreateInfoBuilder<'a> {
    inner: vk::DeviceCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceCreateInfo {
    type Type = DeviceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DeviceCreateInfoNext {}
impl<'a> DeviceCreateInfoBuilder<'a> {
    pub fn insert_next<T: DeviceCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_queue_create_infos(mut self, p_queue_create_infos: &'a [vk::DeviceQueueCreateInfo]) -> Self {
        self.inner.queue_create_info_count = p_queue_create_infos.len() as u32;
        self.inner.p_queue_create_infos = p_queue_create_infos.as_ptr();
        self
    }
    pub fn pp_enabled_layer_names(mut self, pp_enabled_layer_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_layer_count = pp_enabled_layer_names.len() as u32;
        self.inner.pp_enabled_layer_names = pp_enabled_layer_names.as_ptr();
        self
    }
    pub fn pp_enabled_extension_names(mut self, pp_enabled_extension_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_extension_count = pp_enabled_extension_names.len() as u32;
        self.inner.pp_enabled_extension_names = pp_enabled_extension_names.as_ptr();
        self
    }
    pub fn p_enabled_features(mut self, p_enabled_features: Option<&'a vk::PhysicalDeviceFeatures>) -> Self {
        self.inner.p_enabled_features = p_enabled_features.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for DeviceCreateInfoBuilder<'a> {
    type Target = vk::DeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct InstanceCreateInfoBuilder<'a> {
    inner: vk::InstanceCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::InstanceCreateInfo {
    type Type = InstanceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait InstanceCreateInfoNext {}
impl<'a> InstanceCreateInfoBuilder<'a> {
    pub fn insert_next<T: InstanceCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::InstanceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_application_info(mut self, p_application_info: Option<&'a vk::ApplicationInfo>) -> Self {
        self.inner.p_application_info = p_application_info.map_or(ptr::null(), |r| r);
        self
    }
    pub fn pp_enabled_layer_names(mut self, pp_enabled_layer_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_layer_count = pp_enabled_layer_names.len() as u32;
        self.inner.pp_enabled_layer_names = pp_enabled_layer_names.as_ptr();
        self
    }
    pub fn pp_enabled_extension_names(mut self, pp_enabled_extension_names: &'a [*const c_char]) -> Self {
        self.inner.enabled_extension_count = pp_enabled_extension_names.len() as u32;
        self.inner.pp_enabled_extension_names = pp_enabled_extension_names.as_ptr();
        self
    }
}
impl<'a> Deref for InstanceCreateInfoBuilder<'a> {
    type Target = vk::InstanceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct MemoryAllocateInfoBuilder<'a> {
    inner: vk::MemoryAllocateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MemoryAllocateInfo {
    type Type = MemoryAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait MemoryAllocateInfoNext {}
impl<'a> MemoryAllocateInfoBuilder<'a> {
    pub fn insert_next<T: MemoryAllocateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for MemoryAllocateInfoBuilder<'a> {
    type Target = vk::MemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct WriteDescriptorSetBuilder<'a> {
    inner: vk::WriteDescriptorSet,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::WriteDescriptorSet {
    type Type = WriteDescriptorSetBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait WriteDescriptorSetNext {}
impl<'a> WriteDescriptorSetBuilder<'a> {
    pub fn insert_next<T: WriteDescriptorSetNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dst_set(mut self, dst_set: vk::DescriptorSet) -> Self {
        self.inner.dst_set = dst_set;
        self
    }
    pub fn dst_binding(mut self, dst_binding: u32) -> Self {
        self.inner.dst_binding = dst_binding;
        self
    }
    pub fn dst_array_element(mut self, dst_array_element: u32) -> Self {
        self.inner.dst_array_element = dst_array_element;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
    pub fn p_image_info(mut self, p_image_info: &'a [vk::DescriptorImageInfo]) -> Self {
        self.inner.descriptor_count = p_image_info.len() as u32;
        self.inner.p_image_info = p_image_info.as_ptr();
        self
    }
    pub fn p_buffer_info(mut self, p_buffer_info: &'a [vk::DescriptorBufferInfo]) -> Self {
        self.inner.descriptor_count = p_buffer_info.len() as u32;
        self.inner.p_buffer_info = p_buffer_info.as_ptr();
        self
    }
    pub fn p_texel_buffer_view(mut self, p_texel_buffer_view: &'a [vk::BufferView]) -> Self {
        self.inner.descriptor_count = p_texel_buffer_view.len() as u32;
        self.inner.p_texel_buffer_view = p_texel_buffer_view.as_ptr();
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
}
impl<'a> Deref for WriteDescriptorSetBuilder<'a> {
    type Target = vk::WriteDescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BufferViewCreateInfoNext for vk::BufferUsageFlags2CreateInfo {}
impl BufferCreateInfoNext for vk::BufferUsageFlags2CreateInfo {}
impl PhysicalDeviceExternalBufferInfoNext for vk::BufferUsageFlags2CreateInfo {}
impl DescriptorBufferBindingInfoEXTNext for vk::BufferUsageFlags2CreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct BufferCreateInfoBuilder<'a> {
    inner: vk::BufferCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BufferCreateInfo {
    type Type = BufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BufferCreateInfoNext {}
impl<'a> BufferCreateInfoBuilder<'a> {
    pub fn insert_next<T: BufferCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::BufferCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
    pub fn usage(mut self, usage: vk::BufferUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn queue_family_index_count(mut self, queue_family_index_count: u32) -> Self {
        self.inner.queue_family_index_count = queue_family_index_count;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
}
impl<'a> Deref for BufferCreateInfoBuilder<'a> {
    type Target = vk::BufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BufferViewCreateInfoBuilder<'a> {
    inner: vk::BufferViewCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BufferViewCreateInfo {
    type Type = BufferViewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BufferViewCreateInfoNext {}
impl<'a> BufferViewCreateInfoBuilder<'a> {
    pub fn insert_next<T: BufferViewCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for BufferViewCreateInfoBuilder<'a> {
    type Target = vk::BufferViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BufferMemoryBarrierBuilder<'a> {
    inner: vk::BufferMemoryBarrier,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BufferMemoryBarrier {
    type Type = BufferMemoryBarrierBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BufferMemoryBarrierNext {}
impl<'a> BufferMemoryBarrierBuilder<'a> {
    pub fn insert_next<T: BufferMemoryBarrierNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for BufferMemoryBarrierBuilder<'a> {
    type Target = vk::BufferMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageMemoryBarrierBuilder<'a> {
    inner: vk::ImageMemoryBarrier,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageMemoryBarrier {
    type Type = ImageMemoryBarrierBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageMemoryBarrierNext {}
impl<'a> ImageMemoryBarrierBuilder<'a> {
    pub fn insert_next<T: ImageMemoryBarrierNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for ImageMemoryBarrierBuilder<'a> {
    type Target = vk::ImageMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageCreateInfoBuilder<'a> {
    inner: vk::ImageCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageCreateInfo {
    type Type = ImageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageCreateInfoNext {}
impl<'a> ImageCreateInfoBuilder<'a> {
    pub fn insert_next<T: ImageCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image_type(mut self, image_type: vk::ImageType) -> Self {
        self.inner.image_type = image_type;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn extent(mut self, extent: vk::Extent3D) -> Self {
        self.inner.extent = extent;
        self
    }
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.inner.mip_levels = mip_levels;
        self
    }
    pub fn array_layers(mut self, array_layers: u32) -> Self {
        self.inner.array_layers = array_layers;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn queue_family_index_count(mut self, queue_family_index_count: u32) -> Self {
        self.inner.queue_family_index_count = queue_family_index_count;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
    pub fn initial_layout(mut self, initial_layout: vk::ImageLayout) -> Self {
        self.inner.initial_layout = initial_layout;
        self
    }
}
impl<'a> Deref for ImageCreateInfoBuilder<'a> {
    type Target = vk::ImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageViewCreateInfoBuilder<'a> {
    inner: vk::ImageViewCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageViewCreateInfo {
    type Type = ImageViewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageViewCreateInfoNext {}
impl<'a> ImageViewCreateInfoBuilder<'a> {
    pub fn insert_next<T: ImageViewCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for ImageViewCreateInfoBuilder<'a> {
    type Target = vk::ImageViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SparseBufferMemoryBindInfoBuilder<'a> {
    inner: vk::SparseBufferMemoryBindInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SparseBufferMemoryBindInfo {
    type Type = SparseBufferMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SparseBufferMemoryBindInfoBuilder<'a> {
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = buffer;
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.as_ptr();
        self
    }
}
impl<'a> Deref for SparseBufferMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseBufferMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    inner: vk::SparseImageOpaqueMemoryBindInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SparseImageOpaqueMemoryBindInfo {
    type Type = SparseImageOpaqueMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = image;
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.as_ptr();
        self
    }
}
impl<'a> Deref for SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseImageOpaqueMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SparseImageMemoryBindInfoBuilder<'a> {
    inner: vk::SparseImageMemoryBindInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SparseImageMemoryBindInfo {
    type Type = SparseImageMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SparseImageMemoryBindInfoBuilder<'a> {
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = image;
        self
    }
    pub fn p_binds(mut self, p_binds: &'a [vk::SparseImageMemoryBind]) -> Self {
        self.inner.bind_count = p_binds.len() as u32;
        self.inner.p_binds = p_binds.as_ptr();
        self
    }
}
impl<'a> Deref for SparseImageMemoryBindInfoBuilder<'a> {
    type Target = vk::SparseImageMemoryBindInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BindSparseInfoBuilder<'a> {
    inner: vk::BindSparseInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindSparseInfo {
    type Type = BindSparseInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BindSparseInfoNext {}
impl<'a> BindSparseInfoBuilder<'a> {
    pub fn insert_next<T: BindSparseInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphores(mut self, p_wait_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        self.inner.p_wait_semaphores = p_wait_semaphores.as_ptr();
        self
    }
    pub fn p_buffer_binds(mut self, p_buffer_binds: &'a [vk::SparseBufferMemoryBindInfo]) -> Self {
        self.inner.buffer_bind_count = p_buffer_binds.len() as u32;
        self.inner.p_buffer_binds = p_buffer_binds.as_ptr();
        self
    }
    pub fn p_image_opaque_binds(mut self, p_image_opaque_binds: &'a [vk::SparseImageOpaqueMemoryBindInfo]) -> Self {
        self.inner.image_opaque_bind_count = p_image_opaque_binds.len() as u32;
        self.inner.p_image_opaque_binds = p_image_opaque_binds.as_ptr();
        self
    }
    pub fn p_image_binds(mut self, p_image_binds: &'a [vk::SparseImageMemoryBindInfo]) -> Self {
        self.inner.image_bind_count = p_image_binds.len() as u32;
        self.inner.p_image_binds = p_image_binds.as_ptr();
        self
    }
    pub fn p_signal_semaphores(mut self, p_signal_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphores.len() as u32;
        self.inner.p_signal_semaphores = p_signal_semaphores.as_ptr();
        self
    }
}
impl<'a> Deref for BindSparseInfoBuilder<'a> {
    type Target = vk::BindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ShaderModuleCreateInfoBuilder<'a> {
    inner: vk::ShaderModuleCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ShaderModuleCreateInfo {
    type Type = ShaderModuleCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ShaderModuleCreateInfoNext {}
impl<'a> ShaderModuleCreateInfoBuilder<'a> {
    pub fn insert_next<T: ShaderModuleCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ShaderModuleCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn code_size(mut self, code_size: usize) -> Self {
        self.inner.code_size = code_size;
        self
    }
    pub fn p_code(mut self, p_code: *const u32) -> Self {
        self.inner.p_code = p_code;
        self
    }
}
impl<'a> Deref for ShaderModuleCreateInfoBuilder<'a> {
    type Target = vk::ShaderModuleCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineShaderStageCreateInfoNext for vk::ShaderModuleCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorSetLayoutBindingBuilder<'a> {
    inner: vk::DescriptorSetLayoutBinding,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutBinding {
    type Type = DescriptorSetLayoutBindingBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DescriptorSetLayoutBindingBuilder<'a> {
    pub fn binding(mut self, binding: u32) -> Self {
        self.inner.binding = binding;
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
    pub fn descriptor_count(mut self, descriptor_count: u32) -> Self {
        self.inner.descriptor_count = descriptor_count;
        self
    }
    pub fn p_immutable_samplers(mut self, p_immutable_samplers: &'a [vk::Sampler]) -> Self {
        self.inner.descriptor_count = p_immutable_samplers.len() as u32;
        self.inner.p_immutable_samplers = p_immutable_samplers.as_ptr();
        self
    }
    pub fn stage_flags(mut self, stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.stage_flags = stage_flags;
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutBindingBuilder<'a> {
    type Target = vk::DescriptorSetLayoutBinding;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorSetLayoutCreateInfoBuilder<'a> {
    inner: vk::DescriptorSetLayoutCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutCreateInfo {
    type Type = DescriptorSetLayoutCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorSetLayoutCreateInfoNext {}
impl<'a> DescriptorSetLayoutCreateInfoBuilder<'a> {
    pub fn insert_next<T: DescriptorSetLayoutCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorSetLayoutCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_bindings(mut self, p_bindings: &'a [vk::DescriptorSetLayoutBinding]) -> Self {
        self.inner.binding_count = p_bindings.len() as u32;
        self.inner.p_bindings = p_bindings.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutCreateInfoBuilder<'a> {
    type Target = vk::DescriptorSetLayoutCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorPoolCreateInfoBuilder<'a> {
    inner: vk::DescriptorPoolCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorPoolCreateInfo {
    type Type = DescriptorPoolCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorPoolCreateInfoNext {}
impl<'a> DescriptorPoolCreateInfoBuilder<'a> {
    pub fn insert_next<T: DescriptorPoolCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn max_sets(mut self, max_sets: u32) -> Self {
        self.inner.max_sets = max_sets;
        self
    }
    pub fn p_pool_sizes(mut self, p_pool_sizes: &'a [vk::DescriptorPoolSize]) -> Self {
        self.inner.pool_size_count = p_pool_sizes.len() as u32;
        self.inner.p_pool_sizes = p_pool_sizes.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorPoolCreateInfoBuilder<'a> {
    type Target = vk::DescriptorPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorSetAllocateInfoBuilder<'a> {
    inner: vk::DescriptorSetAllocateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorSetAllocateInfo {
    type Type = DescriptorSetAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorSetAllocateInfoNext {}
impl<'a> DescriptorSetAllocateInfoBuilder<'a> {
    pub fn insert_next<T: DescriptorSetAllocateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn descriptor_pool(mut self, descriptor_pool: vk::DescriptorPool) -> Self {
        self.inner.descriptor_pool = descriptor_pool;
        self
    }
    pub fn p_set_layouts(mut self, p_set_layouts: &'a [vk::DescriptorSetLayout]) -> Self {
        self.inner.descriptor_set_count = p_set_layouts.len() as u32;
        self.inner.p_set_layouts = p_set_layouts.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorSetAllocateInfoBuilder<'a> {
    type Target = vk::DescriptorSetAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SpecializationInfoBuilder<'a> {
    inner: vk::SpecializationInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SpecializationInfo {
    type Type = SpecializationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SpecializationInfoBuilder<'a> {
    pub fn p_map_entries(mut self, p_map_entries: &'a [vk::SpecializationMapEntry]) -> Self {
        self.inner.map_entry_count = p_map_entries.len() as u32;
        self.inner.p_map_entries = p_map_entries.as_ptr();
        self
    }
    pub fn data_size(mut self, data_size: usize) -> Self {
        self.inner.data_size = data_size;
        self
    }
    pub fn p_data(mut self, p_data: *const c_void) -> Self {
        self.inner.p_data = p_data;
        self
    }
}
impl<'a> Deref for SpecializationInfoBuilder<'a> {
    type Target = vk::SpecializationInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineShaderStageCreateInfoBuilder<'a> {
    inner: vk::PipelineShaderStageCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineShaderStageCreateInfo {
    type Type = PipelineShaderStageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineShaderStageCreateInfoNext {}
impl<'a> PipelineShaderStageCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineShaderStageCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineShaderStageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage(mut self, stage: vk::ShaderStageFlags) -> Self {
        self.inner.stage = stage;
        self
    }
    pub fn module(mut self, module: vk::ShaderModule) -> Self {
        self.inner.module = module;
        self
    }
    pub fn p_name(mut self, p_name: &'a CStr) -> Self {
        self.inner.p_name = p_name.as_ptr();
        self
    }
    pub fn p_specialization_info(mut self, p_specialization_info: Option<&'a vk::SpecializationInfo>) -> Self {
        self.inner.p_specialization_info = p_specialization_info.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for PipelineShaderStageCreateInfoBuilder<'a> {
    type Target = vk::PipelineShaderStageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ComputePipelineCreateInfoBuilder<'a> {
    inner: vk::ComputePipelineCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ComputePipelineCreateInfo {
    type Type = ComputePipelineCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ComputePipelineCreateInfoNext {}
impl<'a> ComputePipelineCreateInfoBuilder<'a> {
    pub fn insert_next<T: ComputePipelineCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for ComputePipelineCreateInfoBuilder<'a> {
    type Target = vk::ComputePipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ComputePipelineCreateInfoNext for vk::ComputePipelineIndirectBufferInfoNV {}
impl ComputePipelineCreateInfoNext for vk::PipelineCreateFlags2CreateInfo {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineCreateFlags2CreateInfo {}
impl RayTracingPipelineCreateInfoNVNext for vk::PipelineCreateFlags2CreateInfo {}
impl RayTracingPipelineCreateInfoKHRNext for vk::PipelineCreateFlags2CreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineVertexInputStateCreateInfoBuilder<'a> {
    inner: vk::PipelineVertexInputStateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineVertexInputStateCreateInfo {
    type Type = PipelineVertexInputStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineVertexInputStateCreateInfoNext {}
impl<'a> PipelineVertexInputStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineVertexInputStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineVertexInputStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_vertex_binding_descriptions(
        mut self,
        p_vertex_binding_descriptions: &'a [vk::VertexInputBindingDescription],
    ) -> Self {
        self.inner.vertex_binding_description_count = p_vertex_binding_descriptions.len() as u32;
        self.inner.p_vertex_binding_descriptions = p_vertex_binding_descriptions.as_ptr();
        self
    }
    pub fn p_vertex_attribute_descriptions(
        mut self,
        p_vertex_attribute_descriptions: &'a [vk::VertexInputAttributeDescription],
    ) -> Self {
        self.inner.vertex_attribute_description_count = p_vertex_attribute_descriptions.len() as u32;
        self.inner.p_vertex_attribute_descriptions = p_vertex_attribute_descriptions.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineVertexInputStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineVertexInputStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineTessellationStateCreateInfoBuilder<'a> {
    inner: vk::PipelineTessellationStateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineTessellationStateCreateInfo {
    type Type = PipelineTessellationStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineTessellationStateCreateInfoNext {}
impl<'a> PipelineTessellationStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineTessellationStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PipelineTessellationStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineTessellationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineViewportStateCreateInfoBuilder<'a> {
    inner: vk::PipelineViewportStateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineViewportStateCreateInfo {
    type Type = PipelineViewportStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineViewportStateCreateInfoNext {}
impl<'a> PipelineViewportStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineViewportStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineViewportStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.inner.viewport_count = viewport_count;
        self
    }
    pub fn p_viewports(mut self, p_viewports: &'a [vk::Viewport]) -> Self {
        self.inner.viewport_count = p_viewports.len() as u32;
        self.inner.p_viewports = p_viewports.as_ptr();
        self
    }
    pub fn scissor_count(mut self, scissor_count: u32) -> Self {
        self.inner.scissor_count = scissor_count;
        self
    }
    pub fn p_scissors(mut self, p_scissors: &'a [vk::Rect2D]) -> Self {
        self.inner.scissor_count = p_scissors.len() as u32;
        self.inner.p_scissors = p_scissors.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineViewportStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineRasterizationStateCreateInfoBuilder<'a> {
    inner: vk::PipelineRasterizationStateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineRasterizationStateCreateInfo {
    type Type = PipelineRasterizationStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineRasterizationStateCreateInfoNext {}
impl<'a> PipelineRasterizationStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineRasterizationStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PipelineRasterizationStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineRasterizationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineMultisampleStateCreateInfoBuilder<'a> {
    inner: vk::PipelineMultisampleStateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineMultisampleStateCreateInfo {
    type Type = PipelineMultisampleStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineMultisampleStateCreateInfoNext {}
impl<'a> PipelineMultisampleStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineMultisampleStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineMultisampleStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn rasterization_samples(mut self, rasterization_samples: vk::SampleCountFlags) -> Self {
        self.inner.rasterization_samples = rasterization_samples;
        self
    }
    pub fn sample_shading_enable(mut self, sample_shading_enable: bool) -> Self {
        self.inner.sample_shading_enable = if sample_shading_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn min_sample_shading(mut self, min_sample_shading: f32) -> Self {
        self.inner.min_sample_shading = min_sample_shading;
        self
    }
    pub fn p_sample_mask(mut self, p_sample_mask: *const vk::SampleMask) -> Self {
        self.inner.p_sample_mask = p_sample_mask;
        self
    }
    pub fn alpha_to_coverage_enable(mut self, alpha_to_coverage_enable: bool) -> Self {
        self.inner.alpha_to_coverage_enable = if alpha_to_coverage_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn alpha_to_one_enable(mut self, alpha_to_one_enable: bool) -> Self {
        self.inner.alpha_to_one_enable = if alpha_to_one_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PipelineMultisampleStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineMultisampleStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineColorBlendStateCreateInfoBuilder<'a> {
    inner: vk::PipelineColorBlendStateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineColorBlendStateCreateInfo {
    type Type = PipelineColorBlendStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PipelineColorBlendStateCreateInfoNext {}
impl<'a> PipelineColorBlendStateCreateInfoBuilder<'a> {
    pub fn insert_next<T: PipelineColorBlendStateCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineColorBlendStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn logic_op_enable(mut self, logic_op_enable: bool) -> Self {
        self.inner.logic_op_enable = if logic_op_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn logic_op(mut self, logic_op: vk::LogicOp) -> Self {
        self.inner.logic_op = logic_op;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::PipelineColorBlendAttachmentState]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
    pub fn blend_constants(mut self, blend_constants: [f32; 4]) -> Self {
        self.inner.blend_constants = blend_constants;
        self
    }
}
impl<'a> Deref for PipelineColorBlendStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineColorBlendStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineDynamicStateCreateInfoBuilder<'a> {
    inner: vk::PipelineDynamicStateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineDynamicStateCreateInfo {
    type Type = PipelineDynamicStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineDynamicStateCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDynamicStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_dynamic_states(mut self, p_dynamic_states: &'a [vk::DynamicState]) -> Self {
        self.inner.dynamic_state_count = p_dynamic_states.len() as u32;
        self.inner.p_dynamic_states = p_dynamic_states.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineDynamicStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineDynamicStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct GraphicsPipelineCreateInfoBuilder<'a> {
    inner: vk::GraphicsPipelineCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::GraphicsPipelineCreateInfo {
    type Type = GraphicsPipelineCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait GraphicsPipelineCreateInfoNext {}
impl<'a> GraphicsPipelineCreateInfoBuilder<'a> {
    pub fn insert_next<T: GraphicsPipelineCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage_count(mut self, stage_count: u32) -> Self {
        self.inner.stage_count = stage_count;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_vertex_input_state(
        mut self,
        p_vertex_input_state: Option<&'a vk::PipelineVertexInputStateCreateInfo>,
    ) -> Self {
        self.inner.p_vertex_input_state = p_vertex_input_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_input_assembly_state(
        mut self,
        p_input_assembly_state: Option<&'a vk::PipelineInputAssemblyStateCreateInfo>,
    ) -> Self {
        self.inner.p_input_assembly_state = p_input_assembly_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_tessellation_state(
        mut self,
        p_tessellation_state: Option<&'a vk::PipelineTessellationStateCreateInfo>,
    ) -> Self {
        self.inner.p_tessellation_state = p_tessellation_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_viewport_state(mut self, p_viewport_state: Option<&'a vk::PipelineViewportStateCreateInfo>) -> Self {
        self.inner.p_viewport_state = p_viewport_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_rasterization_state(
        mut self,
        p_rasterization_state: Option<&'a vk::PipelineRasterizationStateCreateInfo>,
    ) -> Self {
        self.inner.p_rasterization_state = p_rasterization_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_multisample_state(
        mut self,
        p_multisample_state: Option<&'a vk::PipelineMultisampleStateCreateInfo>,
    ) -> Self {
        self.inner.p_multisample_state = p_multisample_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_depth_stencil_state(
        mut self,
        p_depth_stencil_state: Option<&'a vk::PipelineDepthStencilStateCreateInfo>,
    ) -> Self {
        self.inner.p_depth_stencil_state = p_depth_stencil_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_color_blend_state(
        mut self,
        p_color_blend_state: Option<&'a vk::PipelineColorBlendStateCreateInfo>,
    ) -> Self {
        self.inner.p_color_blend_state = p_color_blend_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_dynamic_state(mut self, p_dynamic_state: Option<&'a vk::PipelineDynamicStateCreateInfo>) -> Self {
        self.inner.p_dynamic_state = p_dynamic_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = render_pass;
        self
    }
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.inner.subpass = subpass;
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: vk::Pipeline) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for GraphicsPipelineCreateInfoBuilder<'a> {
    type Target = vk::GraphicsPipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineCacheCreateInfoBuilder<'a> {
    inner: vk::PipelineCacheCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineCacheCreateInfo {
    type Type = PipelineCacheCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineCacheCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCacheCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn initial_data_size(mut self, initial_data_size: usize) -> Self {
        self.inner.initial_data_size = initial_data_size;
        self
    }
    pub fn p_initial_data(mut self, p_initial_data: *const c_void) -> Self {
        self.inner.p_initial_data = p_initial_data;
        self
    }
}
impl<'a> Deref for PipelineCacheCreateInfoBuilder<'a> {
    type Target = vk::PipelineCacheCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineBinaryCreateInfoKHRBuilder<'a> {
    inner: vk::PipelineBinaryCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineBinaryCreateInfoKHR {
    type Type = PipelineBinaryCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineBinaryCreateInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_keys_and_data_info(mut self, p_keys_and_data_info: Option<&'a vk::PipelineBinaryKeysAndDataKHR>) -> Self {
        self.inner.p_keys_and_data_info = p_keys_and_data_info.map_or(ptr::null(), |r| r);
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = pipeline;
        self
    }
    pub fn p_pipeline_create_info(mut self, p_pipeline_create_info: Option<&'a vk::PipelineCreateInfoKHR>) -> Self {
        self.inner.p_pipeline_create_info = p_pipeline_create_info.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for PipelineBinaryCreateInfoKHRBuilder<'a> {
    type Target = vk::PipelineBinaryCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineBinaryHandlesInfoKHRBuilder<'a> {
    inner: vk::PipelineBinaryHandlesInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineBinaryHandlesInfoKHR {
    type Type = PipelineBinaryHandlesInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineBinaryHandlesInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_pipeline_binaries(mut self, p_pipeline_binaries: &'a mut [vk::PipelineBinaryKHR]) -> Self {
        self.inner.pipeline_binary_count = p_pipeline_binaries.len() as u32;
        self.inner.p_pipeline_binaries = p_pipeline_binaries.as_mut_ptr();
        self
    }
}
impl<'a> Deref for PipelineBinaryHandlesInfoKHRBuilder<'a> {
    type Target = vk::PipelineBinaryHandlesInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineBinaryDataKHRBuilder<'a> {
    inner: vk::PipelineBinaryDataKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineBinaryDataKHR {
    type Type = PipelineBinaryDataKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineBinaryDataKHRBuilder<'a> {
    pub fn data_size(mut self, data_size: usize) -> Self {
        self.inner.data_size = data_size;
        self
    }
    pub fn p_data(mut self, p_data: *mut c_void) -> Self {
        self.inner.p_data = p_data;
        self
    }
}
impl<'a> Deref for PipelineBinaryDataKHRBuilder<'a> {
    type Target = vk::PipelineBinaryDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineBinaryKeysAndDataKHRBuilder<'a> {
    inner: vk::PipelineBinaryKeysAndDataKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineBinaryKeysAndDataKHR {
    type Type = PipelineBinaryKeysAndDataKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineBinaryKeysAndDataKHRBuilder<'a> {
    pub fn p_pipeline_binary_keys(
        mut self,
        p_pipeline_binary_keys: &'a [vk::PipelineBinaryKeyKHR],
        p_pipeline_binary_data: &'a [vk::PipelineBinaryDataKHR],
    ) -> Self {
        self.inner.binary_count = p_pipeline_binary_keys.len() as u32;
        assert_eq!(self.inner.binary_count, p_pipeline_binary_data.len() as u32);
        self.inner.p_pipeline_binary_keys = p_pipeline_binary_keys.as_ptr();
        self.inner.p_pipeline_binary_data = p_pipeline_binary_data.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineBinaryKeysAndDataKHRBuilder<'a> {
    type Target = vk::PipelineBinaryKeysAndDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineBinaryInfoKHRBuilder<'a> {
    inner: vk::PipelineBinaryInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineBinaryInfoKHR {
    type Type = PipelineBinaryInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineBinaryInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_pipeline_binaries(mut self, p_pipeline_binaries: &'a [vk::PipelineBinaryKHR]) -> Self {
        self.inner.binary_count = p_pipeline_binaries.len() as u32;
        self.inner.p_pipeline_binaries = p_pipeline_binaries.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineBinaryInfoKHRBuilder<'a> {
    type Target = vk::PipelineBinaryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for vk::PipelineBinaryInfoKHR {}
impl ComputePipelineCreateInfoNext for vk::PipelineBinaryInfoKHR {}
impl RayTracingPipelineCreateInfoKHRNext for vk::PipelineBinaryInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineLayoutCreateInfoBuilder<'a> {
    inner: vk::PipelineLayoutCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineLayoutCreateInfo {
    type Type = PipelineLayoutCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineLayoutCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineLayoutCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_set_layouts(mut self, p_set_layouts: &'a [vk::DescriptorSetLayout]) -> Self {
        self.inner.set_layout_count = p_set_layouts.len() as u32;
        self.inner.p_set_layouts = p_set_layouts.as_ptr();
        self
    }
    pub fn p_push_constant_ranges(mut self, p_push_constant_ranges: &'a [vk::PushConstantRange]) -> Self {
        self.inner.push_constant_range_count = p_push_constant_ranges.len() as u32;
        self.inner.p_push_constant_ranges = p_push_constant_ranges.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineLayoutCreateInfoBuilder<'a> {
    type Target = vk::PipelineLayoutCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BindDescriptorSetsInfoNext for vk::PipelineLayoutCreateInfo {}
impl PushConstantsInfoNext for vk::PipelineLayoutCreateInfo {}
impl PushDescriptorSetInfoNext for vk::PipelineLayoutCreateInfo {}
impl PushDescriptorSetWithTemplateInfoNext for vk::PipelineLayoutCreateInfo {}
impl SetDescriptorBufferOffsetsInfoEXTNext for vk::PipelineLayoutCreateInfo {}
impl BindDescriptorBufferEmbeddedSamplersInfoEXTNext for vk::PipelineLayoutCreateInfo {}
impl IndirectCommandsLayoutCreateInfoEXTNext for vk::PipelineLayoutCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct SamplerCreateInfoBuilder<'a> {
    inner: vk::SamplerCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SamplerCreateInfo {
    type Type = SamplerCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SamplerCreateInfoNext {}
impl<'a> SamplerCreateInfoBuilder<'a> {
    pub fn insert_next<T: SamplerCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for SamplerCreateInfoBuilder<'a> {
    type Target = vk::SamplerCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CommandBufferInheritanceInfoBuilder<'a> {
    inner: vk::CommandBufferInheritanceInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceInfo {
    type Type = CommandBufferInheritanceInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait CommandBufferInheritanceInfoNext {}
impl<'a> CommandBufferInheritanceInfoBuilder<'a> {
    pub fn insert_next<T: CommandBufferInheritanceInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceInfoBuilder<'a> {
    type Target = vk::CommandBufferInheritanceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CommandBufferBeginInfoBuilder<'a> {
    inner: vk::CommandBufferBeginInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CommandBufferBeginInfo {
    type Type = CommandBufferBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait CommandBufferBeginInfoNext {}
impl<'a> CommandBufferBeginInfoBuilder<'a> {
    pub fn insert_next<T: CommandBufferBeginInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::CommandBufferUsageFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_inheritance_info(mut self, p_inheritance_info: Option<&'a vk::CommandBufferInheritanceInfo>) -> Self {
        self.inner.p_inheritance_info = p_inheritance_info.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for CommandBufferBeginInfoBuilder<'a> {
    type Target = vk::CommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassBeginInfoBuilder<'a> {
    inner: vk::RenderPassBeginInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassBeginInfo {
    type Type = RenderPassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderPassBeginInfoNext {}
impl<'a> RenderPassBeginInfoBuilder<'a> {
    pub fn insert_next<T: RenderPassBeginInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = render_pass;
        self
    }
    pub fn framebuffer(mut self, framebuffer: vk::Framebuffer) -> Self {
        self.inner.framebuffer = framebuffer;
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
    pub fn clear_value_count(mut self, clear_value_count: u32) -> Self {
        self.inner.clear_value_count = clear_value_count;
        self
    }
    pub fn p_clear_values(mut self, p_clear_values: &'a [vk::ClearValue]) -> Self {
        self.inner.clear_value_count = p_clear_values.len() as u32;
        self.inner.p_clear_values = p_clear_values.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassBeginInfoBuilder<'a> {
    type Target = vk::RenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SubpassDescriptionBuilder<'a> {
    inner: vk::SubpassDescription,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SubpassDescription {
    type Type = SubpassDescriptionBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SubpassDescriptionBuilder<'a> {
    pub fn flags(mut self, flags: vk::SubpassDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn p_input_attachments(mut self, p_input_attachments: &'a [vk::AttachmentReference]) -> Self {
        self.inner.input_attachment_count = p_input_attachments.len() as u32;
        self.inner.p_input_attachments = p_input_attachments.as_ptr();
        self
    }
    pub fn p_color_attachments(
        mut self,
        p_color_attachments: &'a [vk::AttachmentReference],
        p_resolve_attachments: Option<&'a [vk::AttachmentReference]>,
    ) -> Self {
        self.inner.color_attachment_count = p_color_attachments.len() as u32;
        self.inner.p_color_attachments = p_color_attachments.as_ptr();
        self.inner.p_resolve_attachments = p_resolve_attachments.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn p_depth_stencil_attachment(
        mut self,
        p_depth_stencil_attachment: Option<&'a vk::AttachmentReference>,
    ) -> Self {
        self.inner.p_depth_stencil_attachment = p_depth_stencil_attachment.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_preserve_attachments(mut self, p_preserve_attachments: &'a [u32]) -> Self {
        self.inner.preserve_attachment_count = p_preserve_attachments.len() as u32;
        self.inner.p_preserve_attachments = p_preserve_attachments.as_ptr();
        self
    }
}
impl<'a> Deref for SubpassDescriptionBuilder<'a> {
    type Target = vk::SubpassDescription;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassCreateInfoBuilder<'a> {
    inner: vk::RenderPassCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassCreateInfo {
    type Type = RenderPassCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderPassCreateInfoNext {}
impl<'a> RenderPassCreateInfoBuilder<'a> {
    pub fn insert_next<T: RenderPassCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderPassCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::AttachmentDescription]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
    pub fn p_subpasses(mut self, p_subpasses: &'a [vk::SubpassDescription]) -> Self {
        self.inner.subpass_count = p_subpasses.len() as u32;
        self.inner.p_subpasses = p_subpasses.as_ptr();
        self
    }
    pub fn p_dependencies(mut self, p_dependencies: &'a [vk::SubpassDependency]) -> Self {
        self.inner.dependency_count = p_dependencies.len() as u32;
        self.inner.p_dependencies = p_dependencies.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassCreateInfoBuilder<'a> {
    type Target = vk::RenderPassCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct EventCreateInfoBuilder<'a> {
    inner: vk::EventCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::EventCreateInfo {
    type Type = EventCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait EventCreateInfoNext {}
impl<'a> EventCreateInfoBuilder<'a> {
    pub fn insert_next<T: EventCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for EventCreateInfoBuilder<'a> {
    type Target = vk::EventCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct FenceCreateInfoBuilder<'a> {
    inner: vk::FenceCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::FenceCreateInfo {
    type Type = FenceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait FenceCreateInfoNext {}
impl<'a> FenceCreateInfoBuilder<'a> {
    pub fn insert_next<T: FenceCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for FenceCreateInfoBuilder<'a> {
    type Target = vk::FenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SemaphoreCreateInfoBuilder<'a> {
    inner: vk::SemaphoreCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SemaphoreCreateInfo {
    type Type = SemaphoreCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SemaphoreCreateInfoNext {}
impl<'a> SemaphoreCreateInfoBuilder<'a> {
    pub fn insert_next<T: SemaphoreCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for SemaphoreCreateInfoBuilder<'a> {
    type Target = vk::SemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct QueryPoolCreateInfoBuilder<'a> {
    inner: vk::QueryPoolCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::QueryPoolCreateInfo {
    type Type = QueryPoolCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait QueryPoolCreateInfoNext {}
impl<'a> QueryPoolCreateInfoBuilder<'a> {
    pub fn insert_next<T: QueryPoolCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for QueryPoolCreateInfoBuilder<'a> {
    type Target = vk::QueryPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct FramebufferCreateInfoBuilder<'a> {
    inner: vk::FramebufferCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::FramebufferCreateInfo {
    type Type = FramebufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait FramebufferCreateInfoNext {}
impl<'a> FramebufferCreateInfoBuilder<'a> {
    pub fn insert_next<T: FramebufferCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::FramebufferCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = render_pass;
        self
    }
    pub fn attachment_count(mut self, attachment_count: u32) -> Self {
        self.inner.attachment_count = attachment_count;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::ImageView]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }
    pub fn layers(mut self, layers: u32) -> Self {
        self.inner.layers = layers;
        self
    }
}
impl<'a> Deref for FramebufferCreateInfoBuilder<'a> {
    type Target = vk::FramebufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SubmitInfoBuilder<'a> {
    inner: vk::SubmitInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SubmitInfo {
    type Type = SubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubmitInfoNext {}
impl<'a> SubmitInfoBuilder<'a> {
    pub fn insert_next<T: SubmitInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
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
    pub fn p_command_buffers(mut self, p_command_buffers: &'a [vk::CommandBuffer]) -> Self {
        self.inner.command_buffer_count = p_command_buffers.len() as u32;
        self.inner.p_command_buffers = p_command_buffers.as_ptr();
        self
    }
    pub fn p_signal_semaphores(mut self, p_signal_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphores.len() as u32;
        self.inner.p_signal_semaphores = p_signal_semaphores.as_ptr();
        self
    }
}
impl<'a> Deref for SubmitInfoBuilder<'a> {
    type Target = vk::SubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DisplaySurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::DisplaySurfaceCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DisplaySurfaceCreateInfoKHR {
    type Type = DisplaySurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DisplaySurfaceCreateInfoKHRNext {}
impl<'a> DisplaySurfaceCreateInfoKHRBuilder<'a> {
    pub fn insert_next<T: DisplaySurfaceCreateInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for DisplaySurfaceCreateInfoKHRBuilder<'a> {
    type Target = vk::DisplaySurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DisplaySurfaceCreateInfoKHRNext for vk::DisplaySurfaceStereoCreateInfoNV {}
impl PresentInfoKHRNext for vk::DisplayPresentInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct AndroidSurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::AndroidSurfaceCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AndroidSurfaceCreateInfoKHR {
    type Type = AndroidSurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AndroidSurfaceCreateInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AndroidSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn window(mut self, window: *mut vk::ANativeWindow) -> Self {
        self.inner.window = window;
        self
    }
}
impl<'a> Deref for AndroidSurfaceCreateInfoKHRBuilder<'a> {
    type Target = vk::AndroidSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ViSurfaceCreateInfoNNBuilder<'a> {
    inner: vk::ViSurfaceCreateInfoNN,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ViSurfaceCreateInfoNN {
    type Type = ViSurfaceCreateInfoNNBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ViSurfaceCreateInfoNNBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ViSurfaceCreateFlagsNN) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn window(mut self, window: *mut c_void) -> Self {
        self.inner.window = window;
        self
    }
}
impl<'a> Deref for ViSurfaceCreateInfoNNBuilder<'a> {
    type Target = vk::ViSurfaceCreateInfoNN;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct WaylandSurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::WaylandSurfaceCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::WaylandSurfaceCreateInfoKHR {
    type Type = WaylandSurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> WaylandSurfaceCreateInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::WaylandSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn display(mut self, display: *mut vk::wl_display) -> Self {
        self.inner.display = display;
        self
    }
    pub fn surface(mut self, surface: *mut vk::wl_surface) -> Self {
        self.inner.surface = surface;
        self
    }
}
impl<'a> Deref for WaylandSurfaceCreateInfoKHRBuilder<'a> {
    type Target = vk::WaylandSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct XlibSurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::XlibSurfaceCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::XlibSurfaceCreateInfoKHR {
    type Type = XlibSurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> XlibSurfaceCreateInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::XlibSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn dpy(mut self, dpy: *mut vk::Display) -> Self {
        self.inner.dpy = dpy;
        self
    }
    pub fn window(mut self, window: vk::Window) -> Self {
        self.inner.window = window;
        self
    }
}
impl<'a> Deref for XlibSurfaceCreateInfoKHRBuilder<'a> {
    type Target = vk::XlibSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct XcbSurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::XcbSurfaceCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::XcbSurfaceCreateInfoKHR {
    type Type = XcbSurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> XcbSurfaceCreateInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::XcbSurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn connection(mut self, connection: *mut vk::xcb_connection_t) -> Self {
        self.inner.connection = connection;
        self
    }
    pub fn window(mut self, window: vk::xcb_window_t) -> Self {
        self.inner.window = window;
        self
    }
}
impl<'a> Deref for XcbSurfaceCreateInfoKHRBuilder<'a> {
    type Target = vk::XcbSurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DirectFBSurfaceCreateInfoEXTBuilder<'a> {
    inner: vk::DirectFBSurfaceCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DirectFBSurfaceCreateInfoEXT {
    type Type = DirectFBSurfaceCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DirectFBSurfaceCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DirectFBSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn dfb(mut self, dfb: *mut vk::IDirectFB) -> Self {
        self.inner.dfb = dfb;
        self
    }
    pub fn surface(mut self, surface: *mut vk::IDirectFBSurface) -> Self {
        self.inner.surface = surface;
        self
    }
}
impl<'a> Deref for DirectFBSurfaceCreateInfoEXTBuilder<'a> {
    type Target = vk::DirectFBSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SwapchainCreateInfoKHRBuilder<'a> {
    inner: vk::SwapchainCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SwapchainCreateInfoKHR {
    type Type = SwapchainCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SwapchainCreateInfoKHRNext {}
impl<'a> SwapchainCreateInfoKHRBuilder<'a> {
    pub fn insert_next<T: SwapchainCreateInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SwapchainCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn surface(mut self, surface: vk::SurfaceKHR) -> Self {
        self.inner.surface = surface;
        self
    }
    pub fn min_image_count(mut self, min_image_count: u32) -> Self {
        self.inner.min_image_count = min_image_count;
        self
    }
    pub fn image_format(mut self, image_format: vk::Format) -> Self {
        self.inner.image_format = image_format;
        self
    }
    pub fn image_color_space(mut self, image_color_space: vk::ColorSpaceKHR) -> Self {
        self.inner.image_color_space = image_color_space;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent2D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
    pub fn image_array_layers(mut self, image_array_layers: u32) -> Self {
        self.inner.image_array_layers = image_array_layers;
        self
    }
    pub fn image_usage(mut self, image_usage: vk::ImageUsageFlags) -> Self {
        self.inner.image_usage = image_usage;
        self
    }
    pub fn image_sharing_mode(mut self, image_sharing_mode: vk::SharingMode) -> Self {
        self.inner.image_sharing_mode = image_sharing_mode;
        self
    }
    pub fn queue_family_index_count(mut self, queue_family_index_count: u32) -> Self {
        self.inner.queue_family_index_count = queue_family_index_count;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
    pub fn pre_transform(mut self, pre_transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.pre_transform = pre_transform;
        self
    }
    pub fn composite_alpha(mut self, composite_alpha: vk::CompositeAlphaFlagsKHR) -> Self {
        self.inner.composite_alpha = composite_alpha;
        self
    }
    pub fn present_mode(mut self, present_mode: vk::PresentModeKHR) -> Self {
        self.inner.present_mode = present_mode;
        self
    }
    pub fn clipped(mut self, clipped: bool) -> Self {
        self.inner.clipped = if clipped { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn old_swapchain(mut self, old_swapchain: vk::SwapchainKHR) -> Self {
        self.inner.old_swapchain = old_swapchain;
        self
    }
}
impl<'a> Deref for SwapchainCreateInfoKHRBuilder<'a> {
    type Target = vk::SwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PresentInfoKHRBuilder<'a> {
    inner: vk::PresentInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PresentInfoKHR {
    type Type = PresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PresentInfoKHRNext {}
impl<'a> PresentInfoKHRBuilder<'a> {
    pub fn insert_next<T: PresentInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphores(mut self, p_wait_semaphores: &'a [vk::Semaphore]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphores.len() as u32;
        self.inner.p_wait_semaphores = p_wait_semaphores.as_ptr();
        self
    }
    pub fn p_swapchains(
        mut self,
        p_swapchains: &'a [vk::SwapchainKHR],
        p_image_indices: &'a [u32],
        p_results: Option<&'a mut [vk::Result]>,
    ) -> Self {
        self.inner.swapchain_count = p_swapchains.len() as u32;
        assert_eq!(self.inner.swapchain_count, p_image_indices.len() as u32);
        self.inner.p_swapchains = p_swapchains.as_ptr();
        self.inner.p_image_indices = p_image_indices.as_ptr();
        self.inner.p_results = p_results.map_or(ptr::null_mut(), |s| s.as_mut_ptr());
        self
    }
}
impl<'a> Deref for PresentInfoKHRBuilder<'a> {
    type Target = vk::PresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugReportCallbackCreateInfoEXTBuilder<'a> {
    inner: vk::DebugReportCallbackCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugReportCallbackCreateInfoEXT {
    type Type = DebugReportCallbackCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DebugReportCallbackCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugReportFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pfn_callback(mut self, pfn_callback: vk::FnDebugReportCallbackEXT) -> Self {
        self.inner.pfn_callback = Some(pfn_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl<'a> Deref for DebugReportCallbackCreateInfoEXTBuilder<'a> {
    type Target = vk::DebugReportCallbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl InstanceCreateInfoNext for vk::DebugReportCallbackCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ValidationFlagsEXTBuilder<'a> {
    inner: vk::ValidationFlagsEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ValidationFlagsEXT {
    type Type = ValidationFlagsEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ValidationFlagsEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_disabled_validation_checks(mut self, p_disabled_validation_checks: &'a [vk::ValidationCheckEXT]) -> Self {
        self.inner.disabled_validation_check_count = p_disabled_validation_checks.len() as u32;
        self.inner.p_disabled_validation_checks = p_disabled_validation_checks.as_ptr();
        self
    }
}
impl<'a> Deref for ValidationFlagsEXTBuilder<'a> {
    type Target = vk::ValidationFlagsEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl InstanceCreateInfoNext for vk::ValidationFlagsEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ValidationFeaturesEXTBuilder<'a> {
    inner: vk::ValidationFeaturesEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ValidationFeaturesEXT {
    type Type = ValidationFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ValidationFeaturesEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_enabled_validation_features(
        mut self,
        p_enabled_validation_features: &'a [vk::ValidationFeatureEnableEXT],
    ) -> Self {
        self.inner.enabled_validation_feature_count = p_enabled_validation_features.len() as u32;
        self.inner.p_enabled_validation_features = p_enabled_validation_features.as_ptr();
        self
    }
    pub fn p_disabled_validation_features(
        mut self,
        p_disabled_validation_features: &'a [vk::ValidationFeatureDisableEXT],
    ) -> Self {
        self.inner.disabled_validation_feature_count = p_disabled_validation_features.len() as u32;
        self.inner.p_disabled_validation_features = p_disabled_validation_features.as_ptr();
        self
    }
}
impl<'a> Deref for ValidationFeaturesEXTBuilder<'a> {
    type Target = vk::ValidationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl InstanceCreateInfoNext for vk::ValidationFeaturesEXT {}
impl ShaderModuleCreateInfoNext for vk::ValidationFeaturesEXT {}
impl ShaderCreateInfoEXTNext for vk::ValidationFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct LayerSettingsCreateInfoEXTBuilder<'a> {
    inner: vk::LayerSettingsCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::LayerSettingsCreateInfoEXT {
    type Type = LayerSettingsCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> LayerSettingsCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_settings(mut self, p_settings: &'a [vk::LayerSettingEXT]) -> Self {
        self.inner.setting_count = p_settings.len() as u32;
        self.inner.p_settings = p_settings.as_ptr();
        self
    }
}
impl<'a> Deref for LayerSettingsCreateInfoEXTBuilder<'a> {
    type Target = vk::LayerSettingsCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl InstanceCreateInfoNext for vk::LayerSettingsCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct LayerSettingEXTBuilder<'a> {
    inner: vk::LayerSettingEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::LayerSettingEXT {
    type Type = LayerSettingEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> LayerSettingEXTBuilder<'a> {
    pub fn p_layer_name(mut self, p_layer_name: &'a CStr) -> Self {
        self.inner.p_layer_name = p_layer_name.as_ptr();
        self
    }
    pub fn p_setting_name(mut self, p_setting_name: &'a CStr) -> Self {
        self.inner.p_setting_name = p_setting_name.as_ptr();
        self
    }
    pub fn ty(mut self, ty: vk::LayerSettingTypeEXT) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn value_count(mut self, value_count: u32) -> Self {
        self.inner.value_count = value_count;
        self
    }
    pub fn p_values(mut self, p_values: *const c_void) -> Self {
        self.inner.p_values = p_values;
        self
    }
}
impl<'a> Deref for LayerSettingEXTBuilder<'a> {
    type Target = vk::LayerSettingEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationStateRasterizationOrderAMD {}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugMarkerObjectNameInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerObjectNameInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugMarkerObjectNameInfoEXT {
    type Type = DebugMarkerObjectNameInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DebugMarkerObjectNameInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::DebugReportObjectTypeEXT) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object(mut self, object: u64) -> Self {
        self.inner.object = object;
        self
    }
    pub fn p_object_name(mut self, p_object_name: &'a CStr) -> Self {
        self.inner.p_object_name = p_object_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugMarkerObjectNameInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugMarkerObjectTagInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerObjectTagInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugMarkerObjectTagInfoEXT {
    type Type = DebugMarkerObjectTagInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DebugMarkerObjectTagInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::DebugReportObjectTypeEXT) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object(mut self, object: u64) -> Self {
        self.inner.object = object;
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.inner.tag_name = tag_name;
        self
    }
    pub fn tag_size(mut self, tag_size: usize) -> Self {
        self.inner.tag_size = tag_size;
        self
    }
    pub fn p_tag(mut self, p_tag: *const c_void) -> Self {
        self.inner.p_tag = p_tag;
        self
    }
}
impl<'a> Deref for DebugMarkerObjectTagInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerObjectTagInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugMarkerMarkerInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerMarkerInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugMarkerMarkerInfoEXT {
    type Type = DebugMarkerMarkerInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DebugMarkerMarkerInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_marker_name(mut self, p_marker_name: &'a CStr) -> Self {
        self.inner.p_marker_name = p_marker_name.as_ptr();
        self
    }
    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.inner.color = color;
        self
    }
}
impl<'a> Deref for DebugMarkerMarkerInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerMarkerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for vk::DedicatedAllocationImageCreateInfoNV {}
impl BufferCreateInfoNext for vk::DedicatedAllocationBufferCreateInfoNV {}
impl MemoryAllocateInfoNext for vk::DedicatedAllocationMemoryAllocateInfoNV {}
impl ImageCreateInfoNext for vk::ExternalMemoryImageCreateInfoNV {}
impl MemoryAllocateInfoNext for vk::ExportMemoryAllocateInfoNV {}
impl MemoryAllocateInfoNext for vk::ImportMemoryWin32HandleInfoNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct ExportMemoryWin32HandleInfoNVBuilder<'a> {
    inner: vk::ExportMemoryWin32HandleInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ExportMemoryWin32HandleInfoNV {
    type Type = ExportMemoryWin32HandleInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ExportMemoryWin32HandleInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |r| r);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
}
impl<'a> Deref for ExportMemoryWin32HandleInfoNVBuilder<'a> {
    type Target = vk::ExportMemoryWin32HandleInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for vk::ExportMemoryWin32HandleInfoNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    inner: vk::Win32KeyedMutexAcquireReleaseInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::Win32KeyedMutexAcquireReleaseInfoNV {
    type Type = Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acquire_syncs(
        mut self,
        p_acquire_syncs: &'a [vk::DeviceMemory],
        p_acquire_keys: &'a [u64],
        p_acquire_timeout_milliseconds: &'a [u32],
    ) -> Self {
        self.inner.acquire_count = p_acquire_syncs.len() as u32;
        assert_eq!(self.inner.acquire_count, p_acquire_keys.len() as u32);
        assert_eq!(self.inner.acquire_count, p_acquire_timeout_milliseconds.len() as u32);
        self.inner.p_acquire_syncs = p_acquire_syncs.as_ptr();
        self.inner.p_acquire_keys = p_acquire_keys.as_ptr();
        self.inner.p_acquire_timeout_milliseconds = p_acquire_timeout_milliseconds.as_ptr();
        self
    }
    pub fn p_release_syncs(mut self, p_release_syncs: &'a [vk::DeviceMemory], p_release_keys: &'a [u64]) -> Self {
        self.inner.release_count = p_release_syncs.len() as u32;
        assert_eq!(self.inner.release_count, p_release_keys.len() as u32);
        self.inner.p_release_syncs = p_release_syncs.as_ptr();
        self.inner.p_release_keys = p_release_keys.as_ptr();
        self
    }
}
impl<'a> Deref for Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    type Target = vk::Win32KeyedMutexAcquireReleaseInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for vk::Win32KeyedMutexAcquireReleaseInfoNV {}
impl SubmitInfo2Next for vk::Win32KeyedMutexAcquireReleaseInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {}
impl DeviceCreateInfoNext for vk::DevicePrivateDataCreateInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePrivateDataFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePrivateDataFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDeviceGeneratedCommandsPropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceClusterAccelerationStructureFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceClusterAccelerationStructureFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceClusterAccelerationStructurePropertiesNV {}
impl RayTracingPipelineCreateInfoKHRNext for vk::RayTracingPipelineClusterAccelerationStructureCreateInfoNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMultiDrawPropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    inner: vk::GraphicsShaderGroupCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::GraphicsShaderGroupCreateInfoNV {
    type Type = GraphicsShaderGroupCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_vertex_input_state(
        mut self,
        p_vertex_input_state: Option<&'a vk::PipelineVertexInputStateCreateInfo>,
    ) -> Self {
        self.inner.p_vertex_input_state = p_vertex_input_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_tessellation_state(
        mut self,
        p_tessellation_state: Option<&'a vk::PipelineTessellationStateCreateInfo>,
    ) -> Self {
        self.inner.p_tessellation_state = p_tessellation_state.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    type Target = vk::GraphicsShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    inner: vk::GraphicsPipelineShaderGroupsCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::GraphicsPipelineShaderGroupsCreateInfoNV {
    type Type = GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::GraphicsShaderGroupCreateInfoNV]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.as_ptr();
        self
    }
    pub fn p_pipelines(mut self, p_pipelines: &'a [vk::Pipeline]) -> Self {
        self.inner.pipeline_count = p_pipelines.len() as u32;
        self.inner.p_pipelines = p_pipelines.as_ptr();
        self
    }
}
impl<'a> Deref for GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    type Target = vk::GraphicsPipelineShaderGroupsCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for vk::GraphicsPipelineShaderGroupsCreateInfoNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct IndirectCommandsLayoutTokenNVBuilder<'a> {
    inner: vk::IndirectCommandsLayoutTokenNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutTokenNV {
    type Type = IndirectCommandsLayoutTokenNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> IndirectCommandsLayoutTokenNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn token_type(mut self, token_type: vk::IndirectCommandsTokenTypeNV) -> Self {
        self.inner.token_type = token_type;
        self
    }
    pub fn stream(mut self, stream: u32) -> Self {
        self.inner.stream = stream;
        self
    }
    pub fn offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn vertex_binding_unit(mut self, vertex_binding_unit: u32) -> Self {
        self.inner.vertex_binding_unit = vertex_binding_unit;
        self
    }
    pub fn vertex_dynamic_stride(mut self, vertex_dynamic_stride: bool) -> Self {
        self.inner.vertex_dynamic_stride = if vertex_dynamic_stride { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn pushconstant_pipeline_layout(mut self, pushconstant_pipeline_layout: vk::PipelineLayout) -> Self {
        self.inner.pushconstant_pipeline_layout = pushconstant_pipeline_layout;
        self
    }
    pub fn pushconstant_shader_stage_flags(mut self, pushconstant_shader_stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.pushconstant_shader_stage_flags = pushconstant_shader_stage_flags;
        self
    }
    pub fn pushconstant_offset(mut self, pushconstant_offset: u32) -> Self {
        self.inner.pushconstant_offset = pushconstant_offset;
        self
    }
    pub fn pushconstant_size(mut self, pushconstant_size: u32) -> Self {
        self.inner.pushconstant_size = pushconstant_size;
        self
    }
    pub fn indirect_state_flags(mut self, indirect_state_flags: vk::IndirectStateFlagsNV) -> Self {
        self.inner.indirect_state_flags = indirect_state_flags;
        self
    }
    pub fn p_index_types(mut self, p_index_types: &'a [vk::IndexType], p_index_type_values: &'a [u32]) -> Self {
        self.inner.index_type_count = p_index_types.len() as u32;
        assert_eq!(self.inner.index_type_count, p_index_type_values.len() as u32);
        self.inner.p_index_types = p_index_types.as_ptr();
        self.inner.p_index_type_values = p_index_type_values.as_ptr();
        self
    }
}
impl<'a> Deref for IndirectCommandsLayoutTokenNVBuilder<'a> {
    type Target = vk::IndirectCommandsLayoutTokenNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    inner: vk::IndirectCommandsLayoutCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutCreateInfoNV {
    type Type = IndirectCommandsLayoutCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::IndirectCommandsLayoutUsageFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn p_tokens(mut self, p_tokens: &'a [vk::IndirectCommandsLayoutTokenNV]) -> Self {
        self.inner.token_count = p_tokens.len() as u32;
        self.inner.p_tokens = p_tokens.as_ptr();
        self
    }
    pub fn p_stream_strides(mut self, p_stream_strides: &'a [u32]) -> Self {
        self.inner.stream_count = p_stream_strides.len() as u32;
        self.inner.p_stream_strides = p_stream_strides.as_ptr();
        self
    }
}
impl<'a> Deref for IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    type Target = vk::IndirectCommandsLayoutCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct GeneratedCommandsInfoNVBuilder<'a> {
    inner: vk::GeneratedCommandsInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::GeneratedCommandsInfoNV {
    type Type = GeneratedCommandsInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> GeneratedCommandsInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = pipeline;
        self
    }
    pub fn indirect_commands_layout(mut self, indirect_commands_layout: vk::IndirectCommandsLayoutNV) -> Self {
        self.inner.indirect_commands_layout = indirect_commands_layout;
        self
    }
    pub fn p_streams(mut self, p_streams: &'a [vk::IndirectCommandsStreamNV]) -> Self {
        self.inner.stream_count = p_streams.len() as u32;
        self.inner.p_streams = p_streams.as_ptr();
        self
    }
    pub fn sequences_count(mut self, sequences_count: u32) -> Self {
        self.inner.sequences_count = sequences_count;
        self
    }
    pub fn preprocess_buffer(mut self, preprocess_buffer: vk::Buffer) -> Self {
        self.inner.preprocess_buffer = preprocess_buffer;
        self
    }
    pub fn preprocess_offset(mut self, preprocess_offset: vk::DeviceSize) -> Self {
        self.inner.preprocess_offset = preprocess_offset;
        self
    }
    pub fn preprocess_size(mut self, preprocess_size: vk::DeviceSize) -> Self {
        self.inner.preprocess_size = preprocess_size;
        self
    }
    pub fn sequences_count_buffer(mut self, sequences_count_buffer: vk::Buffer) -> Self {
        self.inner.sequences_count_buffer = sequences_count_buffer;
        self
    }
    pub fn sequences_count_offset(mut self, sequences_count_offset: vk::DeviceSize) -> Self {
        self.inner.sequences_count_offset = sequences_count_offset;
        self
    }
    pub fn sequences_index_buffer(mut self, sequences_index_buffer: vk::Buffer) -> Self {
        self.inner.sequences_index_buffer = sequences_index_buffer;
        self
    }
    pub fn sequences_index_offset(mut self, sequences_index_offset: vk::DeviceSize) -> Self {
        self.inner.sequences_index_offset = sequences_index_offset;
        self
    }
}
impl<'a> Deref for GeneratedCommandsInfoNVBuilder<'a> {
    type Target = vk::GeneratedCommandsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceFeatures2Builder<'a> {
    inner: vk::PhysicalDeviceFeatures2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceFeatures2 {
    type Type = PhysicalDeviceFeatures2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceFeatures2Next {}
impl<'a> PhysicalDeviceFeatures2Builder<'a> {
    pub fn insert_next<T: PhysicalDeviceFeatures2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceFeatures2Builder<'a> {
    type Target = vk::PhysicalDeviceFeatures2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFeatures2 {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceProperties2Builder<'a> {
    inner: vk::PhysicalDeviceProperties2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceProperties2 {
    type Type = PhysicalDeviceProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceProperties2Next {}
impl<'a> PhysicalDeviceProperties2Builder<'a> {
    pub fn insert_next<T: PhysicalDeviceProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceProperties2Builder<'a> {
    type Target = vk::PhysicalDeviceProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct FormatProperties2Builder<'a> {
    inner: vk::FormatProperties2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::FormatProperties2 {
    type Type = FormatProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait FormatProperties2Next {}
impl<'a> FormatProperties2Builder<'a> {
    pub fn insert_next<T: FormatProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for FormatProperties2Builder<'a> {
    type Target = vk::FormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageFormatProperties2Builder<'a> {
    inner: vk::ImageFormatProperties2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageFormatProperties2 {
    type Type = ImageFormatProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageFormatProperties2Next {}
impl<'a> ImageFormatProperties2Builder<'a> {
    pub fn insert_next<T: ImageFormatProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for ImageFormatProperties2Builder<'a> {
    type Target = vk::ImageFormatProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceImageFormatInfo2Builder<'a> {
    inner: vk::PhysicalDeviceImageFormatInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceImageFormatInfo2 {
    type Type = PhysicalDeviceImageFormatInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceImageFormatInfo2Next {}
impl<'a> PhysicalDeviceImageFormatInfo2Builder<'a> {
    pub fn insert_next<T: PhysicalDeviceImageFormatInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceImageFormatInfo2Builder<'a> {
    type Target = vk::PhysicalDeviceImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct QueueFamilyProperties2Builder<'a> {
    inner: vk::QueueFamilyProperties2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::QueueFamilyProperties2 {
    type Type = QueueFamilyProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait QueueFamilyProperties2Next {}
impl<'a> QueueFamilyProperties2Builder<'a> {
    pub fn insert_next<T: QueueFamilyProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for QueueFamilyProperties2Builder<'a> {
    type Target = vk::QueueFamilyProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceMemoryProperties2Builder<'a> {
    inner: vk::PhysicalDeviceMemoryProperties2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceMemoryProperties2 {
    type Type = PhysicalDeviceMemoryProperties2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceMemoryProperties2Next {}
impl<'a> PhysicalDeviceMemoryProperties2Builder<'a> {
    pub fn insert_next<T: PhysicalDeviceMemoryProperties2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceMemoryProperties2Builder<'a> {
    type Target = vk::PhysicalDeviceMemoryProperties2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePushDescriptorProperties {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDriverProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct PresentRegionsKHRBuilder<'a> {
    inner: vk::PresentRegionsKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PresentRegionsKHR {
    type Type = PresentRegionsKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PresentRegionsKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::PresentRegionKHR]) -> Self {
        self.inner.swapchain_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for PresentRegionsKHRBuilder<'a> {
    type Target = vk::PresentRegionsKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PresentInfoKHRNext for vk::PresentRegionsKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct PresentRegionKHRBuilder<'a> {
    inner: vk::PresentRegionKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PresentRegionKHR {
    type Type = PresentRegionKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PresentRegionKHRBuilder<'a> {
    pub fn p_rectangles(mut self, p_rectangles: &'a [vk::RectLayerKHR]) -> Self {
        self.inner.rectangle_count = p_rectangles.len() as u32;
        self.inner.p_rectangles = p_rectangles.as_ptr();
        self
    }
}
impl<'a> Deref for PresentRegionKHRBuilder<'a> {
    type Target = vk::PresentRegionKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVariablePointersFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVariablePointersFeatures {}
impl PhysicalDeviceImageFormatInfo2Next for vk::PhysicalDeviceExternalImageFormatInfo {}
impl ImageFormatProperties2Next for vk::ExternalImageFormatProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceExternalBufferInfoBuilder<'a> {
    inner: vk::PhysicalDeviceExternalBufferInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceExternalBufferInfo {
    type Type = PhysicalDeviceExternalBufferInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceExternalBufferInfoNext {}
impl<'a> PhysicalDeviceExternalBufferInfoBuilder<'a> {
    pub fn insert_next<T: PhysicalDeviceExternalBufferInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    type Target = vk::PhysicalDeviceExternalBufferInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceIDProperties {}
impl ImageCreateInfoNext for vk::ExternalMemoryImageCreateInfo {}
impl BufferCreateInfoNext for vk::ExternalMemoryBufferCreateInfo {}
impl MemoryAllocateInfoNext for vk::ExportMemoryAllocateInfo {}
impl MemoryAllocateInfoNext for vk::ImportMemoryWin32HandleInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportMemoryWin32HandleInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ExportMemoryWin32HandleInfoKHR {
    type Type = ExportMemoryWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |r| r);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportMemoryWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for vk::ExportMemoryWin32HandleInfoKHR {}
impl MemoryAllocateInfoNext for vk::ImportMemoryZirconHandleInfoFUCHSIA {}
impl MemoryAllocateInfoNext for vk::ImportMemoryFdInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    inner: vk::Win32KeyedMutexAcquireReleaseInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::Win32KeyedMutexAcquireReleaseInfoKHR {
    type Type = Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acquire_syncs(
        mut self,
        p_acquire_syncs: &'a [vk::DeviceMemory],
        p_acquire_keys: &'a [u64],
        p_acquire_timeouts: &'a [u32],
    ) -> Self {
        self.inner.acquire_count = p_acquire_syncs.len() as u32;
        assert_eq!(self.inner.acquire_count, p_acquire_keys.len() as u32);
        assert_eq!(self.inner.acquire_count, p_acquire_timeouts.len() as u32);
        self.inner.p_acquire_syncs = p_acquire_syncs.as_ptr();
        self.inner.p_acquire_keys = p_acquire_keys.as_ptr();
        self.inner.p_acquire_timeouts = p_acquire_timeouts.as_ptr();
        self
    }
    pub fn p_release_syncs(mut self, p_release_syncs: &'a [vk::DeviceMemory], p_release_keys: &'a [u64]) -> Self {
        self.inner.release_count = p_release_syncs.len() as u32;
        assert_eq!(self.inner.release_count, p_release_keys.len() as u32);
        self.inner.p_release_syncs = p_release_syncs.as_ptr();
        self.inner.p_release_keys = p_release_keys.as_ptr();
        self
    }
}
impl<'a> Deref for Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    type Target = vk::Win32KeyedMutexAcquireReleaseInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for vk::Win32KeyedMutexAcquireReleaseInfoKHR {}
impl SubmitInfo2Next for vk::Win32KeyedMutexAcquireReleaseInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImportMemoryMetalHandleInfoEXTBuilder<'a> {
    inner: vk::ImportMemoryMetalHandleInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImportMemoryMetalHandleInfoEXT {
    type Type = ImportMemoryMetalHandleInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImportMemoryMetalHandleInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: *mut c_void) -> Self {
        self.inner.handle = handle;
        self
    }
}
impl<'a> Deref for ImportMemoryMetalHandleInfoEXTBuilder<'a> {
    type Target = vk::ImportMemoryMetalHandleInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for vk::ImportMemoryMetalHandleInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    inner: vk::PhysicalDeviceExternalSemaphoreInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceExternalSemaphoreInfo {
    type Type = PhysicalDeviceExternalSemaphoreInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceExternalSemaphoreInfoNext {}
impl<'a> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    pub fn insert_next<T: PhysicalDeviceExternalSemaphoreInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    type Target = vk::PhysicalDeviceExternalSemaphoreInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SemaphoreCreateInfoNext for vk::ExportSemaphoreCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportSemaphoreWin32HandleInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ExportSemaphoreWin32HandleInfoKHR {
    type Type = ExportSemaphoreWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |r| r);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SemaphoreCreateInfoNext for vk::ExportSemaphoreWin32HandleInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct D3D12FenceSubmitInfoKHRBuilder<'a> {
    inner: vk::D3D12FenceSubmitInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::D3D12FenceSubmitInfoKHR {
    type Type = D3D12FenceSubmitInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> D3D12FenceSubmitInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_values(mut self, p_wait_semaphore_values: &'a [u64]) -> Self {
        self.inner.wait_semaphore_values_count = p_wait_semaphore_values.len() as u32;
        self.inner.p_wait_semaphore_values = p_wait_semaphore_values.as_ptr();
        self
    }
    pub fn p_signal_semaphore_values(mut self, p_signal_semaphore_values: &'a [u64]) -> Self {
        self.inner.signal_semaphore_values_count = p_signal_semaphore_values.len() as u32;
        self.inner.p_signal_semaphore_values = p_signal_semaphore_values.as_ptr();
        self
    }
}
impl<'a> Deref for D3D12FenceSubmitInfoKHRBuilder<'a> {
    type Target = vk::D3D12FenceSubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for vk::D3D12FenceSubmitInfoKHR {}
impl FenceCreateInfoNext for vk::ExportFenceCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct ExportFenceWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportFenceWin32HandleInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ExportFenceWin32HandleInfoKHR {
    type Type = ExportFenceWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ExportFenceWin32HandleInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |r| r);
        self
    }
    pub fn dw_access(mut self, dw_access: vk::DWORD) -> Self {
        self.inner.dw_access = dw_access;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ExportFenceWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ExportFenceWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl FenceCreateInfoNext for vk::ExportFenceWin32HandleInfoKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMultiviewFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMultiviewFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMultiviewProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassMultiviewCreateInfoBuilder<'a> {
    inner: vk::RenderPassMultiviewCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassMultiviewCreateInfo {
    type Type = RenderPassMultiviewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassMultiviewCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_view_masks(mut self, p_view_masks: &'a [u32]) -> Self {
        self.inner.subpass_count = p_view_masks.len() as u32;
        self.inner.p_view_masks = p_view_masks.as_ptr();
        self
    }
    pub fn p_view_offsets(mut self, p_view_offsets: &'a [i32]) -> Self {
        self.inner.dependency_count = p_view_offsets.len() as u32;
        self.inner.p_view_offsets = p_view_offsets.as_ptr();
        self
    }
    pub fn p_correlation_masks(mut self, p_correlation_masks: &'a [u32]) -> Self {
        self.inner.correlation_mask_count = p_correlation_masks.len() as u32;
        self.inner.p_correlation_masks = p_correlation_masks.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassMultiviewCreateInfoBuilder<'a> {
    type Target = vk::RenderPassMultiviewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassCreateInfoNext for vk::RenderPassMultiviewCreateInfo {}
impl SwapchainCreateInfoKHRNext for vk::SwapchainCounterCreateInfoEXT {}
impl MemoryAllocateInfoNext for vk::MemoryAllocateFlagsInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct BindBufferMemoryInfoBuilder<'a> {
    inner: vk::BindBufferMemoryInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindBufferMemoryInfo {
    type Type = BindBufferMemoryInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BindBufferMemoryInfoNext {}
impl<'a> BindBufferMemoryInfoBuilder<'a> {
    pub fn insert_next<T: BindBufferMemoryInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for BindBufferMemoryInfoBuilder<'a> {
    type Target = vk::BindBufferMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    inner: vk::BindBufferMemoryDeviceGroupInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindBufferMemoryDeviceGroupInfo {
    type Type = BindBufferMemoryDeviceGroupInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.as_ptr();
        self
    }
}
impl<'a> Deref for BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    type Target = vk::BindBufferMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BindBufferMemoryInfoNext for vk::BindBufferMemoryDeviceGroupInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct BindImageMemoryInfoBuilder<'a> {
    inner: vk::BindImageMemoryInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindImageMemoryInfo {
    type Type = BindImageMemoryInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BindImageMemoryInfoNext {}
impl<'a> BindImageMemoryInfoBuilder<'a> {
    pub fn insert_next<T: BindImageMemoryInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for BindImageMemoryInfoBuilder<'a> {
    type Target = vk::BindImageMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BindImageMemoryDeviceGroupInfoBuilder<'a> {
    inner: vk::BindImageMemoryDeviceGroupInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindImageMemoryDeviceGroupInfo {
    type Type = BindImageMemoryDeviceGroupInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> BindImageMemoryDeviceGroupInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.as_ptr();
        self
    }
    pub fn p_split_instance_bind_regions(mut self, p_split_instance_bind_regions: &'a [vk::Rect2D]) -> Self {
        self.inner.split_instance_bind_region_count = p_split_instance_bind_regions.len() as u32;
        self.inner.p_split_instance_bind_regions = p_split_instance_bind_regions.as_ptr();
        self
    }
}
impl<'a> Deref for BindImageMemoryDeviceGroupInfoBuilder<'a> {
    type Target = vk::BindImageMemoryDeviceGroupInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BindImageMemoryInfoNext for vk::BindImageMemoryDeviceGroupInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceGroupRenderPassBeginInfoBuilder<'a> {
    inner: vk::DeviceGroupRenderPassBeginInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceGroupRenderPassBeginInfo {
    type Type = DeviceGroupRenderPassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceGroupRenderPassBeginInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
    pub fn p_device_render_areas(mut self, p_device_render_areas: &'a [vk::Rect2D]) -> Self {
        self.inner.device_render_area_count = p_device_render_areas.len() as u32;
        self.inner.p_device_render_areas = p_device_render_areas.as_ptr();
        self
    }
}
impl<'a> Deref for DeviceGroupRenderPassBeginInfoBuilder<'a> {
    type Target = vk::DeviceGroupRenderPassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassBeginInfoNext for vk::DeviceGroupRenderPassBeginInfo {}
impl RenderingInfoNext for vk::DeviceGroupRenderPassBeginInfo {}
impl CommandBufferBeginInfoNext for vk::DeviceGroupCommandBufferBeginInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceGroupSubmitInfoBuilder<'a> {
    inner: vk::DeviceGroupSubmitInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceGroupSubmitInfo {
    type Type = DeviceGroupSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceGroupSubmitInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_device_indices(mut self, p_wait_semaphore_device_indices: &'a [u32]) -> Self {
        self.inner.wait_semaphore_count = p_wait_semaphore_device_indices.len() as u32;
        self.inner.p_wait_semaphore_device_indices = p_wait_semaphore_device_indices.as_ptr();
        self
    }
    pub fn p_command_buffer_device_masks(mut self, p_command_buffer_device_masks: &'a [u32]) -> Self {
        self.inner.command_buffer_count = p_command_buffer_device_masks.len() as u32;
        self.inner.p_command_buffer_device_masks = p_command_buffer_device_masks.as_ptr();
        self
    }
    pub fn p_signal_semaphore_device_indices(mut self, p_signal_semaphore_device_indices: &'a [u32]) -> Self {
        self.inner.signal_semaphore_count = p_signal_semaphore_device_indices.len() as u32;
        self.inner.p_signal_semaphore_device_indices = p_signal_semaphore_device_indices.as_ptr();
        self
    }
}
impl<'a> Deref for DeviceGroupSubmitInfoBuilder<'a> {
    type Target = vk::DeviceGroupSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for vk::DeviceGroupSubmitInfo {}
impl BindSparseInfoNext for vk::DeviceGroupBindSparseInfo {}
impl ImageCreateInfoNext for vk::ImageSwapchainCreateInfoKHR {}
impl BindImageMemoryInfoNext for vk::BindImageMemorySwapchainInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceGroupPresentInfoKHRBuilder<'a> {
    inner: vk::DeviceGroupPresentInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceGroupPresentInfoKHR {
    type Type = DeviceGroupPresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceGroupPresentInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_device_masks(mut self, p_device_masks: &'a [u32]) -> Self {
        self.inner.swapchain_count = p_device_masks.len() as u32;
        self.inner.p_device_masks = p_device_masks.as_ptr();
        self
    }
    pub fn mode(mut self, mode: vk::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl<'a> Deref for DeviceGroupPresentInfoKHRBuilder<'a> {
    type Target = vk::DeviceGroupPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PresentInfoKHRNext for vk::DeviceGroupPresentInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceGroupDeviceCreateInfoBuilder<'a> {
    inner: vk::DeviceGroupDeviceCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceGroupDeviceCreateInfo {
    type Type = DeviceGroupDeviceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceGroupDeviceCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_physical_devices(mut self, p_physical_devices: &'a [vk::PhysicalDevice]) -> Self {
        self.inner.physical_device_count = p_physical_devices.len() as u32;
        self.inner.p_physical_devices = p_physical_devices.as_ptr();
        self
    }
}
impl<'a> Deref for DeviceGroupDeviceCreateInfoBuilder<'a> {
    type Target = vk::DeviceGroupDeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DeviceCreateInfoNext for vk::DeviceGroupDeviceCreateInfo {}
impl SwapchainCreateInfoKHRNext for vk::DeviceGroupSwapchainCreateInfoKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    inner: vk::DescriptorUpdateTemplateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorUpdateTemplateCreateInfo {
    type Type = DescriptorUpdateTemplateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DescriptorUpdateTemplateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_descriptor_update_entries(
        mut self,
        p_descriptor_update_entries: &'a [vk::DescriptorUpdateTemplateEntry],
    ) -> Self {
        self.inner.descriptor_update_entry_count = p_descriptor_update_entries.len() as u32;
        self.inner.p_descriptor_update_entries = p_descriptor_update_entries.as_ptr();
        self
    }
    pub fn template_type(mut self, template_type: vk::DescriptorUpdateTemplateType) -> Self {
        self.inner.template_type = template_type;
        self
    }
    pub fn descriptor_set_layout(mut self, descriptor_set_layout: vk::DescriptorSetLayout) -> Self {
        self.inner.descriptor_set_layout = descriptor_set_layout;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline_layout(mut self, pipeline_layout: vk::PipelineLayout) -> Self {
        self.inner.pipeline_layout = pipeline_layout;
        self
    }
    pub fn set(mut self, set: u32) -> Self {
        self.inner.set = set;
        self
    }
}
impl<'a> Deref for DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    type Target = vk::DescriptorUpdateTemplateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentIdFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentIdFeaturesKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct PresentIdKHRBuilder<'a> {
    inner: vk::PresentIdKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PresentIdKHR {
    type Type = PresentIdKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PresentIdKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_present_ids(mut self, p_present_ids: &'a [u64]) -> Self {
        self.inner.swapchain_count = p_present_ids.len() as u32;
        self.inner.p_present_ids = p_present_ids.as_ptr();
        self
    }
}
impl<'a> Deref for PresentIdKHRBuilder<'a> {
    type Target = vk::PresentIdKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PresentInfoKHRNext for vk::PresentIdKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentId2FeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentId2FeaturesKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct PresentId2KHRBuilder<'a> {
    inner: vk::PresentId2KHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PresentId2KHR {
    type Type = PresentId2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PresentId2KHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_present_ids(mut self, p_present_ids: &'a [u64]) -> Self {
        self.inner.swapchain_count = p_present_ids.len() as u32;
        self.inner.p_present_ids = p_present_ids.as_ptr();
        self
    }
}
impl<'a> Deref for PresentId2KHRBuilder<'a> {
    type Target = vk::PresentId2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PresentInfoKHRNext for vk::PresentId2KHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentWaitFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentWaitFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentWait2FeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentWait2FeaturesKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct HdrMetadataEXTBuilder<'a> {
    inner: vk::HdrMetadataEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::HdrMetadataEXT {
    type Type = HdrMetadataEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait HdrMetadataEXTNext {}
impl<'a> HdrMetadataEXTBuilder<'a> {
    pub fn insert_next<T: HdrMetadataEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for HdrMetadataEXTBuilder<'a> {
    type Target = vk::HdrMetadataEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct HdrVividDynamicMetadataHUAWEIBuilder<'a> {
    inner: vk::HdrVividDynamicMetadataHUAWEI,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::HdrVividDynamicMetadataHUAWEI {
    type Type = HdrVividDynamicMetadataHUAWEIBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> HdrVividDynamicMetadataHUAWEIBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dynamic_metadata_size(mut self, dynamic_metadata_size: usize) -> Self {
        self.inner.dynamic_metadata_size = dynamic_metadata_size;
        self
    }
    pub fn p_dynamic_metadata(mut self, p_dynamic_metadata: *const c_void) -> Self {
        self.inner.p_dynamic_metadata = p_dynamic_metadata;
        self
    }
}
impl<'a> Deref for HdrVividDynamicMetadataHUAWEIBuilder<'a> {
    type Target = vk::HdrVividDynamicMetadataHUAWEI;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl HdrMetadataEXTNext for vk::HdrVividDynamicMetadataHUAWEI {}
impl SurfaceCapabilities2KHRNext for vk::DisplayNativeHdrSurfaceCapabilitiesAMD {}
impl SwapchainCreateInfoKHRNext for vk::SwapchainDisplayNativeHdrCreateInfoAMD {}

#[repr(transparent)]
#[derive(Default)]
pub struct PresentTimesInfoGOOGLEBuilder<'a> {
    inner: vk::PresentTimesInfoGOOGLE,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PresentTimesInfoGOOGLE {
    type Type = PresentTimesInfoGOOGLEBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PresentTimesInfoGOOGLEBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_times(mut self, p_times: &'a [vk::PresentTimeGOOGLE]) -> Self {
        self.inner.swapchain_count = p_times.len() as u32;
        self.inner.p_times = p_times.as_ptr();
        self
    }
}
impl<'a> Deref for PresentTimesInfoGOOGLEBuilder<'a> {
    type Target = vk::PresentTimesInfoGOOGLE;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PresentInfoKHRNext for vk::PresentTimesInfoGOOGLE {}

#[repr(transparent)]
#[derive(Default)]
pub struct IOSSurfaceCreateInfoMVKBuilder<'a> {
    inner: vk::IOSSurfaceCreateInfoMVK,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::IOSSurfaceCreateInfoMVK {
    type Type = IOSSurfaceCreateInfoMVKBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> IOSSurfaceCreateInfoMVKBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::IOSSurfaceCreateFlagsMVK) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_view(mut self, p_view: *const c_void) -> Self {
        self.inner.p_view = p_view;
        self
    }
}
impl<'a> Deref for IOSSurfaceCreateInfoMVKBuilder<'a> {
    type Target = vk::IOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct MacOSSurfaceCreateInfoMVKBuilder<'a> {
    inner: vk::MacOSSurfaceCreateInfoMVK,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MacOSSurfaceCreateInfoMVK {
    type Type = MacOSSurfaceCreateInfoMVKBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MacOSSurfaceCreateInfoMVKBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MacOSSurfaceCreateFlagsMVK) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_view(mut self, p_view: *const c_void) -> Self {
        self.inner.p_view = p_view;
        self
    }
}
impl<'a> Deref for MacOSSurfaceCreateInfoMVKBuilder<'a> {
    type Target = vk::MacOSSurfaceCreateInfoMVK;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct MetalSurfaceCreateInfoEXTBuilder<'a> {
    inner: vk::MetalSurfaceCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MetalSurfaceCreateInfoEXT {
    type Type = MetalSurfaceCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MetalSurfaceCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MetalSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_layer(mut self, p_layer: &'a vk::CAMetalLayer) -> Self {
        self.inner.p_layer = p_layer;
        self
    }
}
impl<'a> Deref for MetalSurfaceCreateInfoEXTBuilder<'a> {
    type Target = vk::MetalSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportWScalingStateCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineViewportWScalingStateCreateInfoNV {
    type Type = PipelineViewportWScalingStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn viewport_w_scaling_enable(mut self, viewport_w_scaling_enable: bool) -> Self {
        self.inner.viewport_w_scaling_enable = if viewport_w_scaling_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.inner.viewport_count = viewport_count;
        self
    }
    pub fn p_viewport_w_scalings(mut self, p_viewport_w_scalings: &'a [vk::ViewportWScalingNV]) -> Self {
        self.inner.viewport_count = p_viewport_w_scalings.len() as u32;
        self.inner.p_viewport_w_scalings = p_viewport_w_scalings.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportWScalingStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportWScalingStateCreateInfoNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportSwizzleStateCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineViewportSwizzleStateCreateInfoNV {
    type Type = PipelineViewportSwizzleStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineViewportSwizzleStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_viewport_swizzles(mut self, p_viewport_swizzles: &'a [vk::ViewportSwizzleNV]) -> Self {
        self.inner.viewport_count = p_viewport_swizzles.len() as u32;
        self.inner.p_viewport_swizzles = p_viewport_swizzles.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportSwizzleStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportSwizzleStateCreateInfoNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDiscardRectanglePropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineDiscardRectangleStateCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineDiscardRectangleStateCreateInfoEXT {
    type Type = PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDiscardRectangleStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn discard_rectangle_mode(mut self, discard_rectangle_mode: vk::DiscardRectangleModeEXT) -> Self {
        self.inner.discard_rectangle_mode = discard_rectangle_mode;
        self
    }
    pub fn discard_rectangle_count(mut self, discard_rectangle_count: u32) -> Self {
        self.inner.discard_rectangle_count = discard_rectangle_count;
        self
    }
    pub fn p_discard_rectangles(mut self, p_discard_rectangles: &'a [vk::Rect2D]) -> Self {
        self.inner.discard_rectangle_count = p_discard_rectangles.len() as u32;
        self.inner.p_discard_rectangles = p_discard_rectangles.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineDiscardRectangleStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for vk::PipelineDiscardRectangleStateCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    inner: vk::RenderPassInputAttachmentAspectCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassInputAttachmentAspectCreateInfo {
    type Type = RenderPassInputAttachmentAspectCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_aspect_references(mut self, p_aspect_references: &'a [vk::InputAttachmentAspectReference]) -> Self {
        self.inner.aspect_reference_count = p_aspect_references.len() as u32;
        self.inner.p_aspect_references = p_aspect_references.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    type Target = vk::RenderPassInputAttachmentAspectCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassCreateInfoNext for vk::RenderPassInputAttachmentAspectCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    inner: vk::PhysicalDeviceSurfaceInfo2KHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceSurfaceInfo2KHR {
    type Type = PhysicalDeviceSurfaceInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceSurfaceInfo2KHRNext {}
impl<'a> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    pub fn insert_next<T: PhysicalDeviceSurfaceInfo2KHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    type Target = vk::PhysicalDeviceSurfaceInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SurfaceCapabilities2KHRBuilder<'a> {
    inner: vk::SurfaceCapabilities2KHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SurfaceCapabilities2KHR {
    type Type = SurfaceCapabilities2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SurfaceCapabilities2KHRNext {}
impl<'a> SurfaceCapabilities2KHRBuilder<'a> {
    pub fn insert_next<T: SurfaceCapabilities2KHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for SurfaceCapabilities2KHRBuilder<'a> {
    type Target = vk::SurfaceCapabilities2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SurfaceFormat2KHRBuilder<'a> {
    inner: vk::SurfaceFormat2KHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SurfaceFormat2KHR {
    type Type = SurfaceFormat2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SurfaceFormat2KHRNext {}
impl<'a> SurfaceFormat2KHRBuilder<'a> {
    pub fn insert_next<T: SurfaceFormat2KHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for SurfaceFormat2KHRBuilder<'a> {
    type Target = vk::SurfaceFormat2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DisplayModeProperties2KHRBuilder<'a> {
    inner: vk::DisplayModeProperties2KHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DisplayModeProperties2KHR {
    type Type = DisplayModeProperties2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DisplayModeProperties2KHRNext {}
impl<'a> DisplayModeProperties2KHRBuilder<'a> {
    pub fn insert_next<T: DisplayModeProperties2KHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for DisplayModeProperties2KHRBuilder<'a> {
    type Target = vk::DisplayModeProperties2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DisplayModeProperties2KHRNext for vk::DisplayModeStereoPropertiesNV {}
impl SurfaceCapabilities2KHRNext for vk::SharedPresentSurfaceCapabilitiesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevice16BitStorageFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevice16BitStorageFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSubgroupProperties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceBufferMemoryRequirementsBuilder<'a> {
    inner: vk::DeviceBufferMemoryRequirements,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceBufferMemoryRequirements {
    type Type = DeviceBufferMemoryRequirementsBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceBufferMemoryRequirementsBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_create_info(mut self, p_create_info: &'a vk::BufferCreateInfo) -> Self {
        self.inner.p_create_info = p_create_info;
        self
    }
}
impl<'a> Deref for DeviceBufferMemoryRequirementsBuilder<'a> {
    type Target = vk::DeviceBufferMemoryRequirements;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageMemoryRequirementsInfo2Builder<'a> {
    inner: vk::ImageMemoryRequirementsInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageMemoryRequirementsInfo2 {
    type Type = ImageMemoryRequirementsInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageMemoryRequirementsInfo2Next {}
impl<'a> ImageMemoryRequirementsInfo2Builder<'a> {
    pub fn insert_next<T: ImageMemoryRequirementsInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for ImageMemoryRequirementsInfo2Builder<'a> {
    type Target = vk::ImageMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceImageMemoryRequirementsBuilder<'a> {
    inner: vk::DeviceImageMemoryRequirements,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceImageMemoryRequirements {
    type Type = DeviceImageMemoryRequirementsBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceImageMemoryRequirementsBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_create_info(mut self, p_create_info: &'a vk::ImageCreateInfo) -> Self {
        self.inner.p_create_info = p_create_info;
        self
    }
    pub fn plane_aspect(mut self, plane_aspect: vk::ImageAspectFlags) -> Self {
        self.inner.plane_aspect = plane_aspect;
        self
    }
}
impl<'a> Deref for DeviceImageMemoryRequirementsBuilder<'a> {
    type Target = vk::DeviceImageMemoryRequirements;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct MemoryRequirements2Builder<'a> {
    inner: vk::MemoryRequirements2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MemoryRequirements2 {
    type Type = MemoryRequirements2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait MemoryRequirements2Next {}
impl<'a> MemoryRequirements2Builder<'a> {
    pub fn insert_next<T: MemoryRequirements2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for MemoryRequirements2Builder<'a> {
    type Target = vk::MemoryRequirements2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePointClippingProperties {}
impl MemoryRequirements2Next for vk::MemoryDedicatedRequirements {}
impl MemoryAllocateInfoNext for vk::MemoryDedicatedAllocateInfo {}
impl ImageViewCreateInfoNext for vk::ImageViewUsageCreateInfo {}
impl ImageViewCreateInfoNext for vk::ImageViewSlicedCreateInfoEXT {}
impl PipelineTessellationStateCreateInfoNext for vk::PipelineTessellationDomainOriginStateCreateInfo {}
impl SamplerCreateInfoNext for vk::SamplerYcbcrConversionInfo {}
impl ImageViewCreateInfoNext for vk::SamplerYcbcrConversionInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct SamplerYcbcrConversionCreateInfoBuilder<'a> {
    inner: vk::SamplerYcbcrConversionCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SamplerYcbcrConversionCreateInfo {
    type Type = SamplerYcbcrConversionCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SamplerYcbcrConversionCreateInfoNext {}
impl<'a> SamplerYcbcrConversionCreateInfoBuilder<'a> {
    pub fn insert_next<T: SamplerYcbcrConversionCreateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    type Target = vk::SamplerYcbcrConversionCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BindImageMemoryInfoNext for vk::BindImagePlaneMemoryInfo {}
impl ImageMemoryRequirementsInfo2Next for vk::ImagePlaneMemoryRequirementsInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSamplerYcbcrConversionFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSamplerYcbcrConversionFeatures {}
impl ImageFormatProperties2Next for vk::SamplerYcbcrConversionImageFormatProperties {}
impl ImageFormatProperties2Next for vk::TextureLODGatherFormatPropertiesAMD {}
impl SubmitInfoNext for vk::ProtectedSubmitInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceProtectedMemoryFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceProtectedMemoryFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceProtectedMemoryProperties {}
impl PipelineMultisampleStateCreateInfoNext for vk::PipelineCoverageToColorStateCreateInfoNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSamplerFilterMinmaxProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct SampleLocationsInfoEXTBuilder<'a> {
    inner: vk::SampleLocationsInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SampleLocationsInfoEXT {
    type Type = SampleLocationsInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SampleLocationsInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_locations_per_pixel(mut self, sample_locations_per_pixel: vk::SampleCountFlags) -> Self {
        self.inner.sample_locations_per_pixel = sample_locations_per_pixel;
        self
    }
    pub fn sample_location_grid_size(mut self, sample_location_grid_size: vk::Extent2D) -> Self {
        self.inner.sample_location_grid_size = sample_location_grid_size;
        self
    }
    pub fn p_sample_locations(mut self, p_sample_locations: &'a [vk::SampleLocationEXT]) -> Self {
        self.inner.sample_locations_count = p_sample_locations.len() as u32;
        self.inner.p_sample_locations = p_sample_locations.as_ptr();
        self
    }
}
impl<'a> Deref for SampleLocationsInfoEXTBuilder<'a> {
    type Target = vk::SampleLocationsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageMemoryBarrierNext for vk::SampleLocationsInfoEXT {}
impl ImageMemoryBarrier2Next for vk::SampleLocationsInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    inner: vk::RenderPassSampleLocationsBeginInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassSampleLocationsBeginInfoEXT {
    type Type = RenderPassSampleLocationsBeginInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachment_initial_sample_locations(
        mut self,
        p_attachment_initial_sample_locations: &'a [vk::AttachmentSampleLocationsEXT],
    ) -> Self {
        self.inner.attachment_initial_sample_locations_count = p_attachment_initial_sample_locations.len() as u32;
        self.inner.p_attachment_initial_sample_locations = p_attachment_initial_sample_locations.as_ptr();
        self
    }
    pub fn p_post_subpass_sample_locations(
        mut self,
        p_post_subpass_sample_locations: &'a [vk::SubpassSampleLocationsEXT],
    ) -> Self {
        self.inner.post_subpass_sample_locations_count = p_post_subpass_sample_locations.len() as u32;
        self.inner.p_post_subpass_sample_locations = p_post_subpass_sample_locations.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    type Target = vk::RenderPassSampleLocationsBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassBeginInfoNext for vk::RenderPassSampleLocationsBeginInfoEXT {}
impl PipelineMultisampleStateCreateInfoNext for vk::PipelineSampleLocationsStateCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSampleLocationsPropertiesEXT {}
impl SamplerCreateInfoNext for vk::SamplerReductionModeCreateInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMultiDrawFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMultiDrawFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceBlendOperationAdvancedPropertiesEXT {}
impl PipelineColorBlendStateCreateInfoNext for vk::PipelineColorBlendAdvancedStateCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceInlineUniformBlockFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceInlineUniformBlockFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceInlineUniformBlockProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct WriteDescriptorSetInlineUniformBlockBuilder<'a> {
    inner: vk::WriteDescriptorSetInlineUniformBlock,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetInlineUniformBlock {
    type Type = WriteDescriptorSetInlineUniformBlockBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> WriteDescriptorSetInlineUniformBlockBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn data_size(mut self, data_size: u32) -> Self {
        self.inner.data_size = data_size;
        self
    }
    pub fn p_data(mut self, p_data: *const c_void) -> Self {
        self.inner.p_data = p_data;
        self
    }
}
impl<'a> Deref for WriteDescriptorSetInlineUniformBlockBuilder<'a> {
    type Target = vk::WriteDescriptorSetInlineUniformBlock;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl WriteDescriptorSetNext for vk::WriteDescriptorSetInlineUniformBlock {}
impl DescriptorPoolCreateInfoNext for vk::DescriptorPoolInlineUniformBlockCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineCoverageModulationStateCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineCoverageModulationStateCreateInfoNV {
    type Type = PipelineCoverageModulationStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageModulationStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_modulation_mode(mut self, coverage_modulation_mode: vk::CoverageModulationModeNV) -> Self {
        self.inner.coverage_modulation_mode = coverage_modulation_mode;
        self
    }
    pub fn coverage_modulation_table_enable(mut self, coverage_modulation_table_enable: bool) -> Self {
        self.inner.coverage_modulation_table_enable = if coverage_modulation_table_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn coverage_modulation_table_count(mut self, coverage_modulation_table_count: u32) -> Self {
        self.inner.coverage_modulation_table_count = coverage_modulation_table_count;
        self
    }
    pub fn p_coverage_modulation_table(mut self, p_coverage_modulation_table: &'a [f32]) -> Self {
        self.inner.coverage_modulation_table_count = p_coverage_modulation_table.len() as u32;
        self.inner.p_coverage_modulation_table = p_coverage_modulation_table.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineCoverageModulationStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineMultisampleStateCreateInfoNext for vk::PipelineCoverageModulationStateCreateInfoNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageFormatListCreateInfoBuilder<'a> {
    inner: vk::ImageFormatListCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageFormatListCreateInfo {
    type Type = ImageFormatListCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImageFormatListCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_view_formats(mut self, p_view_formats: &'a [vk::Format]) -> Self {
        self.inner.view_format_count = p_view_formats.len() as u32;
        self.inner.p_view_formats = p_view_formats.as_ptr();
        self
    }
}
impl<'a> Deref for ImageFormatListCreateInfoBuilder<'a> {
    type Target = vk::ImageFormatListCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for vk::ImageFormatListCreateInfo {}
impl SwapchainCreateInfoKHRNext for vk::ImageFormatListCreateInfo {}
impl PhysicalDeviceImageFormatInfo2Next for vk::ImageFormatListCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct ValidationCacheCreateInfoEXTBuilder<'a> {
    inner: vk::ValidationCacheCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ValidationCacheCreateInfoEXT {
    type Type = ValidationCacheCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ValidationCacheCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ValidationCacheCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn initial_data_size(mut self, initial_data_size: usize) -> Self {
        self.inner.initial_data_size = initial_data_size;
        self
    }
    pub fn p_initial_data(mut self, p_initial_data: *const c_void) -> Self {
        self.inner.p_initial_data = p_initial_data;
        self
    }
}
impl<'a> Deref for ValidationCacheCreateInfoEXTBuilder<'a> {
    type Target = vk::ValidationCacheCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ShaderModuleCreateInfoNext for vk::ShaderModuleValidationCacheCreateInfoEXT {}
impl PipelineShaderStageCreateInfoNext for vk::ShaderModuleValidationCacheCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMaintenance3Properties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMaintenance4Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMaintenance4Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMaintenance4Properties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMaintenance5Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMaintenance5Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMaintenance5Properties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMaintenance6Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMaintenance6Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMaintenance6Properties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMaintenance7FeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMaintenance7FeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMaintenance7PropertiesKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceLayeredApiPropertiesListKHRBuilder<'a> {
    inner: vk::PhysicalDeviceLayeredApiPropertiesListKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceLayeredApiPropertiesListKHR {
    type Type = PhysicalDeviceLayeredApiPropertiesListKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PhysicalDeviceLayeredApiPropertiesListKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_layered_apis(mut self, p_layered_apis: &'a mut [vk::PhysicalDeviceLayeredApiPropertiesKHR]) -> Self {
        self.inner.layered_api_count = p_layered_apis.len() as u32;
        self.inner.p_layered_apis = p_layered_apis.as_mut_ptr();
        self
    }
}
impl<'a> Deref for PhysicalDeviceLayeredApiPropertiesListKHRBuilder<'a> {
    type Target = vk::PhysicalDeviceLayeredApiPropertiesListKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceLayeredApiPropertiesListKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceLayeredApiPropertiesKHRBuilder<'a> {
    inner: vk::PhysicalDeviceLayeredApiPropertiesKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceLayeredApiPropertiesKHR {
    type Type = PhysicalDeviceLayeredApiPropertiesKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceLayeredApiPropertiesKHRNext {}
impl<'a> PhysicalDeviceLayeredApiPropertiesKHRBuilder<'a> {
    pub fn insert_next<T: PhysicalDeviceLayeredApiPropertiesKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceLayeredApiPropertiesKHRBuilder<'a> {
    type Target = vk::PhysicalDeviceLayeredApiPropertiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceLayeredApiPropertiesKHRNext for vk::PhysicalDeviceLayeredApiVulkanPropertiesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMaintenance8FeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMaintenance8FeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMaintenance9FeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMaintenance9FeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMaintenance9PropertiesKHR {}
impl QueueFamilyProperties2Next for vk::QueueFamilyOwnershipTransferPropertiesKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderingAreaInfoBuilder<'a> {
    inner: vk::RenderingAreaInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderingAreaInfo {
    type Type = RenderingAreaInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderingAreaInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn color_attachment_count(mut self, color_attachment_count: u32) -> Self {
        self.inner.color_attachment_count = color_attachment_count;
        self
    }
    pub fn p_color_attachment_formats(mut self, p_color_attachment_formats: &'a [vk::Format]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_formats.len() as u32;
        self.inner.p_color_attachment_formats = p_color_attachment_formats.as_ptr();
        self
    }
    pub fn depth_attachment_format(mut self, depth_attachment_format: vk::Format) -> Self {
        self.inner.depth_attachment_format = depth_attachment_format;
        self
    }
    pub fn stencil_attachment_format(mut self, stencil_attachment_format: vk::Format) -> Self {
        self.inner.stencil_attachment_format = stencil_attachment_format;
        self
    }
}
impl<'a> Deref for RenderingAreaInfoBuilder<'a> {
    type Target = vk::RenderingAreaInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorSetLayoutSupportBuilder<'a> {
    inner: vk::DescriptorSetLayoutSupport,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutSupport {
    type Type = DescriptorSetLayoutSupportBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorSetLayoutSupportNext {}
impl<'a> DescriptorSetLayoutSupportBuilder<'a> {
    pub fn insert_next<T: DescriptorSetLayoutSupportNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutSupportBuilder<'a> {
    type Target = vk::DescriptorSetLayoutSupport;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderDrawParametersFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderDrawParametersFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderFloat16Int8Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderFloat16Int8Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFloatControlsProperties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceHostQueryResetFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceHostQueryResetFeatures {}
impl DeviceQueueCreateInfoNext for vk::DeviceQueueGlobalPriorityCreateInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceGlobalPriorityQueryFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceGlobalPriorityQueryFeatures {}
impl QueueFamilyProperties2Next for vk::QueueFamilyGlobalPriorityProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugUtilsObjectNameInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsObjectNameInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugUtilsObjectNameInfoEXT {
    type Type = DebugUtilsObjectNameInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DebugUtilsObjectNameInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::ObjectType) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.inner.object_handle = object_handle;
        self
    }
    pub fn p_object_name(mut self, p_object_name: Option<&'a CStr>) -> Self {
        self.inner.p_object_name = p_object_name.map_or(ptr::null(), |r| r.as_ptr());
        self
    }
}
impl<'a> Deref for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    type Target = vk::DebugUtilsObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineShaderStageCreateInfoNext for vk::DebugUtilsObjectNameInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugUtilsObjectTagInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsObjectTagInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugUtilsObjectTagInfoEXT {
    type Type = DebugUtilsObjectTagInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DebugUtilsObjectTagInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn object_type(mut self, object_type: vk::ObjectType) -> Self {
        self.inner.object_type = object_type;
        self
    }
    pub fn object_handle(mut self, object_handle: u64) -> Self {
        self.inner.object_handle = object_handle;
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.inner.tag_name = tag_name;
        self
    }
    pub fn tag_size(mut self, tag_size: usize) -> Self {
        self.inner.tag_size = tag_size;
        self
    }
    pub fn p_tag(mut self, p_tag: *const c_void) -> Self {
        self.inner.p_tag = p_tag;
        self
    }
}
impl<'a> Deref for DebugUtilsObjectTagInfoEXTBuilder<'a> {
    type Target = vk::DebugUtilsObjectTagInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugUtilsLabelEXTBuilder<'a> {
    inner: vk::DebugUtilsLabelEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugUtilsLabelEXT {
    type Type = DebugUtilsLabelEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DebugUtilsLabelEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_label_name(mut self, p_label_name: &'a CStr) -> Self {
        self.inner.p_label_name = p_label_name.as_ptr();
        self
    }
    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.inner.color = color;
        self
    }
}
impl<'a> Deref for DebugUtilsLabelEXTBuilder<'a> {
    type Target = vk::DebugUtilsLabelEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsMessengerCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugUtilsMessengerCreateInfoEXT {
    type Type = DebugUtilsMessengerCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugUtilsMessengerCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn message_severity(mut self, message_severity: vk::DebugUtilsMessageSeverityFlagsEXT) -> Self {
        self.inner.message_severity = message_severity;
        self
    }
    pub fn message_type(mut self, message_type: vk::DebugUtilsMessageTypeFlagsEXT) -> Self {
        self.inner.message_type = message_type;
        self
    }
    pub fn pfn_user_callback(mut self, pfn_user_callback: vk::FnDebugUtilsMessengerCallbackEXT) -> Self {
        self.inner.pfn_user_callback = Some(pfn_user_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl<'a> Deref for DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    type Target = vk::DebugUtilsMessengerCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl InstanceCreateInfoNext for vk::DebugUtilsMessengerCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    inner: vk::DebugUtilsMessengerCallbackDataEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DebugUtilsMessengerCallbackDataEXT {
    type Type = DebugUtilsMessengerCallbackDataEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DebugUtilsMessengerCallbackDataEXTNext {}
impl<'a> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    pub fn insert_next<T: DebugUtilsMessengerCallbackDataEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DebugUtilsMessengerCallbackDataFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_message_id_name(mut self, p_message_id_name: Option<&'a CStr>) -> Self {
        self.inner.p_message_id_name = p_message_id_name.map_or(ptr::null(), |r| r.as_ptr());
        self
    }
    pub fn message_id_number(mut self, message_id_number: i32) -> Self {
        self.inner.message_id_number = message_id_number;
        self
    }
    pub fn p_message(mut self, p_message: Option<&'a CStr>) -> Self {
        self.inner.p_message = p_message.map_or(ptr::null(), |r| r.as_ptr());
        self
    }
    pub fn p_queue_labels(mut self, p_queue_labels: &'a [vk::DebugUtilsLabelEXT]) -> Self {
        self.inner.queue_label_count = p_queue_labels.len() as u32;
        self.inner.p_queue_labels = p_queue_labels.as_ptr();
        self
    }
    pub fn p_cmd_buf_labels(mut self, p_cmd_buf_labels: &'a [vk::DebugUtilsLabelEXT]) -> Self {
        self.inner.cmd_buf_label_count = p_cmd_buf_labels.len() as u32;
        self.inner.p_cmd_buf_labels = p_cmd_buf_labels.as_ptr();
        self
    }
    pub fn p_objects(mut self, p_objects: &'a [vk::DebugUtilsObjectNameInfoEXT]) -> Self {
        self.inner.object_count = p_objects.len() as u32;
        self.inner.p_objects = p_objects.as_ptr();
        self
    }
}
impl<'a> Deref for DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    type Target = vk::DebugUtilsMessengerCallbackDataEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDeviceMemoryReportFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
    inner: vk::DeviceDeviceMemoryReportCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceDeviceMemoryReportCreateInfoEXT {
    type Type = DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceMemoryReportFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pfn_user_callback(mut self, pfn_user_callback: vk::FnDeviceMemoryReportCallbackEXT) -> Self {
        self.inner.pfn_user_callback = Some(pfn_user_callback);
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl<'a> Deref for DeviceDeviceMemoryReportCreateInfoEXTBuilder<'a> {
    type Target = vk::DeviceDeviceMemoryReportCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DeviceCreateInfoNext for vk::DeviceDeviceMemoryReportCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImportMemoryHostPointerInfoEXTBuilder<'a> {
    inner: vk::ImportMemoryHostPointerInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImportMemoryHostPointerInfoEXT {
    type Type = ImportMemoryHostPointerInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImportMemoryHostPointerInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn p_host_pointer(mut self, p_host_pointer: *mut c_void) -> Self {
        self.inner.p_host_pointer = p_host_pointer;
        self
    }
}
impl<'a> Deref for ImportMemoryHostPointerInfoEXTBuilder<'a> {
    type Target = vk::ImportMemoryHostPointerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for vk::ImportMemoryHostPointerInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceExternalMemoryHostPropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceConservativeRasterizationPropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderCorePropertiesAMD {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderCoreProperties2AMD {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationConservativeStateCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDescriptorIndexingFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDescriptorIndexingFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDescriptorIndexingProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    inner: vk::DescriptorSetLayoutBindingFlagsCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutBindingFlagsCreateInfo {
    type Type = DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_binding_flags(mut self, p_binding_flags: &'a [vk::DescriptorBindingFlags]) -> Self {
        self.inner.binding_count = p_binding_flags.len() as u32;
        self.inner.p_binding_flags = p_binding_flags.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    type Target = vk::DescriptorSetLayoutBindingFlagsCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DescriptorSetLayoutCreateInfoNext for vk::DescriptorSetLayoutBindingFlagsCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    inner: vk::DescriptorSetVariableDescriptorCountAllocateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorSetVariableDescriptorCountAllocateInfo {
    type Type = DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_descriptor_counts(mut self, p_descriptor_counts: &'a [u32]) -> Self {
        self.inner.descriptor_set_count = p_descriptor_counts.len() as u32;
        self.inner.p_descriptor_counts = p_descriptor_counts.as_ptr();
        self
    }
}
impl<'a> Deref for DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    type Target = vk::DescriptorSetVariableDescriptorCountAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DescriptorSetAllocateInfoNext for vk::DescriptorSetVariableDescriptorCountAllocateInfo {}
impl DescriptorSetLayoutSupportNext for vk::DescriptorSetVariableDescriptorCountLayoutSupport {}

#[repr(transparent)]
#[derive(Default)]
pub struct AttachmentDescription2Builder<'a> {
    inner: vk::AttachmentDescription2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AttachmentDescription2 {
    type Type = AttachmentDescription2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AttachmentDescription2Next {}
impl<'a> AttachmentDescription2Builder<'a> {
    pub fn insert_next<T: AttachmentDescription2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for AttachmentDescription2Builder<'a> {
    type Target = vk::AttachmentDescription2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct AttachmentReference2Builder<'a> {
    inner: vk::AttachmentReference2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AttachmentReference2 {
    type Type = AttachmentReference2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AttachmentReference2Next {}
impl<'a> AttachmentReference2Builder<'a> {
    pub fn insert_next<T: AttachmentReference2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for AttachmentReference2Builder<'a> {
    type Target = vk::AttachmentReference2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SubpassDescription2Builder<'a> {
    inner: vk::SubpassDescription2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SubpassDescription2 {
    type Type = SubpassDescription2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubpassDescription2Next {}
impl<'a> SubpassDescription2Builder<'a> {
    pub fn insert_next<T: SubpassDescription2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SubpassDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn p_input_attachments(mut self, p_input_attachments: &'a [vk::AttachmentReference2]) -> Self {
        self.inner.input_attachment_count = p_input_attachments.len() as u32;
        self.inner.p_input_attachments = p_input_attachments.as_ptr();
        self
    }
    pub fn p_color_attachments(
        mut self,
        p_color_attachments: &'a [vk::AttachmentReference2],
        p_resolve_attachments: Option<&'a [vk::AttachmentReference2]>,
    ) -> Self {
        self.inner.color_attachment_count = p_color_attachments.len() as u32;
        self.inner.p_color_attachments = p_color_attachments.as_ptr();
        self.inner.p_resolve_attachments = p_resolve_attachments.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn p_depth_stencil_attachment(
        mut self,
        p_depth_stencil_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_depth_stencil_attachment = p_depth_stencil_attachment.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_preserve_attachments(mut self, p_preserve_attachments: &'a [u32]) -> Self {
        self.inner.preserve_attachment_count = p_preserve_attachments.len() as u32;
        self.inner.p_preserve_attachments = p_preserve_attachments.as_ptr();
        self
    }
}
impl<'a> Deref for SubpassDescription2Builder<'a> {
    type Target = vk::SubpassDescription2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SubpassDependency2Builder<'a> {
    inner: vk::SubpassDependency2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SubpassDependency2 {
    type Type = SubpassDependency2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubpassDependency2Next {}
impl<'a> SubpassDependency2Builder<'a> {
    pub fn insert_next<T: SubpassDependency2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for SubpassDependency2Builder<'a> {
    type Target = vk::SubpassDependency2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassCreateInfo2Builder<'a> {
    inner: vk::RenderPassCreateInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassCreateInfo2 {
    type Type = RenderPassCreateInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderPassCreateInfo2Next {}
impl<'a> RenderPassCreateInfo2Builder<'a> {
    pub fn insert_next<T: RenderPassCreateInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderPassCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::AttachmentDescription2]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
    pub fn p_subpasses(mut self, p_subpasses: &'a [vk::SubpassDescription2]) -> Self {
        self.inner.subpass_count = p_subpasses.len() as u32;
        self.inner.p_subpasses = p_subpasses.as_ptr();
        self
    }
    pub fn p_dependencies(mut self, p_dependencies: &'a [vk::SubpassDependency2]) -> Self {
        self.inner.dependency_count = p_dependencies.len() as u32;
        self.inner.p_dependencies = p_dependencies.as_ptr();
        self
    }
    pub fn p_correlated_view_masks(mut self, p_correlated_view_masks: &'a [u32]) -> Self {
        self.inner.correlated_view_mask_count = p_correlated_view_masks.len() as u32;
        self.inner.p_correlated_view_masks = p_correlated_view_masks.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassCreateInfo2Builder<'a> {
    type Target = vk::RenderPassCreateInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SubpassEndInfoBuilder<'a> {
    inner: vk::SubpassEndInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SubpassEndInfo {
    type Type = SubpassEndInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubpassEndInfoNext {}
impl<'a> SubpassEndInfoBuilder<'a> {
    pub fn insert_next<T: SubpassEndInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for SubpassEndInfoBuilder<'a> {
    type Target = vk::SubpassEndInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTimelineSemaphoreFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTimelineSemaphoreFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTimelineSemaphoreProperties {}
impl SemaphoreCreateInfoNext for vk::SemaphoreTypeCreateInfo {}
impl PhysicalDeviceExternalSemaphoreInfoNext for vk::SemaphoreTypeCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct TimelineSemaphoreSubmitInfoBuilder<'a> {
    inner: vk::TimelineSemaphoreSubmitInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::TimelineSemaphoreSubmitInfo {
    type Type = TimelineSemaphoreSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> TimelineSemaphoreSubmitInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_wait_semaphore_values(mut self, p_wait_semaphore_values: &'a [u64]) -> Self {
        self.inner.wait_semaphore_value_count = p_wait_semaphore_values.len() as u32;
        self.inner.p_wait_semaphore_values = p_wait_semaphore_values.as_ptr();
        self
    }
    pub fn p_signal_semaphore_values(mut self, p_signal_semaphore_values: &'a [u64]) -> Self {
        self.inner.signal_semaphore_value_count = p_signal_semaphore_values.len() as u32;
        self.inner.p_signal_semaphore_values = p_signal_semaphore_values.as_ptr();
        self
    }
}
impl<'a> Deref for TimelineSemaphoreSubmitInfoBuilder<'a> {
    type Target = vk::TimelineSemaphoreSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for vk::TimelineSemaphoreSubmitInfo {}
impl BindSparseInfoNext for vk::TimelineSemaphoreSubmitInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct SemaphoreWaitInfoBuilder<'a> {
    inner: vk::SemaphoreWaitInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SemaphoreWaitInfo {
    type Type = SemaphoreWaitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SemaphoreWaitInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreWaitFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_semaphores(mut self, p_semaphores: &'a [vk::Semaphore], p_values: &'a [u64]) -> Self {
        self.inner.semaphore_count = p_semaphores.len() as u32;
        assert_eq!(self.inner.semaphore_count, p_values.len() as u32);
        self.inner.p_semaphores = p_semaphores.as_ptr();
        self.inner.p_values = p_values.as_ptr();
        self
    }
}
impl<'a> Deref for SemaphoreWaitInfoBuilder<'a> {
    type Target = vk::SemaphoreWaitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineVertexInputDivisorStateCreateInfoBuilder<'a> {
    inner: vk::PipelineVertexInputDivisorStateCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineVertexInputDivisorStateCreateInfo {
    type Type = PipelineVertexInputDivisorStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineVertexInputDivisorStateCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_vertex_binding_divisors(
        mut self,
        p_vertex_binding_divisors: &'a [vk::VertexInputBindingDivisorDescription],
    ) -> Self {
        self.inner.vertex_binding_divisor_count = p_vertex_binding_divisors.len() as u32;
        self.inner.p_vertex_binding_divisors = p_vertex_binding_divisors.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineVertexInputDivisorStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineVertexInputDivisorStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineVertexInputStateCreateInfoNext for vk::PipelineVertexInputDivisorStateCreateInfo {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVertexAttributeDivisorPropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVertexAttributeDivisorProperties {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePCIBusInfoPropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    inner: vk::ImportAndroidHardwareBufferInfoANDROID,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImportAndroidHardwareBufferInfoANDROID {
    type Type = ImportAndroidHardwareBufferInfoANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: *mut vk::AHardwareBuffer) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl<'a> Deref for ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    type Target = vk::ImportAndroidHardwareBufferInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for vk::ImportAndroidHardwareBufferInfoANDROID {}
impl ImageFormatProperties2Next for vk::AndroidHardwareBufferUsageANDROID {}

#[repr(transparent)]
#[derive(Default)]
pub struct AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    inner: vk::AndroidHardwareBufferPropertiesANDROID,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AndroidHardwareBufferPropertiesANDROID {
    type Type = AndroidHardwareBufferPropertiesANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AndroidHardwareBufferPropertiesANDROIDNext {}
impl<'a> AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    pub fn insert_next<T: AndroidHardwareBufferPropertiesANDROIDNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for AndroidHardwareBufferPropertiesANDROIDBuilder<'a> {
    type Target = vk::AndroidHardwareBufferPropertiesANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AndroidHardwareBufferPropertiesANDROIDNext for vk::AndroidHardwareBufferFormatPropertiesANDROID {}
impl CommandBufferInheritanceInfoNext for vk::CommandBufferInheritanceConditionalRenderingInfoEXT {}
impl ImageCreateInfoNext for vk::ExternalFormatANDROID {}
impl SamplerYcbcrConversionCreateInfoNext for vk::ExternalFormatANDROID {}
impl AttachmentDescription2Next for vk::ExternalFormatANDROID {}
impl GraphicsPipelineCreateInfoNext for vk::ExternalFormatANDROID {}
impl CommandBufferInheritanceInfoNext for vk::ExternalFormatANDROID {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevice8BitStorageFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevice8BitStorageFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceConditionalRenderingFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceConditionalRenderingFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkanMemoryModelFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkanMemoryModelFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderAtomicInt64Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderAtomicInt64Features {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderAtomicFloatFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderAtomicFloat2FeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVertexAttributeDivisorFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVertexAttributeDivisorFeatures {}
impl QueueFamilyProperties2Next for vk::QueueFamilyCheckpointPropertiesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDepthStencilResolveProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct SubpassDescriptionDepthStencilResolveBuilder<'a> {
    inner: vk::SubpassDescriptionDepthStencilResolve,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SubpassDescriptionDepthStencilResolve {
    type Type = SubpassDescriptionDepthStencilResolveBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SubpassDescriptionDepthStencilResolveBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn depth_resolve_mode(mut self, depth_resolve_mode: vk::ResolveModeFlags) -> Self {
        self.inner.depth_resolve_mode = depth_resolve_mode;
        self
    }
    pub fn stencil_resolve_mode(mut self, stencil_resolve_mode: vk::ResolveModeFlags) -> Self {
        self.inner.stencil_resolve_mode = stencil_resolve_mode;
        self
    }
    pub fn p_depth_stencil_resolve_attachment(
        mut self,
        p_depth_stencil_resolve_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_depth_stencil_resolve_attachment = p_depth_stencil_resolve_attachment.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for SubpassDescriptionDepthStencilResolveBuilder<'a> {
    type Target = vk::SubpassDescriptionDepthStencilResolve;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubpassDescription2Next for vk::SubpassDescriptionDepthStencilResolve {}
impl ImageViewCreateInfoNext for vk::ImageViewASTCDecodeModeEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceASTCDecodeFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceASTCDecodeFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTransformFeedbackFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTransformFeedbackFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTransformFeedbackPropertiesEXT {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationStateStreamCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineRepresentativeFragmentTestStateCreateInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExclusiveScissorFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExclusiveScissorFeaturesNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportExclusiveScissorStateCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineViewportExclusiveScissorStateCreateInfoNV {
    type Type = PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn exclusive_scissor_count(mut self, exclusive_scissor_count: u32) -> Self {
        self.inner.exclusive_scissor_count = exclusive_scissor_count;
        self
    }
    pub fn p_exclusive_scissors(mut self, p_exclusive_scissors: &'a [vk::Rect2D]) -> Self {
        self.inner.exclusive_scissor_count = p_exclusive_scissors.len() as u32;
        self.inner.p_exclusive_scissors = p_exclusive_scissors.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportExclusiveScissorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportExclusiveScissorStateCreateInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCornerSampledImageFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCornerSampledImageFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceComputeShaderDerivativesFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceComputeShaderDerivativesFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceComputeShaderDerivativesPropertiesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderImageFootprintFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderImageFootprintFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCopyMemoryIndirectFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCopyMemoryIndirectFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCopyMemoryIndirectPropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMemoryDecompressionFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMemoryDecompressionFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMemoryDecompressionPropertiesNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct ShadingRatePaletteNVBuilder<'a> {
    inner: vk::ShadingRatePaletteNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ShadingRatePaletteNV {
    type Type = ShadingRatePaletteNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ShadingRatePaletteNVBuilder<'a> {
    pub fn p_shading_rate_palette_entries(
        mut self,
        p_shading_rate_palette_entries: &'a [vk::ShadingRatePaletteEntryNV],
    ) -> Self {
        self.inner.shading_rate_palette_entry_count = p_shading_rate_palette_entries.len() as u32;
        self.inner.p_shading_rate_palette_entries = p_shading_rate_palette_entries.as_ptr();
        self
    }
}
impl<'a> Deref for ShadingRatePaletteNVBuilder<'a> {
    type Target = vk::ShadingRatePaletteNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportShadingRateImageStateCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineViewportShadingRateImageStateCreateInfoNV {
    type Type = PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_image_enable(mut self, shading_rate_image_enable: bool) -> Self {
        self.inner.shading_rate_image_enable = if shading_rate_image_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn viewport_count(mut self, viewport_count: u32) -> Self {
        self.inner.viewport_count = viewport_count;
        self
    }
    pub fn p_shading_rate_palettes(mut self, p_shading_rate_palettes: &'a [vk::ShadingRatePaletteNV]) -> Self {
        self.inner.viewport_count = p_shading_rate_palettes.len() as u32;
        self.inner.p_shading_rate_palettes = p_shading_rate_palettes.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportShadingRateImageStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportShadingRateImageStateCreateInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShadingRateImageFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShadingRateImageFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShadingRateImagePropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceInvocationMaskFeaturesHUAWEI {}

#[repr(transparent)]
#[derive(Default)]
pub struct CoarseSampleOrderCustomNVBuilder<'a> {
    inner: vk::CoarseSampleOrderCustomNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CoarseSampleOrderCustomNV {
    type Type = CoarseSampleOrderCustomNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CoarseSampleOrderCustomNVBuilder<'a> {
    pub fn shading_rate(mut self, shading_rate: vk::ShadingRatePaletteEntryNV) -> Self {
        self.inner.shading_rate = shading_rate;
        self
    }
    pub fn sample_count(mut self, sample_count: u32) -> Self {
        self.inner.sample_count = sample_count;
        self
    }
    pub fn p_sample_locations(mut self, p_sample_locations: &'a [vk::CoarseSampleLocationNV]) -> Self {
        self.inner.sample_location_count = p_sample_locations.len() as u32;
        self.inner.p_sample_locations = p_sample_locations.as_ptr();
        self
    }
}
impl<'a> Deref for CoarseSampleOrderCustomNVBuilder<'a> {
    type Target = vk::CoarseSampleOrderCustomNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    type Type = PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_order_type(mut self, sample_order_type: vk::CoarseSampleOrderTypeNV) -> Self {
        self.inner.sample_order_type = sample_order_type;
        self
    }
    pub fn p_custom_sample_orders(mut self, p_custom_sample_orders: &'a [vk::CoarseSampleOrderCustomNV]) -> Self {
        self.inner.custom_sample_order_count = p_custom_sample_orders.len() as u32;
        self.inner.p_custom_sample_orders = p_custom_sample_orders.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMeshShaderFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMeshShaderFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMeshShaderPropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMeshShaderFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMeshShaderFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMeshShaderPropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    inner: vk::RayTracingShaderGroupCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RayTracingShaderGroupCreateInfoKHR {
    type Type = RayTracingShaderGroupCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::RayTracingShaderGroupTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn general_shader(mut self, general_shader: u32) -> Self {
        self.inner.general_shader = general_shader;
        self
    }
    pub fn closest_hit_shader(mut self, closest_hit_shader: u32) -> Self {
        self.inner.closest_hit_shader = closest_hit_shader;
        self
    }
    pub fn any_hit_shader(mut self, any_hit_shader: u32) -> Self {
        self.inner.any_hit_shader = any_hit_shader;
        self
    }
    pub fn intersection_shader(mut self, intersection_shader: u32) -> Self {
        self.inner.intersection_shader = intersection_shader;
        self
    }
    pub fn p_shader_group_capture_replay_handle(mut self, p_shader_group_capture_replay_handle: *const c_void) -> Self {
        self.inner.p_shader_group_capture_replay_handle = p_shader_group_capture_replay_handle;
        self
    }
}
impl<'a> Deref for RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    type Target = vk::RayTracingShaderGroupCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct RayTracingPipelineCreateInfoNVBuilder<'a> {
    inner: vk::RayTracingPipelineCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RayTracingPipelineCreateInfoNV {
    type Type = RayTracingPipelineCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RayTracingPipelineCreateInfoNVNext {}
impl<'a> RayTracingPipelineCreateInfoNVBuilder<'a> {
    pub fn insert_next<T: RayTracingPipelineCreateInfoNVNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::RayTracingShaderGroupCreateInfoNV]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.as_ptr();
        self
    }
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.inner.max_recursion_depth = max_recursion_depth;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: vk::Pipeline) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for RayTracingPipelineCreateInfoNVBuilder<'a> {
    type Target = vk::RayTracingPipelineCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct RayTracingPipelineCreateInfoKHRBuilder<'a> {
    inner: vk::RayTracingPipelineCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RayTracingPipelineCreateInfoKHR {
    type Type = RayTracingPipelineCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RayTracingPipelineCreateInfoKHRNext {}
impl<'a> RayTracingPipelineCreateInfoKHRBuilder<'a> {
    pub fn insert_next<T: RayTracingPipelineCreateInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_groups(mut self, p_groups: &'a [vk::RayTracingShaderGroupCreateInfoKHR]) -> Self {
        self.inner.group_count = p_groups.len() as u32;
        self.inner.p_groups = p_groups.as_ptr();
        self
    }
    pub fn max_pipeline_ray_recursion_depth(mut self, max_pipeline_ray_recursion_depth: u32) -> Self {
        self.inner.max_pipeline_ray_recursion_depth = max_pipeline_ray_recursion_depth;
        self
    }
    pub fn p_library_info(mut self, p_library_info: Option<&'a vk::PipelineLibraryCreateInfoKHR>) -> Self {
        self.inner.p_library_info = p_library_info.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_library_interface(
        mut self,
        p_library_interface: Option<&'a vk::RayTracingPipelineInterfaceCreateInfoKHR>,
    ) -> Self {
        self.inner.p_library_interface = p_library_interface.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_dynamic_state(mut self, p_dynamic_state: Option<&'a vk::PipelineDynamicStateCreateInfo>) -> Self {
        self.inner.p_dynamic_state = p_dynamic_state.map_or(ptr::null(), |r| r);
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: vk::Pipeline) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for RayTracingPipelineCreateInfoKHRBuilder<'a> {
    type Target = vk::RayTracingPipelineCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureInfoNVBuilder<'a> {
    inner: vk::AccelerationStructureInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureInfoNV {
    type Type = AccelerationStructureInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AccelerationStructureInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureTypeNV) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn flags(mut self, flags: vk::BuildAccelerationStructureFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn instance_count(mut self, instance_count: u32) -> Self {
        self.inner.instance_count = instance_count;
        self
    }
    pub fn p_geometries(mut self, p_geometries: &'a [vk::GeometryNV]) -> Self {
        self.inner.geometry_count = p_geometries.len() as u32;
        self.inner.p_geometries = p_geometries.as_ptr();
        self
    }
}
impl<'a> Deref for AccelerationStructureInfoNVBuilder<'a> {
    type Target = vk::AccelerationStructureInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureCreateInfoNVBuilder<'a> {
    inner: vk::AccelerationStructureCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureCreateInfoNV {
    type Type = AccelerationStructureCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AccelerationStructureCreateInfoNVNext {}
impl<'a> AccelerationStructureCreateInfoNVBuilder<'a> {
    pub fn insert_next<T: AccelerationStructureCreateInfoNVNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for AccelerationStructureCreateInfoNVBuilder<'a> {
    type Target = vk::AccelerationStructureCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    inner: vk::BindAccelerationStructureMemoryInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindAccelerationStructureMemoryInfoNV {
    type Type = BindAccelerationStructureMemoryInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureNV) -> Self {
        self.inner.acceleration_structure = acceleration_structure;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = memory;
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
    pub fn p_device_indices(mut self, p_device_indices: &'a [u32]) -> Self {
        self.inner.device_index_count = p_device_indices.len() as u32;
        self.inner.p_device_indices = p_device_indices.as_ptr();
        self
    }
}
impl<'a> Deref for BindAccelerationStructureMemoryInfoNVBuilder<'a> {
    type Target = vk::BindAccelerationStructureMemoryInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    inner: vk::WriteDescriptorSetAccelerationStructureKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetAccelerationStructureKHR {
    type Type = WriteDescriptorSetAccelerationStructureKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acceleration_structures(mut self, p_acceleration_structures: &'a [vk::AccelerationStructureKHR]) -> Self {
        self.inner.acceleration_structure_count = p_acceleration_structures.len() as u32;
        self.inner.p_acceleration_structures = p_acceleration_structures.as_ptr();
        self
    }
}
impl<'a> Deref for WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    type Target = vk::WriteDescriptorSetAccelerationStructureKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl WriteDescriptorSetNext for vk::WriteDescriptorSetAccelerationStructureKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    inner: vk::WriteDescriptorSetAccelerationStructureNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetAccelerationStructureNV {
    type Type = WriteDescriptorSetAccelerationStructureNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acceleration_structures(mut self, p_acceleration_structures: &'a [vk::AccelerationStructureNV]) -> Self {
        self.inner.acceleration_structure_count = p_acceleration_structures.len() as u32;
        self.inner.p_acceleration_structures = p_acceleration_structures.as_ptr();
        self
    }
}
impl<'a> Deref for WriteDescriptorSetAccelerationStructureNVBuilder<'a> {
    type Target = vk::WriteDescriptorSetAccelerationStructureNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl WriteDescriptorSetNext for vk::WriteDescriptorSetAccelerationStructureNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceAccelerationStructureFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceAccelerationStructureFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingPipelineFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingPipelineFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayQueryFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayQueryFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceAccelerationStructurePropertiesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceRayTracingPipelinePropertiesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceRayTracingPropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingMaintenance1FeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingMaintenance1FeaturesKHR {}
impl FormatProperties2Next for vk::DrmFormatModifierPropertiesListEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    inner: vk::PhysicalDeviceImageDrmFormatModifierInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceImageDrmFormatModifierInfoEXT {
    type Type = PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.inner.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn queue_family_index_count(mut self, queue_family_index_count: u32) -> Self {
        self.inner.queue_family_index_count = queue_family_index_count;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
}
impl<'a> Deref for PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceImageDrmFormatModifierInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceImageFormatInfo2Next for vk::PhysicalDeviceImageDrmFormatModifierInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    inner: vk::ImageDrmFormatModifierListCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageDrmFormatModifierListCreateInfoEXT {
    type Type = ImageDrmFormatModifierListCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_drm_format_modifiers(mut self, p_drm_format_modifiers: &'a [u64]) -> Self {
        self.inner.drm_format_modifier_count = p_drm_format_modifiers.len() as u32;
        self.inner.p_drm_format_modifiers = p_drm_format_modifiers.as_ptr();
        self
    }
}
impl<'a> Deref for ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    type Target = vk::ImageDrmFormatModifierListCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for vk::ImageDrmFormatModifierListCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    inner: vk::ImageDrmFormatModifierExplicitCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageDrmFormatModifierExplicitCreateInfoEXT {
    type Type = ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn drm_format_modifier(mut self, drm_format_modifier: u64) -> Self {
        self.inner.drm_format_modifier = drm_format_modifier;
        self
    }
    pub fn p_plane_layouts(mut self, p_plane_layouts: &'a [vk::SubresourceLayout]) -> Self {
        self.inner.drm_format_modifier_plane_count = p_plane_layouts.len() as u32;
        self.inner.p_plane_layouts = p_plane_layouts.as_ptr();
        self
    }
}
impl<'a> Deref for ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    type Target = vk::ImageDrmFormatModifierExplicitCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for vk::ImageDrmFormatModifierExplicitCreateInfoEXT {}
impl ImageCreateInfoNext for vk::ImageStencilUsageCreateInfo {}
impl PhysicalDeviceImageFormatInfo2Next for vk::ImageStencilUsageCreateInfo {}
impl DeviceCreateInfoNext for vk::DeviceMemoryOverallocationCreateInfoAMD {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentDensityMapFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentDensityMapFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentDensityMap2FeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentDensityMapPropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentDensityMap2PropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT {}
impl RenderPassCreateInfoNext for vk::RenderPassFragmentDensityMapCreateInfoEXT {}
impl RenderPassCreateInfo2Next for vk::RenderPassFragmentDensityMapCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassFragmentDensityMapOffsetEndInfoEXTBuilder<'a> {
    inner: vk::RenderPassFragmentDensityMapOffsetEndInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassFragmentDensityMapOffsetEndInfoEXT {
    type Type = RenderPassFragmentDensityMapOffsetEndInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassFragmentDensityMapOffsetEndInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_fragment_density_offsets(mut self, p_fragment_density_offsets: &'a [vk::Offset2D]) -> Self {
        self.inner.fragment_density_offset_count = p_fragment_density_offsets.len() as u32;
        self.inner.p_fragment_density_offsets = p_fragment_density_offsets.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassFragmentDensityMapOffsetEndInfoEXTBuilder<'a> {
    type Target = vk::RenderPassFragmentDensityMapOffsetEndInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubpassEndInfoNext for vk::RenderPassFragmentDensityMapOffsetEndInfoEXT {}
impl RenderingEndInfoEXTNext for vk::RenderPassFragmentDensityMapOffsetEndInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceScalarBlockLayoutFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceScalarBlockLayoutFeatures {}
impl SurfaceCapabilities2KHRNext for vk::SurfaceProtectedCapabilitiesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceUniformBufferStandardLayoutFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceUniformBufferStandardLayoutFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDepthClipEnableFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDepthClipEnableFeaturesEXT {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationDepthClipStateCreateInfoEXT {}
impl PhysicalDeviceMemoryProperties2Next for vk::PhysicalDeviceMemoryBudgetPropertiesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMemoryPriorityFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMemoryPriorityFeaturesEXT {}
impl MemoryAllocateInfoNext for vk::MemoryPriorityAllocateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceBufferDeviceAddressFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceBufferDeviceAddressFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {}
impl BufferCreateInfoNext for vk::BufferOpaqueCaptureAddressCreateInfo {}
impl BufferCreateInfoNext for vk::BufferDeviceAddressCreateInfoEXT {}
impl PhysicalDeviceImageFormatInfo2Next for vk::PhysicalDeviceImageViewImageFormatInfoEXT {}
impl ImageFormatProperties2Next for vk::FilterCubicImageViewImageFormatPropertiesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImagelessFramebufferFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImagelessFramebufferFeatures {}

#[repr(transparent)]
#[derive(Default)]
pub struct FramebufferAttachmentsCreateInfoBuilder<'a> {
    inner: vk::FramebufferAttachmentsCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::FramebufferAttachmentsCreateInfo {
    type Type = FramebufferAttachmentsCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> FramebufferAttachmentsCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachment_image_infos(
        mut self,
        p_attachment_image_infos: &'a [vk::FramebufferAttachmentImageInfo],
    ) -> Self {
        self.inner.attachment_image_info_count = p_attachment_image_infos.len() as u32;
        self.inner.p_attachment_image_infos = p_attachment_image_infos.as_ptr();
        self
    }
}
impl<'a> Deref for FramebufferAttachmentsCreateInfoBuilder<'a> {
    type Target = vk::FramebufferAttachmentsCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl FramebufferCreateInfoNext for vk::FramebufferAttachmentsCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct FramebufferAttachmentImageInfoBuilder<'a> {
    inner: vk::FramebufferAttachmentImageInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::FramebufferAttachmentImageInfo {
    type Type = FramebufferAttachmentImageInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> FramebufferAttachmentImageInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn width(mut self, width: u32) -> Self {
        self.inner.width = width;
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.inner.height = height;
        self
    }
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.inner.layer_count = layer_count;
        self
    }
    pub fn p_view_formats(mut self, p_view_formats: &'a [vk::Format]) -> Self {
        self.inner.view_format_count = p_view_formats.len() as u32;
        self.inner.p_view_formats = p_view_formats.as_ptr();
        self
    }
}
impl<'a> Deref for FramebufferAttachmentImageInfoBuilder<'a> {
    type Target = vk::FramebufferAttachmentImageInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassAttachmentBeginInfoBuilder<'a> {
    inner: vk::RenderPassAttachmentBeginInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassAttachmentBeginInfo {
    type Type = RenderPassAttachmentBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassAttachmentBeginInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attachments(mut self, p_attachments: &'a [vk::ImageView]) -> Self {
        self.inner.attachment_count = p_attachments.len() as u32;
        self.inner.p_attachments = p_attachments.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassAttachmentBeginInfoBuilder<'a> {
    type Target = vk::RenderPassAttachmentBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassBeginInfoNext for vk::RenderPassAttachmentBeginInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTextureCompressionASTCHDRFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTextureCompressionASTCHDRFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCooperativeMatrixFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCooperativeMatrixFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCooperativeMatrixPropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineCreationFeedbackCreateInfoBuilder<'a> {
    inner: vk::PipelineCreationFeedbackCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineCreationFeedbackCreateInfo {
    type Type = PipelineCreationFeedbackCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineCreationFeedbackCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_pipeline_creation_feedback(
        mut self,
        p_pipeline_creation_feedback: *mut vk::PipelineCreationFeedback,
    ) -> Self {
        self.inner.p_pipeline_creation_feedback = p_pipeline_creation_feedback;
        self
    }
    pub fn p_pipeline_stage_creation_feedbacks(
        mut self,
        p_pipeline_stage_creation_feedbacks: &'a mut [vk::PipelineCreationFeedback],
    ) -> Self {
        self.inner.pipeline_stage_creation_feedback_count = p_pipeline_stage_creation_feedbacks.len() as u32;
        self.inner.p_pipeline_stage_creation_feedbacks = p_pipeline_stage_creation_feedbacks.as_mut_ptr();
        self
    }
}
impl<'a> Deref for PipelineCreationFeedbackCreateInfoBuilder<'a> {
    type Target = vk::PipelineCreationFeedbackCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for vk::PipelineCreationFeedbackCreateInfo {}
impl ComputePipelineCreateInfoNext for vk::PipelineCreationFeedbackCreateInfo {}
impl RayTracingPipelineCreateInfoNVNext for vk::PipelineCreationFeedbackCreateInfo {}
impl RayTracingPipelineCreateInfoKHRNext for vk::PipelineCreationFeedbackCreateInfo {}
impl ExecutionGraphPipelineCreateInfoAMDXNext for vk::PipelineCreationFeedbackCreateInfo {}
impl PhysicalDeviceSurfaceInfo2KHRNext for vk::SurfaceFullScreenExclusiveInfoEXT {}
impl SwapchainCreateInfoKHRNext for vk::SurfaceFullScreenExclusiveInfoEXT {}
impl PhysicalDeviceSurfaceInfo2KHRNext for vk::SurfaceFullScreenExclusiveWin32InfoEXT {}
impl SwapchainCreateInfoKHRNext for vk::SurfaceFullScreenExclusiveWin32InfoEXT {}
impl SurfaceCapabilities2KHRNext for vk::SurfaceCapabilitiesFullScreenExclusiveEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentBarrierFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentBarrierFeaturesNV {}
impl SurfaceCapabilities2KHRNext for vk::SurfaceCapabilitiesPresentBarrierNV {}
impl SwapchainCreateInfoKHRNext for vk::SwapchainPresentBarrierCreateInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePerformanceQueryFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePerformanceQueryFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePerformanceQueryPropertiesKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    inner: vk::QueryPoolPerformanceCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::QueryPoolPerformanceCreateInfoKHR {
    type Type = QueryPoolPerformanceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
    pub fn p_counter_indices(mut self, p_counter_indices: &'a [u32]) -> Self {
        self.inner.counter_index_count = p_counter_indices.len() as u32;
        self.inner.p_counter_indices = p_counter_indices.as_ptr();
        self
    }
}
impl<'a> Deref for QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    type Target = vk::QueryPoolPerformanceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl QueryPoolCreateInfoNext for vk::QueryPoolPerformanceCreateInfoKHR {}
impl SubmitInfoNext for vk::PerformanceQuerySubmitInfoKHR {}
impl SubmitInfo2Next for vk::PerformanceQuerySubmitInfoKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCoverageReductionModeFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCoverageReductionModeFeaturesNV {}
impl PipelineMultisampleStateCreateInfoNext for vk::PipelineCoverageReductionStateCreateInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {}

#[repr(transparent)]
#[derive(Default)]
pub struct InitializePerformanceApiInfoINTELBuilder<'a> {
    inner: vk::InitializePerformanceApiInfoINTEL,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::InitializePerformanceApiInfoINTEL {
    type Type = InitializePerformanceApiInfoINTELBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> InitializePerformanceApiInfoINTELBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_user_data(mut self, p_user_data: *mut c_void) -> Self {
        self.inner.p_user_data = p_user_data;
        self
    }
}
impl<'a> Deref for InitializePerformanceApiInfoINTELBuilder<'a> {
    type Target = vk::InitializePerformanceApiInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl QueryPoolCreateInfoNext for vk::QueryPoolPerformanceQueryCreateInfoINTEL {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderClockFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderClockFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceIndexTypeUint8Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceIndexTypeUint8Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderSMBuiltinsPropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {}
impl AttachmentReference2Next for vk::AttachmentReferenceStencilLayout {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {}
impl AttachmentDescription2Next for vk::AttachmentDescriptionStencilLayout {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderDemoteToHelperInvocationFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTexelBufferAlignmentProperties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSubgroupSizeControlFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSubgroupSizeControlFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSubgroupSizeControlProperties {}
impl PipelineShaderStageCreateInfoNext for vk::PipelineShaderStageRequiredSubgroupSizeCreateInfo {}
impl ShaderCreateInfoEXTNext for vk::PipelineShaderStageRequiredSubgroupSizeCreateInfo {}
impl ComputePipelineCreateInfoNext for vk::SubpassShadingPipelineCreateInfoHUAWEI {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSubpassShadingPropertiesHUAWEI {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceClusterCullingShaderPropertiesHUAWEI {}
impl MemoryAllocateInfoNext for vk::MemoryOpaqueCaptureAddressAllocateInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceLineRasterizationFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceLineRasterizationFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceLineRasterizationProperties {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationLineStateCreateInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineCreationCacheControlFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineCreationCacheControlFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkan11Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkan11Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVulkan11Properties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkan12Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkan12Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVulkan12Properties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkan13Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkan13Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVulkan13Properties {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVulkan14Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVulkan14Features {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceVulkan14Properties {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineCompilerControlCreateInfoAMD {}
impl ComputePipelineCreateInfoNext for vk::PipelineCompilerControlCreateInfoAMD {}
impl ExecutionGraphPipelineCreateInfoAMDXNext for vk::PipelineCompilerControlCreateInfoAMD {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCoherentMemoryFeaturesAMD {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCoherentMemoryFeaturesAMD {}
impl SamplerCreateInfoNext for vk::SamplerCustomBorderColorCreateInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCustomBorderColorPropertiesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCustomBorderColorFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCustomBorderColorFeaturesEXT {}
impl SamplerCreateInfoNext for vk::SamplerBorderColorComponentMappingCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceBorderColorSwizzleFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    inner: vk::AccelerationStructureGeometryTrianglesDataKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureGeometryTrianglesDataKHR {
    type Type = AccelerationStructureGeometryTrianglesDataKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AccelerationStructureGeometryTrianglesDataKHRNext {}
impl<'a> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    pub fn insert_next<T: AccelerationStructureGeometryTrianglesDataKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    type Target = vk::AccelerationStructureGeometryTrianglesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AccelerationStructureGeometryKHRNext for vk::AccelerationStructureGeometryLinearSweptSpheresDataNV {}
impl AccelerationStructureGeometryKHRNext for vk::AccelerationStructureGeometrySpheresDataNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureGeometryKHRBuilder<'a> {
    inner: vk::AccelerationStructureGeometryKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureGeometryKHR {
    type Type = AccelerationStructureGeometryKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AccelerationStructureGeometryKHRNext {}
impl<'a> AccelerationStructureGeometryKHRBuilder<'a> {
    pub fn insert_next<T: AccelerationStructureGeometryKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for AccelerationStructureGeometryKHRBuilder<'a> {
    type Target = vk::AccelerationStructureGeometryKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureBuildGeometryInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureBuildGeometryInfoKHR {
    type Type = AccelerationStructureBuildGeometryInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn flags(mut self, flags: vk::BuildAccelerationStructureFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn mode(mut self, mode: vk::BuildAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
    pub fn src_acceleration_structure(mut self, src_acceleration_structure: vk::AccelerationStructureKHR) -> Self {
        self.inner.src_acceleration_structure = src_acceleration_structure;
        self
    }
    pub fn dst_acceleration_structure(mut self, dst_acceleration_structure: vk::AccelerationStructureKHR) -> Self {
        self.inner.dst_acceleration_structure = dst_acceleration_structure;
        self
    }
    pub fn p_geometries(
        mut self,
        p_geometries: Option<&'a [vk::AccelerationStructureGeometryKHR]>,
        pp_geometries: Option<&'a [*const vk::AccelerationStructureGeometryKHR]>,
    ) -> Self {
        self.inner.geometry_count = p_geometries
            .map(|s| s.len() as u32)
            .or(pp_geometries.map(|s| s.len() as u32))
            .unwrap_or(0);
        if let Some(len) = p_geometries.map(|s| s.len()) {
            assert_eq!(self.inner.geometry_count, len as u32);
        }
        if let Some(len) = pp_geometries.map(|s| s.len()) {
            assert_eq!(self.inner.geometry_count, len as u32);
        }
        self.inner.p_geometries = p_geometries.map_or(ptr::null(), |s| s.as_ptr());
        self.inner.pp_geometries = pp_geometries.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn scratch_data(mut self, scratch_data: vk::DeviceOrHostAddressKHR) -> Self {
        self.inner.scratch_data = scratch_data;
        self
    }
}
impl<'a> Deref for AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureBuildGeometryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureCreateInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureCreateInfoKHR {
    type Type = AccelerationStructureCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait AccelerationStructureCreateInfoKHRNext {}
impl<'a> AccelerationStructureCreateInfoKHRBuilder<'a> {
    pub fn insert_next<T: AccelerationStructureCreateInfoKHRNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for AccelerationStructureCreateInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureVersionInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureVersionInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureVersionInfoKHR {
    type Type = AccelerationStructureVersionInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AccelerationStructureVersionInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_version_data(mut self, p_version_data: *const u8) -> Self {
        self.inner.p_version_data = p_version_data;
        self
    }
}
impl<'a> Deref for AccelerationStructureVersionInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureVersionInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineLibraryCreateInfoKHRBuilder<'a> {
    inner: vk::PipelineLibraryCreateInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineLibraryCreateInfoKHR {
    type Type = PipelineLibraryCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineLibraryCreateInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_libraries(mut self, p_libraries: &'a [vk::Pipeline]) -> Self {
        self.inner.library_count = p_libraries.len() as u32;
        self.inner.p_libraries = p_libraries.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineLibraryCreateInfoKHRBuilder<'a> {
    type Target = vk::PipelineLibraryCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for vk::PipelineLibraryCreateInfoKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExtendedDynamicState2FeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExtendedDynamicState3FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExtendedDynamicState3FeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceExtendedDynamicState3PropertiesEXT {}
impl RenderPassBeginInfoNext for vk::RenderPassTransformBeginInfoQCOM {}
impl BufferImageCopy2Next for vk::CopyCommandTransformInfoQCOM {}
impl ImageBlit2Next for vk::CopyCommandTransformInfoQCOM {}
impl CommandBufferInheritanceInfoNext for vk::CommandBufferInheritanceRenderPassTransformInfoQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePartitionedAccelerationStructureFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePartitionedAccelerationStructureFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePartitionedAccelerationStructurePropertiesNV {}
impl PartitionedAccelerationStructureInstancesInputNVNext for vk::PartitionedAccelerationStructureFlagsNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct WriteDescriptorSetPartitionedAccelerationStructureNVBuilder<'a> {
    inner: vk::WriteDescriptorSetPartitionedAccelerationStructureNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetPartitionedAccelerationStructureNV {
    type Type = WriteDescriptorSetPartitionedAccelerationStructureNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> WriteDescriptorSetPartitionedAccelerationStructureNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_acceleration_structures(mut self, p_acceleration_structures: &'a [vk::DeviceAddress]) -> Self {
        self.inner.acceleration_structure_count = p_acceleration_structures.len() as u32;
        self.inner.p_acceleration_structures = p_acceleration_structures.as_ptr();
        self
    }
}
impl<'a> Deref for WriteDescriptorSetPartitionedAccelerationStructureNVBuilder<'a> {
    type Target = vk::WriteDescriptorSetPartitionedAccelerationStructureNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl WriteDescriptorSetNext for vk::WriteDescriptorSetPartitionedAccelerationStructureNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct PartitionedAccelerationStructureInstancesInputNVBuilder<'a> {
    inner: vk::PartitionedAccelerationStructureInstancesInputNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PartitionedAccelerationStructureInstancesInputNV {
    type Type = PartitionedAccelerationStructureInstancesInputNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PartitionedAccelerationStructureInstancesInputNVNext {}
impl<'a> PartitionedAccelerationStructureInstancesInputNVBuilder<'a> {
    pub fn insert_next<T: PartitionedAccelerationStructureInstancesInputNVNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PartitionedAccelerationStructureInstancesInputNVBuilder<'a> {
    type Target = vk::PartitionedAccelerationStructureInstancesInputNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDiagnosticsConfigFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDiagnosticsConfigFeaturesNV {}
impl DeviceCreateInfoNext for vk::DeviceDiagnosticsConfigCreateInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRobustness2FeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRobustness2FeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceRobustness2PropertiesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageRobustnessFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageRobustnessFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePortabilitySubsetFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePortabilitySubsetFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePortabilitySubsetPropertiesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevice4444FormatsFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevice4444FormatsFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSubpassShadingFeaturesHUAWEI {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceClusterCullingShaderFeaturesHUAWEIBuilder<'a> {
    inner: vk::PhysicalDeviceClusterCullingShaderFeaturesHUAWEI,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
    type Type = PhysicalDeviceClusterCullingShaderFeaturesHUAWEIBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PhysicalDeviceClusterCullingShaderFeaturesHUAWEINext {}
impl<'a> PhysicalDeviceClusterCullingShaderFeaturesHUAWEIBuilder<'a> {
    pub fn insert_next<T: PhysicalDeviceClusterCullingShaderFeaturesHUAWEINext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for PhysicalDeviceClusterCullingShaderFeaturesHUAWEIBuilder<'a> {
    type Target = vk::PhysicalDeviceClusterCullingShaderFeaturesHUAWEI;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceClusterCullingShaderFeaturesHUAWEI {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceClusterCullingShaderFeaturesHUAWEI {}
impl PhysicalDeviceClusterCullingShaderFeaturesHUAWEINext for vk::PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageBlit2Builder<'a> {
    inner: vk::ImageBlit2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageBlit2 {
    type Type = ImageBlit2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageBlit2Next {}
impl<'a> ImageBlit2Builder<'a> {
    pub fn insert_next<T: ImageBlit2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for ImageBlit2Builder<'a> {
    type Target = vk::ImageBlit2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BufferImageCopy2Builder<'a> {
    inner: vk::BufferImageCopy2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BufferImageCopy2 {
    type Type = BufferImageCopy2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BufferImageCopy2Next {}
impl<'a> BufferImageCopy2Builder<'a> {
    pub fn insert_next<T: BufferImageCopy2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for BufferImageCopy2Builder<'a> {
    type Target = vk::BufferImageCopy2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CopyBufferInfo2Builder<'a> {
    inner: vk::CopyBufferInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CopyBufferInfo2 {
    type Type = CopyBufferInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CopyBufferInfo2Builder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_buffer(mut self, src_buffer: vk::Buffer) -> Self {
        self.inner.src_buffer = src_buffer;
        self
    }
    pub fn dst_buffer(mut self, dst_buffer: vk::Buffer) -> Self {
        self.inner.dst_buffer = dst_buffer;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyBufferInfo2Builder<'a> {
    type Target = vk::CopyBufferInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CopyImageInfo2Builder<'a> {
    inner: vk::CopyImageInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CopyImageInfo2 {
    type Type = CopyImageInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CopyImageInfo2Builder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = src_image;
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = dst_image;
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyImageInfo2Builder<'a> {
    type Target = vk::CopyImageInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BlitImageInfo2Builder<'a> {
    inner: vk::BlitImageInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BlitImageInfo2 {
    type Type = BlitImageInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BlitImageInfo2Next {}
impl<'a> BlitImageInfo2Builder<'a> {
    pub fn insert_next<T: BlitImageInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = src_image;
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = dst_image;
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageBlit2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
    pub fn filter(mut self, filter: vk::Filter) -> Self {
        self.inner.filter = filter;
        self
    }
}
impl<'a> Deref for BlitImageInfo2Builder<'a> {
    type Target = vk::BlitImageInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CopyBufferToImageInfo2Builder<'a> {
    inner: vk::CopyBufferToImageInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CopyBufferToImageInfo2 {
    type Type = CopyBufferToImageInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CopyBufferToImageInfo2Builder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_buffer(mut self, src_buffer: vk::Buffer) -> Self {
        self.inner.src_buffer = src_buffer;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = dst_image;
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferImageCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyBufferToImageInfo2Builder<'a> {
    type Target = vk::CopyBufferToImageInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CopyImageToBufferInfo2Builder<'a> {
    inner: vk::CopyImageToBufferInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CopyImageToBufferInfo2 {
    type Type = CopyImageToBufferInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CopyImageToBufferInfo2Builder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = src_image;
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_buffer(mut self, dst_buffer: vk::Buffer) -> Self {
        self.inner.dst_buffer = dst_buffer;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::BufferImageCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyImageToBufferInfo2Builder<'a> {
    type Target = vk::CopyImageToBufferInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ResolveImageInfo2Builder<'a> {
    inner: vk::ResolveImageInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ResolveImageInfo2 {
    type Type = ResolveImageInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ResolveImageInfo2Builder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = src_image;
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = dst_image;
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageResolve2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for ResolveImageInfo2Builder<'a> {
    type Target = vk::ResolveImageInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderImageAtomicInt64FeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    inner: vk::FragmentShadingRateAttachmentInfoKHR,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::FragmentShadingRateAttachmentInfoKHR {
    type Type = FragmentShadingRateAttachmentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_fragment_shading_rate_attachment(
        mut self,
        p_fragment_shading_rate_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_fragment_shading_rate_attachment = p_fragment_shading_rate_attachment.map_or(ptr::null(), |r| r);
        self
    }
    pub fn shading_rate_attachment_texel_size(mut self, shading_rate_attachment_texel_size: vk::Extent2D) -> Self {
        self.inner.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
        self
    }
}
impl<'a> Deref for FragmentShadingRateAttachmentInfoKHRBuilder<'a> {
    type Target = vk::FragmentShadingRateAttachmentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubpassDescription2Next for vk::FragmentShadingRateAttachmentInfoKHR {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineFragmentShadingRateStateCreateInfoKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentShadingRateFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentShadingRateFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentShadingRatePropertiesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderTerminateInvocationFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderTerminateInvocationFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentShadingRateEnumsFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentShadingRateEnumsPropertiesNV {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineFragmentShadingRateEnumStateCreateInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImage2DViewOf3DFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImage2DViewOf3DFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageSlicedViewOf3DFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageSlicedViewOf3DFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceLegacyVertexAttributesFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceLegacyVertexAttributesFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceLegacyVertexAttributesPropertiesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMutableDescriptorTypeFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMutableDescriptorTypeFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct MutableDescriptorTypeListEXTBuilder<'a> {
    inner: vk::MutableDescriptorTypeListEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MutableDescriptorTypeListEXT {
    type Type = MutableDescriptorTypeListEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MutableDescriptorTypeListEXTBuilder<'a> {
    pub fn p_descriptor_types(mut self, p_descriptor_types: &'a [vk::DescriptorType]) -> Self {
        self.inner.descriptor_type_count = p_descriptor_types.len() as u32;
        self.inner.p_descriptor_types = p_descriptor_types.as_ptr();
        self
    }
}
impl<'a> Deref for MutableDescriptorTypeListEXTBuilder<'a> {
    type Target = vk::MutableDescriptorTypeListEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct MutableDescriptorTypeCreateInfoEXTBuilder<'a> {
    inner: vk::MutableDescriptorTypeCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MutableDescriptorTypeCreateInfoEXT {
    type Type = MutableDescriptorTypeCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MutableDescriptorTypeCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_mutable_descriptor_type_lists(
        mut self,
        p_mutable_descriptor_type_lists: &'a [vk::MutableDescriptorTypeListEXT],
    ) -> Self {
        self.inner.mutable_descriptor_type_list_count = p_mutable_descriptor_type_lists.len() as u32;
        self.inner.p_mutable_descriptor_type_lists = p_mutable_descriptor_type_lists.as_ptr();
        self
    }
}
impl<'a> Deref for MutableDescriptorTypeCreateInfoEXTBuilder<'a> {
    type Target = vk::MutableDescriptorTypeCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DescriptorSetLayoutCreateInfoNext for vk::MutableDescriptorTypeCreateInfoEXT {}
impl DescriptorPoolCreateInfoNext for vk::MutableDescriptorTypeCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDepthClipControlFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDepthClipControlFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT {}
impl GeneratedCommandsInfoEXTNext for vk::GeneratedCommandsPipelineInfoEXT {}
impl GeneratedCommandsMemoryRequirementsInfoEXTNext for vk::GeneratedCommandsPipelineInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct GeneratedCommandsShaderInfoEXTBuilder<'a> {
    inner: vk::GeneratedCommandsShaderInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::GeneratedCommandsShaderInfoEXT {
    type Type = GeneratedCommandsShaderInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> GeneratedCommandsShaderInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_shaders(mut self, p_shaders: &'a [vk::ShaderEXT]) -> Self {
        self.inner.shader_count = p_shaders.len() as u32;
        self.inner.p_shaders = p_shaders.as_ptr();
        self
    }
}
impl<'a> Deref for GeneratedCommandsShaderInfoEXTBuilder<'a> {
    type Target = vk::GeneratedCommandsShaderInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GeneratedCommandsInfoEXTNext for vk::GeneratedCommandsShaderInfoEXT {}
impl GeneratedCommandsMemoryRequirementsInfoEXTNext for vk::GeneratedCommandsShaderInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct GeneratedCommandsMemoryRequirementsInfoEXTBuilder<'a> {
    inner: vk::GeneratedCommandsMemoryRequirementsInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::GeneratedCommandsMemoryRequirementsInfoEXT {
    type Type = GeneratedCommandsMemoryRequirementsInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait GeneratedCommandsMemoryRequirementsInfoEXTNext {}
impl<'a> GeneratedCommandsMemoryRequirementsInfoEXTBuilder<'a> {
    pub fn insert_next<T: GeneratedCommandsMemoryRequirementsInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for GeneratedCommandsMemoryRequirementsInfoEXTBuilder<'a> {
    type Target = vk::GeneratedCommandsMemoryRequirementsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct IndirectExecutionSetShaderLayoutInfoEXTBuilder<'a> {
    inner: vk::IndirectExecutionSetShaderLayoutInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::IndirectExecutionSetShaderLayoutInfoEXT {
    type Type = IndirectExecutionSetShaderLayoutInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> IndirectExecutionSetShaderLayoutInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_set_layouts(mut self, p_set_layouts: &'a [vk::DescriptorSetLayout]) -> Self {
        self.inner.set_layout_count = p_set_layouts.len() as u32;
        self.inner.p_set_layouts = p_set_layouts.as_ptr();
        self
    }
}
impl<'a> Deref for IndirectExecutionSetShaderLayoutInfoEXTBuilder<'a> {
    type Target = vk::IndirectExecutionSetShaderLayoutInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct IndirectExecutionSetShaderInfoEXTBuilder<'a> {
    inner: vk::IndirectExecutionSetShaderInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::IndirectExecutionSetShaderInfoEXT {
    type Type = IndirectExecutionSetShaderInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> IndirectExecutionSetShaderInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_initial_shaders(
        mut self,
        p_initial_shaders: &'a [vk::ShaderEXT],
        p_set_layout_infos: Option<&'a [vk::IndirectExecutionSetShaderLayoutInfoEXT]>,
    ) -> Self {
        self.inner.shader_count = p_initial_shaders.len() as u32;
        self.inner.p_initial_shaders = p_initial_shaders.as_ptr();
        self.inner.p_set_layout_infos = p_set_layout_infos.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn max_shader_count(mut self, max_shader_count: u32) -> Self {
        self.inner.max_shader_count = max_shader_count;
        self
    }
    pub fn p_push_constant_ranges(mut self, p_push_constant_ranges: &'a [vk::PushConstantRange]) -> Self {
        self.inner.push_constant_range_count = p_push_constant_ranges.len() as u32;
        self.inner.p_push_constant_ranges = p_push_constant_ranges.as_ptr();
        self
    }
}
impl<'a> Deref for IndirectExecutionSetShaderInfoEXTBuilder<'a> {
    type Target = vk::IndirectExecutionSetShaderInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct GeneratedCommandsInfoEXTBuilder<'a> {
    inner: vk::GeneratedCommandsInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::GeneratedCommandsInfoEXT {
    type Type = GeneratedCommandsInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait GeneratedCommandsInfoEXTNext {}
impl<'a> GeneratedCommandsInfoEXTBuilder<'a> {
    pub fn insert_next<T: GeneratedCommandsInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for GeneratedCommandsInfoEXTBuilder<'a> {
    type Target = vk::GeneratedCommandsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct IndirectCommandsLayoutCreateInfoEXTBuilder<'a> {
    inner: vk::IndirectCommandsLayoutCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutCreateInfoEXT {
    type Type = IndirectCommandsLayoutCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait IndirectCommandsLayoutCreateInfoEXTNext {}
impl<'a> IndirectCommandsLayoutCreateInfoEXTBuilder<'a> {
    pub fn insert_next<T: IndirectCommandsLayoutCreateInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::IndirectCommandsLayoutUsageFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn shader_stages(mut self, shader_stages: vk::ShaderStageFlags) -> Self {
        self.inner.shader_stages = shader_stages;
        self
    }
    pub fn indirect_stride(mut self, indirect_stride: u32) -> Self {
        self.inner.indirect_stride = indirect_stride;
        self
    }
    pub fn pipeline_layout(mut self, pipeline_layout: vk::PipelineLayout) -> Self {
        self.inner.pipeline_layout = pipeline_layout;
        self
    }
    pub fn p_tokens(mut self, p_tokens: &'a [vk::IndirectCommandsLayoutTokenEXT]) -> Self {
        self.inner.token_count = p_tokens.len() as u32;
        self.inner.p_tokens = p_tokens.as_ptr();
        self
    }
}
impl<'a> Deref for IndirectCommandsLayoutCreateInfoEXTBuilder<'a> {
    type Target = vk::IndirectCommandsLayoutCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportDepthClipControlCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDepthClampControlFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDepthClampControlFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineViewportDepthClampControlCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineViewportDepthClampControlCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineViewportDepthClampControlCreateInfoEXT {
    type Type = PipelineViewportDepthClampControlCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineViewportDepthClampControlCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn depth_clamp_mode(mut self, depth_clamp_mode: vk::DepthClampModeEXT) -> Self {
        self.inner.depth_clamp_mode = depth_clamp_mode;
        self
    }
    pub fn p_depth_clamp_range(mut self, p_depth_clamp_range: Option<&'a vk::DepthClampRangeEXT>) -> Self {
        self.inner.p_depth_clamp_range = p_depth_clamp_range.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for PipelineViewportDepthClampControlCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineViewportDepthClampControlCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineViewportStateCreateInfoNext for vk::PipelineViewportDepthClampControlCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVertexInputDynamicStateFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExternalMemoryRDMAFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceColorWriteEnableFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceColorWriteEnableFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineColorWriteCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineColorWriteCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineColorWriteCreateInfoEXT {
    type Type = PipelineColorWriteCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineColorWriteCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_color_write_enables(mut self, p_color_write_enables: &'a [vk::Bool32]) -> Self {
        self.inner.attachment_count = p_color_write_enables.len() as u32;
        self.inner.p_color_write_enables = p_color_write_enables.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineColorWriteCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineColorWriteCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineColorBlendStateCreateInfoNext for vk::PipelineColorWriteCreateInfoEXT {}
impl SubpassDependency2Next for vk::MemoryBarrier2 {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageMemoryBarrier2Builder<'a> {
    inner: vk::ImageMemoryBarrier2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageMemoryBarrier2 {
    type Type = ImageMemoryBarrier2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ImageMemoryBarrier2Next {}
impl<'a> ImageMemoryBarrier2Builder<'a> {
    pub fn insert_next<T: ImageMemoryBarrier2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for ImageMemoryBarrier2Builder<'a> {
    type Target = vk::ImageMemoryBarrier2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BufferMemoryBarrier2Builder<'a> {
    inner: vk::BufferMemoryBarrier2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BufferMemoryBarrier2 {
    type Type = BufferMemoryBarrier2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BufferMemoryBarrier2Next {}
impl<'a> BufferMemoryBarrier2Builder<'a> {
    pub fn insert_next<T: BufferMemoryBarrier2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for BufferMemoryBarrier2Builder<'a> {
    type Target = vk::BufferMemoryBarrier2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubpassDependency2Next for vk::MemoryBarrierAccessFlags3KHR {}
impl BufferMemoryBarrier2Next for vk::MemoryBarrierAccessFlags3KHR {}
impl ImageMemoryBarrier2Next for vk::MemoryBarrierAccessFlags3KHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct DependencyInfoBuilder<'a> {
    inner: vk::DependencyInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DependencyInfo {
    type Type = DependencyInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DependencyInfoNext {}
impl<'a> DependencyInfoBuilder<'a> {
    pub fn insert_next<T: DependencyInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dependency_flags(mut self, dependency_flags: vk::DependencyFlags) -> Self {
        self.inner.dependency_flags = dependency_flags;
        self
    }
    pub fn p_memory_barriers(mut self, p_memory_barriers: &'a [vk::MemoryBarrier2]) -> Self {
        self.inner.memory_barrier_count = p_memory_barriers.len() as u32;
        self.inner.p_memory_barriers = p_memory_barriers.as_ptr();
        self
    }
    pub fn p_buffer_memory_barriers(mut self, p_buffer_memory_barriers: &'a [vk::BufferMemoryBarrier2]) -> Self {
        self.inner.buffer_memory_barrier_count = p_buffer_memory_barriers.len() as u32;
        self.inner.p_buffer_memory_barriers = p_buffer_memory_barriers.as_ptr();
        self
    }
    pub fn p_image_memory_barriers(mut self, p_image_memory_barriers: &'a [vk::ImageMemoryBarrier2]) -> Self {
        self.inner.image_memory_barrier_count = p_image_memory_barriers.len() as u32;
        self.inner.p_image_memory_barriers = p_image_memory_barriers.as_ptr();
        self
    }
}
impl<'a> Deref for DependencyInfoBuilder<'a> {
    type Target = vk::DependencyInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CommandBufferSubmitInfoBuilder<'a> {
    inner: vk::CommandBufferSubmitInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CommandBufferSubmitInfo {
    type Type = CommandBufferSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait CommandBufferSubmitInfoNext {}
impl<'a> CommandBufferSubmitInfoBuilder<'a> {
    pub fn insert_next<T: CommandBufferSubmitInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for CommandBufferSubmitInfoBuilder<'a> {
    type Target = vk::CommandBufferSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SubmitInfo2Builder<'a> {
    inner: vk::SubmitInfo2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SubmitInfo2 {
    type Type = SubmitInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubmitInfo2Next {}
impl<'a> SubmitInfo2Builder<'a> {
    pub fn insert_next<T: SubmitInfo2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SubmitFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_wait_semaphore_infos(mut self, p_wait_semaphore_infos: &'a [vk::SemaphoreSubmitInfo]) -> Self {
        self.inner.wait_semaphore_info_count = p_wait_semaphore_infos.len() as u32;
        self.inner.p_wait_semaphore_infos = p_wait_semaphore_infos.as_ptr();
        self
    }
    pub fn p_command_buffer_infos(mut self, p_command_buffer_infos: &'a [vk::CommandBufferSubmitInfo]) -> Self {
        self.inner.command_buffer_info_count = p_command_buffer_infos.len() as u32;
        self.inner.p_command_buffer_infos = p_command_buffer_infos.as_ptr();
        self
    }
    pub fn p_signal_semaphore_infos(mut self, p_signal_semaphore_infos: &'a [vk::SemaphoreSubmitInfo]) -> Self {
        self.inner.signal_semaphore_info_count = p_signal_semaphore_infos.len() as u32;
        self.inner.p_signal_semaphore_infos = p_signal_semaphore_infos.as_ptr();
        self
    }
}
impl<'a> Deref for SubmitInfo2Builder<'a> {
    type Target = vk::SubmitInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl QueueFamilyProperties2Next for vk::QueueFamilyCheckpointProperties2NV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSynchronization2Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSynchronization2Features {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceUnifiedImageLayoutsFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceUnifiedImageLayoutsFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceHostImageCopyFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceHostImageCopyFeatures {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceHostImageCopyPropertiesBuilder<'a> {
    inner: vk::PhysicalDeviceHostImageCopyProperties,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceHostImageCopyProperties {
    type Type = PhysicalDeviceHostImageCopyPropertiesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PhysicalDeviceHostImageCopyPropertiesBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_copy_src_layouts(mut self, p_copy_src_layouts: &'a mut [vk::ImageLayout]) -> Self {
        self.inner.copy_src_layout_count = p_copy_src_layouts.len() as u32;
        self.inner.p_copy_src_layouts = p_copy_src_layouts.as_mut_ptr();
        self
    }
    pub fn p_copy_dst_layouts(mut self, p_copy_dst_layouts: &'a mut [vk::ImageLayout]) -> Self {
        self.inner.copy_dst_layout_count = p_copy_dst_layouts.len() as u32;
        self.inner.p_copy_dst_layouts = p_copy_dst_layouts.as_mut_ptr();
        self
    }
    pub fn optimal_tiling_layout_uuid(mut self, optimal_tiling_layout_uuid: [u8; vk::UUID_SIZE]) -> Self {
        self.inner.optimal_tiling_layout_uuid = optimal_tiling_layout_uuid;
        self
    }
    pub fn identical_memory_type_requirements(mut self, identical_memory_type_requirements: bool) -> Self {
        self.inner.identical_memory_type_requirements = if identical_memory_type_requirements {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceHostImageCopyPropertiesBuilder<'a> {
    type Target = vk::PhysicalDeviceHostImageCopyProperties;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceHostImageCopyProperties {}

#[repr(transparent)]
#[derive(Default)]
pub struct MemoryToImageCopyBuilder<'a> {
    inner: vk::MemoryToImageCopy,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MemoryToImageCopy {
    type Type = MemoryToImageCopyBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MemoryToImageCopyBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_host_pointer(mut self, p_host_pointer: *const c_void) -> Self {
        self.inner.p_host_pointer = p_host_pointer;
        self
    }
    pub fn memory_row_length(mut self, memory_row_length: u32) -> Self {
        self.inner.memory_row_length = memory_row_length;
        self
    }
    pub fn memory_image_height(mut self, memory_image_height: u32) -> Self {
        self.inner.memory_image_height = memory_image_height;
        self
    }
    pub fn image_subresource(mut self, image_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.image_subresource = image_subresource;
        self
    }
    pub fn image_offset(mut self, image_offset: vk::Offset3D) -> Self {
        self.inner.image_offset = image_offset;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent3D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
}
impl<'a> Deref for MemoryToImageCopyBuilder<'a> {
    type Target = vk::MemoryToImageCopy;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageToMemoryCopyBuilder<'a> {
    inner: vk::ImageToMemoryCopy,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageToMemoryCopy {
    type Type = ImageToMemoryCopyBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImageToMemoryCopyBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_host_pointer(mut self, p_host_pointer: *mut c_void) -> Self {
        self.inner.p_host_pointer = p_host_pointer;
        self
    }
    pub fn memory_row_length(mut self, memory_row_length: u32) -> Self {
        self.inner.memory_row_length = memory_row_length;
        self
    }
    pub fn memory_image_height(mut self, memory_image_height: u32) -> Self {
        self.inner.memory_image_height = memory_image_height;
        self
    }
    pub fn image_subresource(mut self, image_subresource: vk::ImageSubresourceLayers) -> Self {
        self.inner.image_subresource = image_subresource;
        self
    }
    pub fn image_offset(mut self, image_offset: vk::Offset3D) -> Self {
        self.inner.image_offset = image_offset;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent3D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
}
impl<'a> Deref for ImageToMemoryCopyBuilder<'a> {
    type Target = vk::ImageToMemoryCopy;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CopyMemoryToImageInfoBuilder<'a> {
    inner: vk::CopyMemoryToImageInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CopyMemoryToImageInfo {
    type Type = CopyMemoryToImageInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CopyMemoryToImageInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::HostImageCopyFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = dst_image;
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::MemoryToImageCopy]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyMemoryToImageInfoBuilder<'a> {
    type Target = vk::CopyMemoryToImageInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CopyImageToMemoryInfoBuilder<'a> {
    inner: vk::CopyImageToMemoryInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CopyImageToMemoryInfo {
    type Type = CopyImageToMemoryInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CopyImageToMemoryInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::HostImageCopyFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = src_image;
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageToMemoryCopy]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyImageToMemoryInfoBuilder<'a> {
    type Target = vk::CopyImageToMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CopyImageToImageInfoBuilder<'a> {
    inner: vk::CopyImageToImageInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CopyImageToImageInfo {
    type Type = CopyImageToImageInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CopyImageToImageInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::HostImageCopyFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn src_image(mut self, src_image: vk::Image) -> Self {
        self.inner.src_image = src_image;
        self
    }
    pub fn src_image_layout(mut self, src_image_layout: vk::ImageLayout) -> Self {
        self.inner.src_image_layout = src_image_layout;
        self
    }
    pub fn dst_image(mut self, dst_image: vk::Image) -> Self {
        self.inner.dst_image = dst_image;
        self
    }
    pub fn dst_image_layout(mut self, dst_image_layout: vk::ImageLayout) -> Self {
        self.inner.dst_image_layout = dst_image_layout;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::ImageCopy2]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyImageToImageInfoBuilder<'a> {
    type Target = vk::CopyImageToImageInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubresourceLayout2Next for vk::SubresourceHostMemcpySize {}
impl ImageFormatProperties2Next for vk::HostImageCopyDevicePerformanceQuery {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceLegacyDitheringFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceLegacyDitheringFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {}
impl SurfaceCapabilities2KHRNext for vk::SurfaceCapabilitiesPresentId2KHR {}
impl SurfaceCapabilities2KHRNext for vk::SurfaceCapabilitiesPresentWait2KHR {}
impl FormatProperties2Next for vk::SubpassResolvePerformanceQueryEXT {}
impl SubpassDescription2Next for vk::MultisampledRenderToSingleSampledInfoEXT {}
impl RenderingInfoNext for vk::MultisampledRenderToSingleSampledInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineProtectedAccessFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineProtectedAccessFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceInheritedViewportScissorFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceInheritedViewportScissorFeaturesNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    inner: vk::CommandBufferInheritanceViewportScissorInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceViewportScissorInfoNV {
    type Type = CommandBufferInheritanceViewportScissorInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn viewport_scissor_2d(mut self, viewport_scissor_2d: bool) -> Self {
        self.inner.viewport_scissor_2d = if viewport_scissor_2d { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn viewport_depth_count(mut self, viewport_depth_count: u32) -> Self {
        self.inner.viewport_depth_count = viewport_depth_count;
        self
    }
    pub fn p_viewport_depths(mut self, p_viewport_depths: &'a vk::Viewport) -> Self {
        self.inner.p_viewport_depths = p_viewport_depths;
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceViewportScissorInfoNVBuilder<'a> {
    type Target = vk::CommandBufferInheritanceViewportScissorInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CommandBufferInheritanceInfoNext for vk::CommandBufferInheritanceViewportScissorInfoNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceProvokingVertexFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceProvokingVertexFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceProvokingVertexPropertiesEXT {}
impl PipelineRasterizationStateCreateInfoNext for vk::PipelineRasterizationProvokingVertexStateCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct CuModuleCreateInfoNVXBuilder<'a> {
    inner: vk::CuModuleCreateInfoNVX,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CuModuleCreateInfoNVX {
    type Type = CuModuleCreateInfoNVXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait CuModuleCreateInfoNVXNext {}
impl<'a> CuModuleCreateInfoNVXBuilder<'a> {
    pub fn insert_next<T: CuModuleCreateInfoNVXNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn data_size(mut self, data_size: usize) -> Self {
        self.inner.data_size = data_size;
        self
    }
    pub fn p_data(mut self, p_data: *const c_void) -> Self {
        self.inner.p_data = p_data;
        self
    }
}
impl<'a> Deref for CuModuleCreateInfoNVXBuilder<'a> {
    type Target = vk::CuModuleCreateInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CuModuleCreateInfoNVXNext for vk::CuModuleTexturingModeCreateInfoNVX {}

#[repr(transparent)]
#[derive(Default)]
pub struct CuFunctionCreateInfoNVXBuilder<'a> {
    inner: vk::CuFunctionCreateInfoNVX,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CuFunctionCreateInfoNVX {
    type Type = CuFunctionCreateInfoNVXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CuFunctionCreateInfoNVXBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn module(mut self, module: vk::CuModuleNVX) -> Self {
        self.inner.module = module;
        self
    }
    pub fn p_name(mut self, p_name: &'a CStr) -> Self {
        self.inner.p_name = p_name.as_ptr();
        self
    }
}
impl<'a> Deref for CuFunctionCreateInfoNVXBuilder<'a> {
    type Target = vk::CuFunctionCreateInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CuLaunchInfoNVXBuilder<'a> {
    inner: vk::CuLaunchInfoNVX,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CuLaunchInfoNVX {
    type Type = CuLaunchInfoNVXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CuLaunchInfoNVXBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn function(mut self, function: vk::CuFunctionNVX) -> Self {
        self.inner.function = function;
        self
    }
    pub fn grid_dim_x(mut self, grid_dim_x: u32) -> Self {
        self.inner.grid_dim_x = grid_dim_x;
        self
    }
    pub fn grid_dim_y(mut self, grid_dim_y: u32) -> Self {
        self.inner.grid_dim_y = grid_dim_y;
        self
    }
    pub fn grid_dim_z(mut self, grid_dim_z: u32) -> Self {
        self.inner.grid_dim_z = grid_dim_z;
        self
    }
    pub fn block_dim_x(mut self, block_dim_x: u32) -> Self {
        self.inner.block_dim_x = block_dim_x;
        self
    }
    pub fn block_dim_y(mut self, block_dim_y: u32) -> Self {
        self.inner.block_dim_y = block_dim_y;
        self
    }
    pub fn block_dim_z(mut self, block_dim_z: u32) -> Self {
        self.inner.block_dim_z = block_dim_z;
        self
    }
    pub fn shared_mem_bytes(mut self, shared_mem_bytes: u32) -> Self {
        self.inner.shared_mem_bytes = shared_mem_bytes;
        self
    }
    pub fn p_params(mut self, p_params: &'a [*const c_void]) -> Self {
        self.inner.param_count = p_params.len();
        self.inner.p_params = p_params.as_ptr();
        self
    }
    pub fn p_extras(mut self, p_extras: &'a [*const c_void]) -> Self {
        self.inner.extra_count = p_extras.len();
        self.inner.p_extras = p_extras.as_ptr();
        self
    }
}
impl<'a> Deref for CuLaunchInfoNVXBuilder<'a> {
    type Target = vk::CuLaunchInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDescriptorBufferFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDescriptorBufferFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDescriptorBufferPropertiesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorBufferBindingInfoEXTBuilder<'a> {
    inner: vk::DescriptorBufferBindingInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorBufferBindingInfoEXT {
    type Type = DescriptorBufferBindingInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorBufferBindingInfoEXTNext {}
impl<'a> DescriptorBufferBindingInfoEXTBuilder<'a> {
    pub fn insert_next<T: DescriptorBufferBindingInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for DescriptorBufferBindingInfoEXTBuilder<'a> {
    type Target = vk::DescriptorBufferBindingInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DescriptorBufferBindingInfoEXTNext for vk::DescriptorBufferBindingPushDescriptorBufferHandleEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct DescriptorGetInfoEXTBuilder<'a> {
    inner: vk::DescriptorGetInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DescriptorGetInfoEXT {
    type Type = DescriptorGetInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DescriptorGetInfoEXTNext {}
impl<'a> DescriptorGetInfoEXTBuilder<'a> {
    pub fn insert_next<T: DescriptorGetInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for DescriptorGetInfoEXTBuilder<'a> {
    type Target = vk::DescriptorGetInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct OpaqueCaptureDescriptorDataCreateInfoEXTBuilder<'a> {
    inner: vk::OpaqueCaptureDescriptorDataCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {
    type Type = OpaqueCaptureDescriptorDataCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> OpaqueCaptureDescriptorDataCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn opaque_capture_descriptor_data(mut self, opaque_capture_descriptor_data: *const c_void) -> Self {
        self.inner.opaque_capture_descriptor_data = opaque_capture_descriptor_data;
        self
    }
}
impl<'a> Deref for OpaqueCaptureDescriptorDataCreateInfoEXTBuilder<'a> {
    type Target = vk::OpaqueCaptureDescriptorDataCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BufferCreateInfoNext for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {}
impl ImageCreateInfoNext for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {}
impl ImageViewCreateInfoNext for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {}
impl SamplerCreateInfoNext for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {}
impl AccelerationStructureCreateInfoKHRNext for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {}
impl AccelerationStructureCreateInfoNVNext for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {}
impl TensorCreateInfoARMNext for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {}
impl TensorViewCreateInfoARMNext for vk::OpaqueCaptureDescriptorDataCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderIntegerDotProductFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderIntegerDotProductFeatures {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderIntegerDotProductProperties {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDrmPropertiesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentShaderBarycentricFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentShaderBarycentricPropertiesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingMotionBlurFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingValidationFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingValidationFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV {}
impl AccelerationStructureGeometryTrianglesDataKHRNext for vk::AccelerationStructureGeometryMotionTrianglesDataNV {}
impl AccelerationStructureCreateInfoKHRNext for vk::AccelerationStructureMotionInfoNV {}
impl MemoryAllocateInfoNext for vk::ImportMemoryBufferCollectionFUCHSIA {}
impl ImageCreateInfoNext for vk::BufferCollectionImageCreateInfoFUCHSIA {}
impl BufferCreateInfoNext for vk::BufferCollectionBufferCreateInfoFUCHSIA {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    inner: vk::ImageFormatConstraintsInfoFUCHSIA,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageFormatConstraintsInfoFUCHSIA {
    type Type = ImageFormatConstraintsInfoFUCHSIABuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_create_info(mut self, image_create_info: vk::ImageCreateInfo) -> Self {
        self.inner.image_create_info = image_create_info;
        self
    }
    pub fn required_format_features(mut self, required_format_features: vk::FormatFeatureFlags) -> Self {
        self.inner.required_format_features = required_format_features;
        self
    }
    pub fn flags(mut self, flags: vk::ImageFormatConstraintsFlagsFUCHSIA) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn sysmem_pixel_format(mut self, sysmem_pixel_format: u64) -> Self {
        self.inner.sysmem_pixel_format = sysmem_pixel_format;
        self
    }
    pub fn p_color_spaces(mut self, p_color_spaces: &'a [vk::SysmemColorSpaceFUCHSIA]) -> Self {
        self.inner.color_space_count = p_color_spaces.len() as u32;
        self.inner.p_color_spaces = p_color_spaces.as_ptr();
        self
    }
}
impl<'a> Deref for ImageFormatConstraintsInfoFUCHSIABuilder<'a> {
    type Target = vk::ImageFormatConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageConstraintsInfoFUCHSIABuilder<'a> {
    inner: vk::ImageConstraintsInfoFUCHSIA,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageConstraintsInfoFUCHSIA {
    type Type = ImageConstraintsInfoFUCHSIABuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImageConstraintsInfoFUCHSIABuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_format_constraints(mut self, p_format_constraints: &'a [vk::ImageFormatConstraintsInfoFUCHSIA]) -> Self {
        self.inner.format_constraints_count = p_format_constraints.len() as u32;
        self.inner.p_format_constraints = p_format_constraints.as_ptr();
        self
    }
    pub fn buffer_collection_constraints(
        mut self,
        buffer_collection_constraints: vk::BufferCollectionConstraintsInfoFUCHSIA,
    ) -> Self {
        self.inner.buffer_collection_constraints = buffer_collection_constraints;
        self
    }
    pub fn flags(mut self, flags: vk::ImageConstraintsInfoFlagsFUCHSIA) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for ImageConstraintsInfoFUCHSIABuilder<'a> {
    type Target = vk::ImageConstraintsInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CudaModuleCreateInfoNVBuilder<'a> {
    inner: vk::CudaModuleCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CudaModuleCreateInfoNV {
    type Type = CudaModuleCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CudaModuleCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn data_size(mut self, data_size: usize) -> Self {
        self.inner.data_size = data_size;
        self
    }
    pub fn p_data(mut self, p_data: *const c_void) -> Self {
        self.inner.p_data = p_data;
        self
    }
}
impl<'a> Deref for CudaModuleCreateInfoNVBuilder<'a> {
    type Target = vk::CudaModuleCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CudaFunctionCreateInfoNVBuilder<'a> {
    inner: vk::CudaFunctionCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CudaFunctionCreateInfoNV {
    type Type = CudaFunctionCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CudaFunctionCreateInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn module(mut self, module: vk::CudaModuleNV) -> Self {
        self.inner.module = module;
        self
    }
    pub fn p_name(mut self, p_name: &'a CStr) -> Self {
        self.inner.p_name = p_name.as_ptr();
        self
    }
}
impl<'a> Deref for CudaFunctionCreateInfoNVBuilder<'a> {
    type Target = vk::CudaFunctionCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CudaLaunchInfoNVBuilder<'a> {
    inner: vk::CudaLaunchInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CudaLaunchInfoNV {
    type Type = CudaLaunchInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CudaLaunchInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn function(mut self, function: vk::CudaFunctionNV) -> Self {
        self.inner.function = function;
        self
    }
    pub fn grid_dim_x(mut self, grid_dim_x: u32) -> Self {
        self.inner.grid_dim_x = grid_dim_x;
        self
    }
    pub fn grid_dim_y(mut self, grid_dim_y: u32) -> Self {
        self.inner.grid_dim_y = grid_dim_y;
        self
    }
    pub fn grid_dim_z(mut self, grid_dim_z: u32) -> Self {
        self.inner.grid_dim_z = grid_dim_z;
        self
    }
    pub fn block_dim_x(mut self, block_dim_x: u32) -> Self {
        self.inner.block_dim_x = block_dim_x;
        self
    }
    pub fn block_dim_y(mut self, block_dim_y: u32) -> Self {
        self.inner.block_dim_y = block_dim_y;
        self
    }
    pub fn block_dim_z(mut self, block_dim_z: u32) -> Self {
        self.inner.block_dim_z = block_dim_z;
        self
    }
    pub fn shared_mem_bytes(mut self, shared_mem_bytes: u32) -> Self {
        self.inner.shared_mem_bytes = shared_mem_bytes;
        self
    }
    pub fn param_count(mut self, param_count: usize) -> Self {
        self.inner.param_count = param_count;
        self
    }
    pub fn p_params(mut self, p_params: &'a [*const c_void]) -> Self {
        self.inner.param_count = p_params.len();
        self.inner.p_params = p_params.as_ptr();
        self
    }
    pub fn extra_count(mut self, extra_count: usize) -> Self {
        self.inner.extra_count = extra_count;
        self
    }
    pub fn p_extras(mut self, p_extras: &'a [*const c_void]) -> Self {
        self.inner.extra_count = p_extras.len();
        self.inner.p_extras = p_extras.as_ptr();
        self
    }
}
impl<'a> Deref for CudaLaunchInfoNVBuilder<'a> {
    type Target = vk::CudaLaunchInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRGBA10X6FormatsFeaturesEXT {}
impl FormatProperties2Next for vk::FormatProperties3 {}
impl FormatProperties2Next for vk::DrmFormatModifierPropertiesList2EXT {}
impl AndroidHardwareBufferPropertiesANDROIDNext for vk::AndroidHardwareBufferFormatProperties2ANDROID {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineRenderingCreateInfoBuilder<'a> {
    inner: vk::PipelineRenderingCreateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineRenderingCreateInfo {
    type Type = PipelineRenderingCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineRenderingCreateInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn color_attachment_count(mut self, color_attachment_count: u32) -> Self {
        self.inner.color_attachment_count = color_attachment_count;
        self
    }
    pub fn p_color_attachment_formats(mut self, p_color_attachment_formats: &'a [vk::Format]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_formats.len() as u32;
        self.inner.p_color_attachment_formats = p_color_attachment_formats.as_ptr();
        self
    }
    pub fn depth_attachment_format(mut self, depth_attachment_format: vk::Format) -> Self {
        self.inner.depth_attachment_format = depth_attachment_format;
        self
    }
    pub fn stencil_attachment_format(mut self, stencil_attachment_format: vk::Format) -> Self {
        self.inner.stencil_attachment_format = stencil_attachment_format;
        self
    }
}
impl<'a> Deref for PipelineRenderingCreateInfoBuilder<'a> {
    type Target = vk::PipelineRenderingCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for vk::PipelineRenderingCreateInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderingInfoBuilder<'a> {
    inner: vk::RenderingInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderingInfo {
    type Type = RenderingInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderingInfoNext {}
impl<'a> RenderingInfoBuilder<'a> {
    pub fn insert_next<T: RenderingInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderingFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
    pub fn layer_count(mut self, layer_count: u32) -> Self {
        self.inner.layer_count = layer_count;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn p_color_attachments(mut self, p_color_attachments: &'a [vk::RenderingAttachmentInfo]) -> Self {
        self.inner.color_attachment_count = p_color_attachments.len() as u32;
        self.inner.p_color_attachments = p_color_attachments.as_ptr();
        self
    }
    pub fn p_depth_attachment(mut self, p_depth_attachment: Option<&'a vk::RenderingAttachmentInfo>) -> Self {
        self.inner.p_depth_attachment = p_depth_attachment.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_stencil_attachment(mut self, p_stencil_attachment: Option<&'a vk::RenderingAttachmentInfo>) -> Self {
        self.inner.p_stencil_attachment = p_stencil_attachment.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for RenderingInfoBuilder<'a> {
    type Target = vk::RenderingInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderingEndInfoEXTBuilder<'a> {
    inner: vk::RenderingEndInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderingEndInfoEXT {
    type Type = RenderingEndInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderingEndInfoEXTNext {}
impl<'a> RenderingEndInfoEXTBuilder<'a> {
    pub fn insert_next<T: RenderingEndInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for RenderingEndInfoEXTBuilder<'a> {
    type Target = vk::RenderingEndInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderingAttachmentInfoBuilder<'a> {
    inner: vk::RenderingAttachmentInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderingAttachmentInfo {
    type Type = RenderingAttachmentInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait RenderingAttachmentInfoNext {}
impl<'a> RenderingAttachmentInfoBuilder<'a> {
    pub fn insert_next<T: RenderingAttachmentInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for RenderingAttachmentInfoBuilder<'a> {
    type Target = vk::RenderingAttachmentInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderingInfoNext for vk::RenderingFragmentShadingRateAttachmentInfoKHR {}
impl RenderingInfoNext for vk::RenderingFragmentDensityMapAttachmentInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDynamicRenderingFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDynamicRenderingFeatures {}

#[repr(transparent)]
#[derive(Default)]
pub struct CommandBufferInheritanceRenderingInfoBuilder<'a> {
    inner: vk::CommandBufferInheritanceRenderingInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceRenderingInfo {
    type Type = CommandBufferInheritanceRenderingInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CommandBufferInheritanceRenderingInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::RenderingFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn view_mask(mut self, view_mask: u32) -> Self {
        self.inner.view_mask = view_mask;
        self
    }
    pub fn p_color_attachment_formats(mut self, p_color_attachment_formats: &'a [vk::Format]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_formats.len() as u32;
        self.inner.p_color_attachment_formats = p_color_attachment_formats.as_ptr();
        self
    }
    pub fn depth_attachment_format(mut self, depth_attachment_format: vk::Format) -> Self {
        self.inner.depth_attachment_format = depth_attachment_format;
        self
    }
    pub fn stencil_attachment_format(mut self, stencil_attachment_format: vk::Format) -> Self {
        self.inner.stencil_attachment_format = stencil_attachment_format;
        self
    }
    pub fn rasterization_samples(mut self, rasterization_samples: vk::SampleCountFlags) -> Self {
        self.inner.rasterization_samples = rasterization_samples;
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceRenderingInfoBuilder<'a> {
    type Target = vk::CommandBufferInheritanceRenderingInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CommandBufferInheritanceInfoNext for vk::CommandBufferInheritanceRenderingInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct AttachmentSampleCountInfoAMDBuilder<'a> {
    inner: vk::AttachmentSampleCountInfoAMD,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AttachmentSampleCountInfoAMD {
    type Type = AttachmentSampleCountInfoAMDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AttachmentSampleCountInfoAMDBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn color_attachment_count(mut self, color_attachment_count: u32) -> Self {
        self.inner.color_attachment_count = color_attachment_count;
        self
    }
    pub fn p_color_attachment_samples(mut self, p_color_attachment_samples: &'a [vk::SampleCountFlags]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_samples.len() as u32;
        self.inner.p_color_attachment_samples = p_color_attachment_samples.as_ptr();
        self
    }
    pub fn depth_stencil_attachment_samples(mut self, depth_stencil_attachment_samples: vk::SampleCountFlags) -> Self {
        self.inner.depth_stencil_attachment_samples = depth_stencil_attachment_samples;
        self
    }
}
impl<'a> Deref for AttachmentSampleCountInfoAMDBuilder<'a> {
    type Target = vk::AttachmentSampleCountInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CommandBufferInheritanceInfoNext for vk::AttachmentSampleCountInfoAMD {}
impl GraphicsPipelineCreateInfoNext for vk::AttachmentSampleCountInfoAMD {}
impl CommandBufferInheritanceInfoNext for vk::MultiviewPerViewAttributesInfoNVX {}
impl GraphicsPipelineCreateInfoNext for vk::MultiviewPerViewAttributesInfoNVX {}
impl RenderingInfoNext for vk::MultiviewPerViewAttributesInfoNVX {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageViewMinLodFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageViewMinLodFeaturesEXT {}
impl ImageViewCreateInfoNext for vk::ImageViewMinLodCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceLinearColorAttachmentFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceLinearColorAttachmentFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineBinaryFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineBinaryFeaturesKHR {}
impl DeviceCreateInfoNext for vk::DevicePipelineBinaryInternalCacheControlKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePipelineBinaryPropertiesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {}
impl GraphicsPipelineCreateInfoNext for vk::GraphicsPipelineLibraryCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceNestedCommandBufferFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceNestedCommandBufferFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceNestedCommandBufferPropertiesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderModuleIdentifierFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderModuleIdentifierFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderModuleIdentifierPropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineShaderStageModuleIdentifierCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineShaderStageModuleIdentifierCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineShaderStageModuleIdentifierCreateInfoEXT {
    type Type = PipelineShaderStageModuleIdentifierCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineShaderStageModuleIdentifierCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_identifier(mut self, p_identifier: &'a [u8]) -> Self {
        self.inner.identifier_size = p_identifier.len() as u32;
        self.inner.p_identifier = p_identifier.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineShaderStageModuleIdentifierCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineShaderStageModuleIdentifierCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineShaderStageCreateInfoNext for vk::PipelineShaderStageModuleIdentifierCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ImageCompressionControlEXTBuilder<'a> {
    inner: vk::ImageCompressionControlEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ImageCompressionControlEXT {
    type Type = ImageCompressionControlEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ImageCompressionControlEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageCompressionFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn compression_control_plane_count(mut self, compression_control_plane_count: u32) -> Self {
        self.inner.compression_control_plane_count = compression_control_plane_count;
        self
    }
    pub fn p_fixed_rate_flags(mut self, p_fixed_rate_flags: &'a mut [vk::ImageCompressionFixedRateFlagsEXT]) -> Self {
        self.inner.compression_control_plane_count = p_fixed_rate_flags.len() as u32;
        self.inner.p_fixed_rate_flags = p_fixed_rate_flags.as_mut_ptr();
        self
    }
}
impl<'a> Deref for ImageCompressionControlEXTBuilder<'a> {
    type Target = vk::ImageCompressionControlEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ImageCreateInfoNext for vk::ImageCompressionControlEXT {}
impl SwapchainCreateInfoKHRNext for vk::ImageCompressionControlEXT {}
impl PhysicalDeviceImageFormatInfo2Next for vk::ImageCompressionControlEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageCompressionControlFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageCompressionControlFeaturesEXT {}
impl ImageFormatProperties2Next for vk::ImageCompressionPropertiesEXT {}
impl SurfaceFormat2KHRNext for vk::ImageCompressionPropertiesEXT {}
impl SubresourceLayout2Next for vk::ImageCompressionPropertiesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct SubresourceLayout2Builder<'a> {
    inner: vk::SubresourceLayout2,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SubresourceLayout2 {
    type Type = SubresourceLayout2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SubresourceLayout2Next {}
impl<'a> SubresourceLayout2Builder<'a> {
    pub fn insert_next<T: SubresourceLayout2Next>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for SubresourceLayout2Builder<'a> {
    type Target = vk::SubresourceLayout2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassCreateInfo2Next for vk::RenderPassCreationControlEXT {}
impl SubpassDescription2Next for vk::RenderPassCreationControlEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassCreationFeedbackCreateInfoEXTBuilder<'a> {
    inner: vk::RenderPassCreationFeedbackCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassCreationFeedbackCreateInfoEXT {
    type Type = RenderPassCreationFeedbackCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassCreationFeedbackCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_render_pass_feedback(
        mut self,
        p_render_pass_feedback: *mut vk::RenderPassCreationFeedbackInfoEXT,
    ) -> Self {
        self.inner.p_render_pass_feedback = p_render_pass_feedback;
        self
    }
}
impl<'a> Deref for RenderPassCreationFeedbackCreateInfoEXTBuilder<'a> {
    type Target = vk::RenderPassCreationFeedbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassCreateInfo2Next for vk::RenderPassCreationFeedbackCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassSubpassFeedbackCreateInfoEXTBuilder<'a> {
    inner: vk::RenderPassSubpassFeedbackCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassSubpassFeedbackCreateInfoEXT {
    type Type = RenderPassSubpassFeedbackCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassSubpassFeedbackCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_subpass_feedback(mut self, p_subpass_feedback: *mut vk::RenderPassSubpassFeedbackInfoEXT) -> Self {
        self.inner.p_subpass_feedback = p_subpass_feedback;
        self
    }
}
impl<'a> Deref for RenderPassSubpassFeedbackCreateInfoEXTBuilder<'a> {
    type Target = vk::RenderPassSubpassFeedbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubpassDescription2Next for vk::RenderPassSubpassFeedbackCreateInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSubpassMergeFeedbackFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct MicromapBuildInfoEXTBuilder<'a> {
    inner: vk::MicromapBuildInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MicromapBuildInfoEXT {
    type Type = MicromapBuildInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MicromapBuildInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::MicromapTypeEXT) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn flags(mut self, flags: vk::BuildMicromapFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn mode(mut self, mode: vk::BuildMicromapModeEXT) -> Self {
        self.inner.mode = mode;
        self
    }
    pub fn dst_micromap(mut self, dst_micromap: vk::MicromapEXT) -> Self {
        self.inner.dst_micromap = dst_micromap;
        self
    }
    pub fn p_usage_counts(
        mut self,
        p_usage_counts: Option<&'a [vk::MicromapUsageEXT]>,
        pp_usage_counts: Option<&'a [*const vk::MicromapUsageEXT]>,
    ) -> Self {
        self.inner.usage_counts_count = p_usage_counts
            .map(|s| s.len() as u32)
            .or(pp_usage_counts.map(|s| s.len() as u32))
            .unwrap_or(0);
        if let Some(len) = p_usage_counts.map(|s| s.len()) {
            assert_eq!(self.inner.usage_counts_count, len as u32);
        }
        if let Some(len) = pp_usage_counts.map(|s| s.len()) {
            assert_eq!(self.inner.usage_counts_count, len as u32);
        }
        self.inner.p_usage_counts = p_usage_counts.map_or(ptr::null(), |s| s.as_ptr());
        self.inner.pp_usage_counts = pp_usage_counts.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn data(mut self, data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.data = data;
        self
    }
    pub fn scratch_data(mut self, scratch_data: vk::DeviceOrHostAddressKHR) -> Self {
        self.inner.scratch_data = scratch_data;
        self
    }
    pub fn triangle_array(mut self, triangle_array: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.triangle_array = triangle_array;
        self
    }
    pub fn triangle_array_stride(mut self, triangle_array_stride: vk::DeviceSize) -> Self {
        self.inner.triangle_array_stride = triangle_array_stride;
        self
    }
}
impl<'a> Deref for MicromapBuildInfoEXTBuilder<'a> {
    type Target = vk::MicromapBuildInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct MicromapVersionInfoEXTBuilder<'a> {
    inner: vk::MicromapVersionInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MicromapVersionInfoEXT {
    type Type = MicromapVersionInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MicromapVersionInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_version_data(mut self, p_version_data: *const u8) -> Self {
        self.inner.p_version_data = p_version_data;
        self
    }
}
impl<'a> Deref for MicromapVersionInfoEXTBuilder<'a> {
    type Target = vk::MicromapVersionInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceOpacityMicromapFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceOpacityMicromapFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceOpacityMicromapPropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureTrianglesOpacityMicromapEXTBuilder<'a> {
    inner: vk::AccelerationStructureTrianglesOpacityMicromapEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureTrianglesOpacityMicromapEXT {
    type Type = AccelerationStructureTrianglesOpacityMicromapEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AccelerationStructureTrianglesOpacityMicromapEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn index_buffer(mut self, index_buffer: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.index_buffer = index_buffer;
        self
    }
    pub fn index_stride(mut self, index_stride: vk::DeviceSize) -> Self {
        self.inner.index_stride = index_stride;
        self
    }
    pub fn base_triangle(mut self, base_triangle: u32) -> Self {
        self.inner.base_triangle = base_triangle;
        self
    }
    pub fn p_usage_counts(
        mut self,
        p_usage_counts: Option<&'a [vk::MicromapUsageEXT]>,
        pp_usage_counts: Option<&'a [*const vk::MicromapUsageEXT]>,
    ) -> Self {
        self.inner.usage_counts_count = p_usage_counts
            .map(|s| s.len() as u32)
            .or(pp_usage_counts.map(|s| s.len() as u32))
            .unwrap_or(0);
        if let Some(len) = p_usage_counts.map(|s| s.len()) {
            assert_eq!(self.inner.usage_counts_count, len as u32);
        }
        if let Some(len) = pp_usage_counts.map(|s| s.len()) {
            assert_eq!(self.inner.usage_counts_count, len as u32);
        }
        self.inner.p_usage_counts = p_usage_counts.map_or(ptr::null(), |s| s.as_ptr());
        self.inner.pp_usage_counts = pp_usage_counts.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn micromap(mut self, micromap: vk::MicromapEXT) -> Self {
        self.inner.micromap = micromap;
        self
    }
}
impl<'a> Deref for AccelerationStructureTrianglesOpacityMicromapEXTBuilder<'a> {
    type Target = vk::AccelerationStructureTrianglesOpacityMicromapEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AccelerationStructureGeometryTrianglesDataKHRNext for vk::AccelerationStructureTrianglesOpacityMicromapEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDisplacementMicromapFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDisplacementMicromapFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDisplacementMicromapPropertiesNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct AccelerationStructureTrianglesDisplacementMicromapNVBuilder<'a> {
    inner: vk::AccelerationStructureTrianglesDisplacementMicromapNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AccelerationStructureTrianglesDisplacementMicromapNV {
    type Type = AccelerationStructureTrianglesDisplacementMicromapNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AccelerationStructureTrianglesDisplacementMicromapNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn displacement_bias_and_scale_format(mut self, displacement_bias_and_scale_format: vk::Format) -> Self {
        self.inner.displacement_bias_and_scale_format = displacement_bias_and_scale_format;
        self
    }
    pub fn displacement_vector_format(mut self, displacement_vector_format: vk::Format) -> Self {
        self.inner.displacement_vector_format = displacement_vector_format;
        self
    }
    pub fn displacement_bias_and_scale_buffer(
        mut self,
        displacement_bias_and_scale_buffer: vk::DeviceOrHostAddressConstKHR,
    ) -> Self {
        self.inner.displacement_bias_and_scale_buffer = displacement_bias_and_scale_buffer;
        self
    }
    pub fn displacement_bias_and_scale_stride(mut self, displacement_bias_and_scale_stride: vk::DeviceSize) -> Self {
        self.inner.displacement_bias_and_scale_stride = displacement_bias_and_scale_stride;
        self
    }
    pub fn displacement_vector_buffer(mut self, displacement_vector_buffer: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.displacement_vector_buffer = displacement_vector_buffer;
        self
    }
    pub fn displacement_vector_stride(mut self, displacement_vector_stride: vk::DeviceSize) -> Self {
        self.inner.displacement_vector_stride = displacement_vector_stride;
        self
    }
    pub fn displaced_micromap_primitive_flags(
        mut self,
        displaced_micromap_primitive_flags: vk::DeviceOrHostAddressConstKHR,
    ) -> Self {
        self.inner.displaced_micromap_primitive_flags = displaced_micromap_primitive_flags;
        self
    }
    pub fn displaced_micromap_primitive_flags_stride(
        mut self,
        displaced_micromap_primitive_flags_stride: vk::DeviceSize,
    ) -> Self {
        self.inner.displaced_micromap_primitive_flags_stride = displaced_micromap_primitive_flags_stride;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn index_buffer(mut self, index_buffer: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.index_buffer = index_buffer;
        self
    }
    pub fn index_stride(mut self, index_stride: vk::DeviceSize) -> Self {
        self.inner.index_stride = index_stride;
        self
    }
    pub fn base_triangle(mut self, base_triangle: u32) -> Self {
        self.inner.base_triangle = base_triangle;
        self
    }
    pub fn p_usage_counts(
        mut self,
        p_usage_counts: Option<&'a [vk::MicromapUsageEXT]>,
        pp_usage_counts: Option<&'a [*const vk::MicromapUsageEXT]>,
    ) -> Self {
        self.inner.usage_counts_count = p_usage_counts
            .map(|s| s.len() as u32)
            .or(pp_usage_counts.map(|s| s.len() as u32))
            .unwrap_or(0);
        if let Some(len) = p_usage_counts.map(|s| s.len()) {
            assert_eq!(self.inner.usage_counts_count, len as u32);
        }
        if let Some(len) = pp_usage_counts.map(|s| s.len()) {
            assert_eq!(self.inner.usage_counts_count, len as u32);
        }
        self.inner.p_usage_counts = p_usage_counts.map_or(ptr::null(), |s| s.as_ptr());
        self.inner.pp_usage_counts = pp_usage_counts.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn micromap(mut self, micromap: vk::MicromapEXT) -> Self {
        self.inner.micromap = micromap;
        self
    }
}
impl<'a> Deref for AccelerationStructureTrianglesDisplacementMicromapNVBuilder<'a> {
    type Target = vk::AccelerationStructureTrianglesDisplacementMicromapNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl AccelerationStructureGeometryTrianglesDataKHRNext for vk::AccelerationStructureTrianglesDisplacementMicromapNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelinePropertiesFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelinePropertiesFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {}
impl BufferMemoryBarrierNext for vk::ExternalMemoryAcquireUnmodifiedEXT {}
impl BufferMemoryBarrier2Next for vk::ExternalMemoryAcquireUnmodifiedEXT {}
impl ImageMemoryBarrierNext for vk::ExternalMemoryAcquireUnmodifiedEXT {}
impl ImageMemoryBarrier2Next for vk::ExternalMemoryAcquireUnmodifiedEXT {}
impl InstanceCreateInfoNext for vk::ExportMetalObjectCreateInfoEXT {}
impl MemoryAllocateInfoNext for vk::ExportMetalObjectCreateInfoEXT {}
impl ImageCreateInfoNext for vk::ExportMetalObjectCreateInfoEXT {}
impl ImageViewCreateInfoNext for vk::ExportMetalObjectCreateInfoEXT {}
impl BufferViewCreateInfoNext for vk::ExportMetalObjectCreateInfoEXT {}
impl SemaphoreCreateInfoNext for vk::ExportMetalObjectCreateInfoEXT {}
impl EventCreateInfoNext for vk::ExportMetalObjectCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ExportMetalObjectsInfoEXTBuilder<'a> {
    inner: vk::ExportMetalObjectsInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ExportMetalObjectsInfoEXT {
    type Type = ExportMetalObjectsInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ExportMetalObjectsInfoEXTNext {}
impl<'a> ExportMetalObjectsInfoEXTBuilder<'a> {
    pub fn insert_next<T: ExportMetalObjectsInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for ExportMetalObjectsInfoEXTBuilder<'a> {
    type Target = vk::ExportMetalObjectsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl ExportMetalObjectsInfoEXTNext for vk::ExportMetalDeviceInfoEXT {}
impl ExportMetalObjectsInfoEXTNext for vk::ExportMetalCommandQueueInfoEXT {}
impl ExportMetalObjectsInfoEXTNext for vk::ExportMetalBufferInfoEXT {}
impl MemoryAllocateInfoNext for vk::ImportMetalBufferInfoEXT {}
impl ExportMetalObjectsInfoEXTNext for vk::ExportMetalTextureInfoEXT {}
impl ImageCreateInfoNext for vk::ImportMetalTextureInfoEXT {}
impl ExportMetalObjectsInfoEXTNext for vk::ExportMetalIOSurfaceInfoEXT {}
impl ImageCreateInfoNext for vk::ImportMetalIOSurfaceInfoEXT {}
impl ExportMetalObjectsInfoEXTNext for vk::ExportMetalSharedEventInfoEXT {}
impl SemaphoreCreateInfoNext for vk::ImportMetalSharedEventInfoEXT {}
impl EventCreateInfoNext for vk::ImportMetalSharedEventInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceNonSeamlessCubeMapFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceNonSeamlessCubeMapFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineRobustnessFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineRobustnessFeatures {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineRobustnessCreateInfo {}
impl ComputePipelineCreateInfoNext for vk::PipelineRobustnessCreateInfo {}
impl PipelineShaderStageCreateInfoNext for vk::PipelineRobustnessCreateInfo {}
impl RayTracingPipelineCreateInfoKHRNext for vk::PipelineRobustnessCreateInfo {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDevicePipelineRobustnessProperties {}
impl ImageViewCreateInfoNext for vk::ImageViewSampleWeightCreateInfoQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageProcessingFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageProcessingFeaturesQCOM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceImageProcessingPropertiesQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTilePropertiesFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTilePropertiesFeaturesQCOM {}
impl CommandBufferInheritanceInfoNext for vk::TileMemoryBindInfoQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceAmigoProfilingFeaturesSEC {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceAmigoProfilingFeaturesSEC {}
impl SubmitInfoNext for vk::AmigoProfilingSubmitInfoSEC {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {}
impl RenderingAttachmentInfoNext for vk::AttachmentFeedbackLoopInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceAddressBindingReportFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceAddressBindingReportFeaturesEXT {}
impl DebugUtilsMessengerCallbackDataEXTNext for vk::DeviceAddressBindingCallbackDataEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceOpticalFlowFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceOpticalFlowFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceOpticalFlowPropertiesNV {}
impl PhysicalDeviceImageFormatInfo2Next for vk::OpticalFlowImageFormatInfoNV {}
impl ImageCreateInfoNext for vk::OpticalFlowImageFormatInfoNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct OpticalFlowSessionCreateInfoNVBuilder<'a> {
    inner: vk::OpticalFlowSessionCreateInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::OpticalFlowSessionCreateInfoNV {
    type Type = OpticalFlowSessionCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait OpticalFlowSessionCreateInfoNVNext {}
impl<'a> OpticalFlowSessionCreateInfoNVBuilder<'a> {
    pub fn insert_next<T: OpticalFlowSessionCreateInfoNVNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for OpticalFlowSessionCreateInfoNVBuilder<'a> {
    type Target = vk::OpticalFlowSessionCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct OpticalFlowSessionCreatePrivateDataInfoNVBuilder<'a> {
    inner: vk::OpticalFlowSessionCreatePrivateDataInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::OpticalFlowSessionCreatePrivateDataInfoNV {
    type Type = OpticalFlowSessionCreatePrivateDataInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> OpticalFlowSessionCreatePrivateDataInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn id(mut self, id: u32) -> Self {
        self.inner.id = id;
        self
    }
    pub fn size(mut self, size: u32) -> Self {
        self.inner.size = size;
        self
    }
    pub fn p_private_data(mut self, p_private_data: *const c_void) -> Self {
        self.inner.p_private_data = p_private_data;
        self
    }
}
impl<'a> Deref for OpticalFlowSessionCreatePrivateDataInfoNVBuilder<'a> {
    type Target = vk::OpticalFlowSessionCreatePrivateDataInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl OpticalFlowSessionCreateInfoNVNext for vk::OpticalFlowSessionCreatePrivateDataInfoNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct OpticalFlowExecuteInfoNVBuilder<'a> {
    inner: vk::OpticalFlowExecuteInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::OpticalFlowExecuteInfoNV {
    type Type = OpticalFlowExecuteInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> OpticalFlowExecuteInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::OpticalFlowExecuteFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::Rect2D]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for OpticalFlowExecuteInfoNVBuilder<'a> {
    type Target = vk::OpticalFlowExecuteInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFaultFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFaultFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct DepthBiasInfoEXTBuilder<'a> {
    inner: vk::DepthBiasInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DepthBiasInfoEXT {
    type Type = DepthBiasInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait DepthBiasInfoEXTNext {}
impl<'a> DepthBiasInfoEXTBuilder<'a> {
    pub fn insert_next<T: DepthBiasInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for DepthBiasInfoEXTBuilder<'a> {
    type Target = vk::DepthBiasInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DepthBiasInfoEXTNext for vk::DepthBiasRepresentationInfoEXT {}
impl PipelineRasterizationStateCreateInfoNext for vk::DepthBiasRepresentationInfoEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderCoreBuiltinsPropertiesARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderCoreBuiltinsFeaturesARM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderCoreBuiltinsFeaturesARM {}

#[repr(transparent)]
#[derive(Default)]
pub struct FrameBoundaryEXTBuilder<'a> {
    inner: vk::FrameBoundaryEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::FrameBoundaryEXT {
    type Type = FrameBoundaryEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> FrameBoundaryEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::FrameBoundaryFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn frame_id(mut self, frame_id: u64) -> Self {
        self.inner.frame_id = frame_id;
        self
    }
    pub fn p_images(mut self, p_images: &'a [vk::Image]) -> Self {
        self.inner.image_count = p_images.len() as u32;
        self.inner.p_images = p_images.as_ptr();
        self
    }
    pub fn p_buffers(mut self, p_buffers: &'a [vk::Buffer]) -> Self {
        self.inner.buffer_count = p_buffers.len() as u32;
        self.inner.p_buffers = p_buffers.as_ptr();
        self
    }
    pub fn tag_name(mut self, tag_name: u64) -> Self {
        self.inner.tag_name = tag_name;
        self
    }
    pub fn tag_size(mut self, tag_size: usize) -> Self {
        self.inner.tag_size = tag_size;
        self
    }
    pub fn p_tag(mut self, p_tag: *const c_void) -> Self {
        self.inner.p_tag = p_tag;
        self
    }
}
impl<'a> Deref for FrameBoundaryEXTBuilder<'a> {
    type Target = vk::FrameBoundaryEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for vk::FrameBoundaryEXT {}
impl SubmitInfo2Next for vk::FrameBoundaryEXT {}
impl PresentInfoKHRNext for vk::FrameBoundaryEXT {}
impl BindSparseInfoNext for vk::FrameBoundaryEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFrameBoundaryFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFrameBoundaryFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {}
impl PhysicalDeviceSurfaceInfo2KHRNext for vk::SurfacePresentModeEXT {}
impl SurfaceCapabilities2KHRNext for vk::SurfacePresentScalingCapabilitiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct SurfacePresentModeCompatibilityEXTBuilder<'a> {
    inner: vk::SurfacePresentModeCompatibilityEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SurfacePresentModeCompatibilityEXT {
    type Type = SurfacePresentModeCompatibilityEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SurfacePresentModeCompatibilityEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_present_modes(mut self, p_present_modes: &'a mut [vk::PresentModeKHR]) -> Self {
        self.inner.present_mode_count = p_present_modes.len() as u32;
        self.inner.p_present_modes = p_present_modes.as_mut_ptr();
        self
    }
}
impl<'a> Deref for SurfacePresentModeCompatibilityEXTBuilder<'a> {
    type Target = vk::SurfacePresentModeCompatibilityEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SurfaceCapabilities2KHRNext for vk::SurfacePresentModeCompatibilityEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSwapchainMaintenance1FeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct SwapchainPresentFenceInfoEXTBuilder<'a> {
    inner: vk::SwapchainPresentFenceInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SwapchainPresentFenceInfoEXT {
    type Type = SwapchainPresentFenceInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SwapchainPresentFenceInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_fences(mut self, p_fences: &'a [vk::Fence]) -> Self {
        self.inner.swapchain_count = p_fences.len() as u32;
        self.inner.p_fences = p_fences.as_ptr();
        self
    }
}
impl<'a> Deref for SwapchainPresentFenceInfoEXTBuilder<'a> {
    type Target = vk::SwapchainPresentFenceInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PresentInfoKHRNext for vk::SwapchainPresentFenceInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct SwapchainPresentModesCreateInfoEXTBuilder<'a> {
    inner: vk::SwapchainPresentModesCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SwapchainPresentModesCreateInfoEXT {
    type Type = SwapchainPresentModesCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SwapchainPresentModesCreateInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_present_modes(mut self, p_present_modes: &'a [vk::PresentModeKHR]) -> Self {
        self.inner.present_mode_count = p_present_modes.len() as u32;
        self.inner.p_present_modes = p_present_modes.as_ptr();
        self
    }
}
impl<'a> Deref for SwapchainPresentModesCreateInfoEXTBuilder<'a> {
    type Target = vk::SwapchainPresentModesCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SwapchainCreateInfoKHRNext for vk::SwapchainPresentModesCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct SwapchainPresentModeInfoEXTBuilder<'a> {
    inner: vk::SwapchainPresentModeInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SwapchainPresentModeInfoEXT {
    type Type = SwapchainPresentModeInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> SwapchainPresentModeInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_present_modes(mut self, p_present_modes: &'a [vk::PresentModeKHR]) -> Self {
        self.inner.swapchain_count = p_present_modes.len() as u32;
        self.inner.p_present_modes = p_present_modes.as_ptr();
        self
    }
}
impl<'a> Deref for SwapchainPresentModeInfoEXTBuilder<'a> {
    type Target = vk::SwapchainPresentModeInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PresentInfoKHRNext for vk::SwapchainPresentModeInfoEXT {}
impl SwapchainCreateInfoKHRNext for vk::SwapchainPresentScalingCreateInfoEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ReleaseSwapchainImagesInfoEXTBuilder<'a> {
    inner: vk::ReleaseSwapchainImagesInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ReleaseSwapchainImagesInfoEXT {
    type Type = ReleaseSwapchainImagesInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ReleaseSwapchainImagesInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: vk::SwapchainKHR) -> Self {
        self.inner.swapchain = swapchain;
        self
    }
    pub fn p_image_indices(mut self, p_image_indices: &'a [u32]) -> Self {
        self.inner.image_index_count = p_image_indices.len() as u32;
        self.inner.p_image_indices = p_image_indices.as_ptr();
        self
    }
}
impl<'a> Deref for ReleaseSwapchainImagesInfoEXTBuilder<'a> {
    type Target = vk::ReleaseSwapchainImagesInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDepthBiasControlFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDepthBiasControlFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingInvocationReorderFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingInvocationReorderFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceRayTracingInvocationReorderPropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceExtendedSparseAddressSpacePropertiesNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct DirectDriverLoadingListLUNARGBuilder<'a> {
    inner: vk::DirectDriverLoadingListLUNARG,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DirectDriverLoadingListLUNARG {
    type Type = DirectDriverLoadingListLUNARGBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DirectDriverLoadingListLUNARGBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn mode(mut self, mode: vk::DirectDriverLoadingModeLUNARG) -> Self {
        self.inner.mode = mode;
        self
    }
    pub fn p_drivers(mut self, p_drivers: &'a [vk::DirectDriverLoadingInfoLUNARG]) -> Self {
        self.inner.driver_count = p_drivers.len() as u32;
        self.inner.p_drivers = p_drivers.as_ptr();
        self
    }
}
impl<'a> Deref for DirectDriverLoadingListLUNARGBuilder<'a> {
    type Target = vk::DirectDriverLoadingListLUNARG;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl InstanceCreateInfoNext for vk::DirectDriverLoadingListLUNARG {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRayTracingPositionFetchFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRayTracingPositionFetchFeaturesKHR {}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceImageSubresourceInfoBuilder<'a> {
    inner: vk::DeviceImageSubresourceInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceImageSubresourceInfo {
    type Type = DeviceImageSubresourceInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceImageSubresourceInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_create_info(mut self, p_create_info: &'a vk::ImageCreateInfo) -> Self {
        self.inner.p_create_info = p_create_info;
        self
    }
    pub fn p_subresource(mut self, p_subresource: &'a vk::ImageSubresource2) -> Self {
        self.inner.p_subresource = p_subresource;
        self
    }
}
impl<'a> Deref for DeviceImageSubresourceInfoBuilder<'a> {
    type Target = vk::DeviceImageSubresourceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderCorePropertiesARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {}

#[repr(transparent)]
#[derive(Default)]
pub struct MultiviewPerViewRenderAreasRenderPassBeginInfoQCOMBuilder<'a> {
    inner: vk::MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
    type Type = MultiviewPerViewRenderAreasRenderPassBeginInfoQCOMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MultiviewPerViewRenderAreasRenderPassBeginInfoQCOMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_per_view_render_areas(mut self, p_per_view_render_areas: &'a [vk::Rect2D]) -> Self {
        self.inner.per_view_render_area_count = p_per_view_render_areas.len() as u32;
        self.inner.p_per_view_render_areas = p_per_view_render_areas.as_ptr();
        self
    }
}
impl<'a> Deref for MultiviewPerViewRenderAreasRenderPassBeginInfoQCOMBuilder<'a> {
    type Target = vk::MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderPassBeginInfoNext for vk::MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {}
impl RenderingInfoNext for vk::MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {}

#[repr(transparent)]
#[derive(Default)]
pub struct QueryLowLatencySupportNVBuilder<'a> {
    inner: vk::QueryLowLatencySupportNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::QueryLowLatencySupportNV {
    type Type = QueryLowLatencySupportNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> QueryLowLatencySupportNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_queried_low_latency_data(mut self, p_queried_low_latency_data: *mut c_void) -> Self {
        self.inner.p_queried_low_latency_data = p_queried_low_latency_data;
        self
    }
}
impl<'a> Deref for QueryLowLatencySupportNVBuilder<'a> {
    type Target = vk::QueryLowLatencySupportNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SemaphoreCreateInfoNext for vk::QueryLowLatencySupportNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct MemoryMapInfoBuilder<'a> {
    inner: vk::MemoryMapInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MemoryMapInfo {
    type Type = MemoryMapInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait MemoryMapInfoNext {}
impl<'a> MemoryMapInfoBuilder<'a> {
    pub fn insert_next<T: MemoryMapInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for MemoryMapInfoBuilder<'a> {
    type Target = vk::MemoryMapInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderObjectFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderObjectFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderObjectPropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct ShaderCreateInfoEXTBuilder<'a> {
    inner: vk::ShaderCreateInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ShaderCreateInfoEXT {
    type Type = ShaderCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ShaderCreateInfoEXTNext {}
impl<'a> ShaderCreateInfoEXTBuilder<'a> {
    pub fn insert_next<T: ShaderCreateInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ShaderCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn stage(mut self, stage: vk::ShaderStageFlags) -> Self {
        self.inner.stage = stage;
        self
    }
    pub fn next_stage(mut self, next_stage: vk::ShaderStageFlags) -> Self {
        self.inner.next_stage = next_stage;
        self
    }
    pub fn code_type(mut self, code_type: vk::ShaderCodeTypeEXT) -> Self {
        self.inner.code_type = code_type;
        self
    }
    pub fn code_size(mut self, code_size: usize) -> Self {
        self.inner.code_size = code_size;
        self
    }
    pub fn p_code(mut self, p_code: *const c_void) -> Self {
        self.inner.p_code = p_code;
        self
    }
    pub fn p_name(mut self, p_name: Option<&'a CStr>) -> Self {
        self.inner.p_name = p_name.map_or(ptr::null(), |r| r.as_ptr());
        self
    }
    pub fn p_set_layouts(mut self, p_set_layouts: &'a [vk::DescriptorSetLayout]) -> Self {
        self.inner.set_layout_count = p_set_layouts.len() as u32;
        self.inner.p_set_layouts = p_set_layouts.as_ptr();
        self
    }
    pub fn p_push_constant_ranges(mut self, p_push_constant_ranges: &'a [vk::PushConstantRange]) -> Self {
        self.inner.push_constant_range_count = p_push_constant_ranges.len() as u32;
        self.inner.p_push_constant_ranges = p_push_constant_ranges.as_ptr();
        self
    }
    pub fn p_specialization_info(mut self, p_specialization_info: Option<&'a vk::SpecializationInfo>) -> Self {
        self.inner.p_specialization_info = p_specialization_info.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for ShaderCreateInfoEXTBuilder<'a> {
    type Target = vk::ShaderCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderTileImageFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderTileImageFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderTileImagePropertiesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCooperativeMatrixFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCooperativeMatrixFeaturesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCooperativeMatrixPropertiesKHR {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceShaderEnqueuePropertiesAMDX {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderEnqueueFeaturesAMDX {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderEnqueueFeaturesAMDX {}

#[repr(transparent)]
#[derive(Default)]
pub struct ExecutionGraphPipelineCreateInfoAMDXBuilder<'a> {
    inner: vk::ExecutionGraphPipelineCreateInfoAMDX,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ExecutionGraphPipelineCreateInfoAMDX {
    type Type = ExecutionGraphPipelineCreateInfoAMDXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait ExecutionGraphPipelineCreateInfoAMDXNext {}
impl<'a> ExecutionGraphPipelineCreateInfoAMDXBuilder<'a> {
    pub fn insert_next<T: ExecutionGraphPipelineCreateInfoAMDXNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_stages(mut self, p_stages: &'a [vk::PipelineShaderStageCreateInfo]) -> Self {
        self.inner.stage_count = p_stages.len() as u32;
        self.inner.p_stages = p_stages.as_ptr();
        self
    }
    pub fn p_library_info(mut self, p_library_info: Option<&'a vk::PipelineLibraryCreateInfoKHR>) -> Self {
        self.inner.p_library_info = p_library_info.map_or(ptr::null(), |r| r);
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: vk::Pipeline) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for ExecutionGraphPipelineCreateInfoAMDXBuilder<'a> {
    type Target = vk::ExecutionGraphPipelineCreateInfoAMDX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PipelineShaderStageNodeCreateInfoAMDXBuilder<'a> {
    inner: vk::PipelineShaderStageNodeCreateInfoAMDX,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PipelineShaderStageNodeCreateInfoAMDX {
    type Type = PipelineShaderStageNodeCreateInfoAMDXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PipelineShaderStageNodeCreateInfoAMDXBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_name(mut self, p_name: Option<&'a CStr>) -> Self {
        self.inner.p_name = p_name.map_or(ptr::null(), |r| r.as_ptr());
        self
    }
    pub fn index(mut self, index: u32) -> Self {
        self.inner.index = index;
        self
    }
}
impl<'a> Deref for PipelineShaderStageNodeCreateInfoAMDXBuilder<'a> {
    type Target = vk::PipelineShaderStageNodeCreateInfoAMDX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PipelineShaderStageCreateInfoNext for vk::PipelineShaderStageNodeCreateInfoAMDX {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceAntiLagFeaturesAMD {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceAntiLagFeaturesAMD {}

#[repr(transparent)]
#[derive(Default)]
pub struct AntiLagDataAMDBuilder<'a> {
    inner: vk::AntiLagDataAMD,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::AntiLagDataAMD {
    type Type = AntiLagDataAMDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> AntiLagDataAMDBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn mode(mut self, mode: vk::AntiLagModeAMD) -> Self {
        self.inner.mode = mode;
        self
    }
    pub fn max_fps(mut self, max_fps: u32) -> Self {
        self.inner.max_fps = max_fps;
        self
    }
    pub fn p_presentation_info(mut self, p_presentation_info: Option<&'a vk::AntiLagPresentationInfoAMD>) -> Self {
        self.inner.p_presentation_info = p_presentation_info.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for AntiLagDataAMDBuilder<'a> {
    type Target = vk::AntiLagDataAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BindMemoryStatusBuilder<'a> {
    inner: vk::BindMemoryStatus,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindMemoryStatus {
    type Type = BindMemoryStatusBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> BindMemoryStatusBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_result(mut self, p_result: *mut vk::Result) -> Self {
        self.inner.p_result = p_result;
        self
    }
}
impl<'a> Deref for BindMemoryStatusBuilder<'a> {
    type Target = vk::BindMemoryStatus;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl BindBufferMemoryInfoNext for vk::BindMemoryStatus {}
impl BindImageMemoryInfoNext for vk::BindMemoryStatus {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTileMemoryHeapFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTileMemoryHeapFeaturesQCOM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTileMemoryHeapPropertiesQCOM {}
impl RenderPassCreateInfoNext for vk::TileMemorySizeInfoQCOM {}
impl RenderPassCreateInfo2Next for vk::TileMemorySizeInfoQCOM {}
impl RenderingInfoNext for vk::TileMemorySizeInfoQCOM {}
impl MemoryRequirements2Next for vk::TileMemoryRequirementsQCOM {}

#[repr(transparent)]
#[derive(Default)]
pub struct BindDescriptorSetsInfoBuilder<'a> {
    inner: vk::BindDescriptorSetsInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindDescriptorSetsInfo {
    type Type = BindDescriptorSetsInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BindDescriptorSetsInfoNext {}
impl<'a> BindDescriptorSetsInfoBuilder<'a> {
    pub fn insert_next<T: BindDescriptorSetsInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stage_flags(mut self, stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.stage_flags = stage_flags;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn first_set(mut self, first_set: u32) -> Self {
        self.inner.first_set = first_set;
        self
    }
    pub fn p_descriptor_sets(mut self, p_descriptor_sets: &'a [vk::DescriptorSet]) -> Self {
        self.inner.descriptor_set_count = p_descriptor_sets.len() as u32;
        self.inner.p_descriptor_sets = p_descriptor_sets.as_ptr();
        self
    }
    pub fn p_dynamic_offsets(mut self, p_dynamic_offsets: &'a [u32]) -> Self {
        self.inner.dynamic_offset_count = p_dynamic_offsets.len() as u32;
        self.inner.p_dynamic_offsets = p_dynamic_offsets.as_ptr();
        self
    }
}
impl<'a> Deref for BindDescriptorSetsInfoBuilder<'a> {
    type Target = vk::BindDescriptorSetsInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PushConstantsInfoBuilder<'a> {
    inner: vk::PushConstantsInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PushConstantsInfo {
    type Type = PushConstantsInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PushConstantsInfoNext {}
impl<'a> PushConstantsInfoBuilder<'a> {
    pub fn insert_next<T: PushConstantsInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn stage_flags(mut self, stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.stage_flags = stage_flags;
        self
    }
    pub fn offset(mut self, offset: u32) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: u32) -> Self {
        self.inner.size = size;
        self
    }
    pub fn p_values(mut self, p_values: *const c_void) -> Self {
        self.inner.p_values = p_values;
        self
    }
}
impl<'a> Deref for PushConstantsInfoBuilder<'a> {
    type Target = vk::PushConstantsInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PushDescriptorSetInfoBuilder<'a> {
    inner: vk::PushDescriptorSetInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PushDescriptorSetInfo {
    type Type = PushDescriptorSetInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PushDescriptorSetInfoNext {}
impl<'a> PushDescriptorSetInfoBuilder<'a> {
    pub fn insert_next<T: PushDescriptorSetInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stage_flags(mut self, stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.stage_flags = stage_flags;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn set(mut self, set: u32) -> Self {
        self.inner.set = set;
        self
    }
    pub fn p_descriptor_writes(mut self, p_descriptor_writes: &'a [vk::WriteDescriptorSet]) -> Self {
        self.inner.descriptor_write_count = p_descriptor_writes.len() as u32;
        self.inner.p_descriptor_writes = p_descriptor_writes.as_ptr();
        self
    }
}
impl<'a> Deref for PushDescriptorSetInfoBuilder<'a> {
    type Target = vk::PushDescriptorSetInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct PushDescriptorSetWithTemplateInfoBuilder<'a> {
    inner: vk::PushDescriptorSetWithTemplateInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PushDescriptorSetWithTemplateInfo {
    type Type = PushDescriptorSetWithTemplateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait PushDescriptorSetWithTemplateInfoNext {}
impl<'a> PushDescriptorSetWithTemplateInfoBuilder<'a> {
    pub fn insert_next<T: PushDescriptorSetWithTemplateInfoNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn descriptor_update_template(mut self, descriptor_update_template: vk::DescriptorUpdateTemplate) -> Self {
        self.inner.descriptor_update_template = descriptor_update_template;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn set(mut self, set: u32) -> Self {
        self.inner.set = set;
        self
    }
    pub fn p_data(mut self, p_data: *const c_void) -> Self {
        self.inner.p_data = p_data;
        self
    }
}
impl<'a> Deref for PushDescriptorSetWithTemplateInfoBuilder<'a> {
    type Target = vk::PushDescriptorSetWithTemplateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct SetDescriptorBufferOffsetsInfoEXTBuilder<'a> {
    inner: vk::SetDescriptorBufferOffsetsInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::SetDescriptorBufferOffsetsInfoEXT {
    type Type = SetDescriptorBufferOffsetsInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait SetDescriptorBufferOffsetsInfoEXTNext {}
impl<'a> SetDescriptorBufferOffsetsInfoEXTBuilder<'a> {
    pub fn insert_next<T: SetDescriptorBufferOffsetsInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stage_flags(mut self, stage_flags: vk::ShaderStageFlags) -> Self {
        self.inner.stage_flags = stage_flags;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn first_set(mut self, first_set: u32) -> Self {
        self.inner.first_set = first_set;
        self
    }
    pub fn p_buffer_indices(mut self, p_buffer_indices: &'a [u32], p_offsets: &'a [vk::DeviceSize]) -> Self {
        self.inner.set_count = p_buffer_indices.len() as u32;
        assert_eq!(self.inner.set_count, p_offsets.len() as u32);
        self.inner.p_buffer_indices = p_buffer_indices.as_ptr();
        self.inner.p_offsets = p_offsets.as_ptr();
        self
    }
}
impl<'a> Deref for SetDescriptorBufferOffsetsInfoEXTBuilder<'a> {
    type Target = vk::SetDescriptorBufferOffsetsInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct BindDescriptorBufferEmbeddedSamplersInfoEXTBuilder<'a> {
    inner: vk::BindDescriptorBufferEmbeddedSamplersInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::BindDescriptorBufferEmbeddedSamplersInfoEXT {
    type Type = BindDescriptorBufferEmbeddedSamplersInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait BindDescriptorBufferEmbeddedSamplersInfoEXTNext {}
impl<'a> BindDescriptorBufferEmbeddedSamplersInfoEXTBuilder<'a> {
    pub fn insert_next<T: BindDescriptorBufferEmbeddedSamplersInfoEXTNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for BindDescriptorBufferEmbeddedSamplersInfoEXTBuilder<'a> {
    type Target = vk::BindDescriptorBufferEmbeddedSamplersInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCubicClampFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCubicClampFeaturesQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceYcbcrDegammaFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceYcbcrDegammaFeaturesQCOM {}
impl SamplerYcbcrConversionCreateInfoNext for vk::SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCubicWeightsFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCubicWeightsFeaturesQCOM {}
impl SamplerCreateInfoNext for vk::SamplerCubicWeightsCreateInfoQCOM {}
impl BlitImageInfo2Next for vk::BlitImageCubicWeightsInfoQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageProcessing2FeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageProcessing2FeaturesQCOM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceImageProcessing2PropertiesQCOM {}
impl SamplerCreateInfoNext for vk::SamplerBlockMatchWindowCreateInfoQCOM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDescriptorPoolOverallocationFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDescriptorPoolOverallocationFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceLayeredDriverPropertiesMSFT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePerStageDescriptorSetFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePerStageDescriptorSetFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceExternalFormatResolveFeaturesANDROID {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceExternalFormatResolveFeaturesANDROID {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceExternalFormatResolvePropertiesANDROID {}
impl AndroidHardwareBufferPropertiesANDROIDNext for vk::AndroidHardwareBufferFormatResolvePropertiesANDROID {}

#[repr(transparent)]
#[derive(Default)]
pub struct GetLatencyMarkerInfoNVBuilder<'a> {
    inner: vk::GetLatencyMarkerInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::GetLatencyMarkerInfoNV {
    type Type = GetLatencyMarkerInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> GetLatencyMarkerInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_timings(mut self, p_timings: &'a mut [vk::LatencyTimingsFrameReportNV]) -> Self {
        self.inner.timing_count = p_timings.len() as u32;
        self.inner.p_timings = p_timings.as_mut_ptr();
        self
    }
}
impl<'a> Deref for GetLatencyMarkerInfoNVBuilder<'a> {
    type Target = vk::GetLatencyMarkerInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for vk::LatencySubmissionPresentIdNV {}
impl SubmitInfo2Next for vk::LatencySubmissionPresentIdNV {}
impl SwapchainCreateInfoKHRNext for vk::SwapchainLatencyCreateInfoNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct LatencySurfaceCapabilitiesNVBuilder<'a> {
    inner: vk::LatencySurfaceCapabilitiesNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::LatencySurfaceCapabilitiesNV {
    type Type = LatencySurfaceCapabilitiesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> LatencySurfaceCapabilitiesNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_present_modes(mut self, p_present_modes: &'a mut [vk::PresentModeKHR]) -> Self {
        self.inner.present_mode_count = p_present_modes.len() as u32;
        self.inner.p_present_modes = p_present_modes.as_mut_ptr();
        self
    }
}
impl<'a> Deref for LatencySurfaceCapabilitiesNVBuilder<'a> {
    type Target = vk::LatencySurfaceCapabilitiesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SurfaceCapabilities2KHRNext for vk::LatencySurfaceCapabilitiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCudaKernelLaunchFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCudaKernelLaunchFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCudaKernelLaunchPropertiesNV {}
impl DeviceQueueCreateInfoNext for vk::DeviceQueueShaderCoreControlCreateInfoARM {}
impl DeviceCreateInfoNext for vk::DeviceQueueShaderCoreControlCreateInfoARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceSchedulingControlsFeaturesARM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceSchedulingControlsFeaturesARM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceSchedulingControlsPropertiesARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRelaxedLineRasterizationFeaturesIMG {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRelaxedLineRasterizationFeaturesIMG {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRenderPassStripedFeaturesARM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRenderPassStripedFeaturesARM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceRenderPassStripedPropertiesARM {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassStripeBeginInfoARMBuilder<'a> {
    inner: vk::RenderPassStripeBeginInfoARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassStripeBeginInfoARM {
    type Type = RenderPassStripeBeginInfoARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassStripeBeginInfoARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_stripe_infos(mut self, p_stripe_infos: &'a [vk::RenderPassStripeInfoARM]) -> Self {
        self.inner.stripe_info_count = p_stripe_infos.len() as u32;
        self.inner.p_stripe_infos = p_stripe_infos.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassStripeBeginInfoARMBuilder<'a> {
    type Target = vk::RenderPassStripeBeginInfoARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl RenderingInfoNext for vk::RenderPassStripeBeginInfoARM {}
impl RenderPassBeginInfoNext for vk::RenderPassStripeBeginInfoARM {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderPassStripeSubmitInfoARMBuilder<'a> {
    inner: vk::RenderPassStripeSubmitInfoARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderPassStripeSubmitInfoARM {
    type Type = RenderPassStripeSubmitInfoARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderPassStripeSubmitInfoARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_stripe_semaphore_infos(mut self, p_stripe_semaphore_infos: &'a [vk::SemaphoreSubmitInfo]) -> Self {
        self.inner.stripe_semaphore_info_count = p_stripe_semaphore_infos.len() as u32;
        self.inner.p_stripe_semaphore_infos = p_stripe_semaphore_infos.as_ptr();
        self
    }
}
impl<'a> Deref for RenderPassStripeSubmitInfoARMBuilder<'a> {
    type Target = vk::RenderPassStripeSubmitInfoARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl CommandBufferSubmitInfoNext for vk::RenderPassStripeSubmitInfoARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePipelineOpacityMicromapFeaturesARM {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePipelineOpacityMicromapFeaturesARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderSubgroupRotateFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderSubgroupRotateFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderExpectAssumeFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderExpectAssumeFeatures {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderFloatControls2Features {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderFloatControls2Features {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDynamicRenderingLocalReadFeatures {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDynamicRenderingLocalReadFeatures {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderingAttachmentLocationInfoBuilder<'a> {
    inner: vk::RenderingAttachmentLocationInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderingAttachmentLocationInfo {
    type Type = RenderingAttachmentLocationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderingAttachmentLocationInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn color_attachment_count(mut self, color_attachment_count: u32) -> Self {
        self.inner.color_attachment_count = color_attachment_count;
        self
    }
    pub fn p_color_attachment_locations(mut self, p_color_attachment_locations: &'a [u32]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_locations.len() as u32;
        self.inner.p_color_attachment_locations = p_color_attachment_locations.as_ptr();
        self
    }
}
impl<'a> Deref for RenderingAttachmentLocationInfoBuilder<'a> {
    type Target = vk::RenderingAttachmentLocationInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for vk::RenderingAttachmentLocationInfo {}
impl CommandBufferInheritanceInfoNext for vk::RenderingAttachmentLocationInfo {}

#[repr(transparent)]
#[derive(Default)]
pub struct RenderingInputAttachmentIndexInfoBuilder<'a> {
    inner: vk::RenderingInputAttachmentIndexInfo,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::RenderingInputAttachmentIndexInfo {
    type Type = RenderingInputAttachmentIndexInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> RenderingInputAttachmentIndexInfoBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_color_attachment_input_indices(mut self, p_color_attachment_input_indices: &'a [u32]) -> Self {
        self.inner.color_attachment_count = p_color_attachment_input_indices.len() as u32;
        self.inner.p_color_attachment_input_indices = p_color_attachment_input_indices.as_ptr();
        self
    }
    pub fn p_depth_input_attachment_index(mut self, p_depth_input_attachment_index: Option<&'a u32>) -> Self {
        self.inner.p_depth_input_attachment_index = p_depth_input_attachment_index.map_or(ptr::null(), |r| r);
        self
    }
    pub fn p_stencil_input_attachment_index(mut self, p_stencil_input_attachment_index: Option<&'a u32>) -> Self {
        self.inner.p_stencil_input_attachment_index = p_stencil_input_attachment_index.map_or(ptr::null(), |r| r);
        self
    }
}
impl<'a> Deref for RenderingInputAttachmentIndexInfoBuilder<'a> {
    type Target = vk::RenderingInputAttachmentIndexInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl GraphicsPipelineCreateInfoNext for vk::RenderingInputAttachmentIndexInfo {}
impl CommandBufferInheritanceInfoNext for vk::RenderingInputAttachmentIndexInfo {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderQuadControlFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderQuadControlFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceMapMemoryPlacedFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceMapMemoryPlacedFeaturesEXT {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceMapMemoryPlacedPropertiesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct MemoryMapPlacedInfoEXTBuilder<'a> {
    inner: vk::MemoryMapPlacedInfoEXT,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::MemoryMapPlacedInfoEXT {
    type Type = MemoryMapPlacedInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> MemoryMapPlacedInfoEXTBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_placed_address(mut self, p_placed_address: *mut c_void) -> Self {
        self.inner.p_placed_address = p_placed_address;
        self
    }
}
impl<'a> Deref for MemoryMapPlacedInfoEXTBuilder<'a> {
    type Target = vk::MemoryMapPlacedInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryMapInfoNext for vk::MemoryMapPlacedInfoEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderBfloat16FeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderBfloat16FeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceRawAccessChainsFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceRawAccessChainsFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCommandBufferInheritanceFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCommandBufferInheritanceFeaturesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceImageAlignmentControlFeaturesMESA {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceImageAlignmentControlFeaturesMESA {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceImageAlignmentControlPropertiesMESA {}
impl ImageCreateInfoNext for vk::ImageAlignmentControlCreateInfoMESA {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderReplicatedCompositesFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderReplicatedCompositesFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentModeFifoLatestReadyFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentModeFifoLatestReadyFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCooperativeMatrix2FeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCooperativeMatrix2FeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCooperativeMatrix2PropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceHdrVividFeaturesHUAWEI {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceHdrVividFeaturesHUAWEI {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceVertexAttributeRobustnessFeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceVertexAttributeRobustnessFeaturesEXT {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDepthClampZeroOneFeaturesKHR {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDepthClampZeroOneFeaturesKHR {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceCooperativeVectorFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceCooperativeVectorFeaturesNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceCooperativeVectorPropertiesNV {}

#[repr(transparent)]
#[derive(Default)]
pub struct ConvertCooperativeVectorMatrixInfoNVBuilder<'a> {
    inner: vk::ConvertCooperativeVectorMatrixInfoNV,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::ConvertCooperativeVectorMatrixInfoNV {
    type Type = ConvertCooperativeVectorMatrixInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> ConvertCooperativeVectorMatrixInfoNVBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_size(mut self, src_size: usize) -> Self {
        self.inner.src_size = src_size;
        self
    }
    pub fn src_data(mut self, src_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.src_data = src_data;
        self
    }
    pub fn p_dst_size(mut self, p_dst_size: *mut usize) -> Self {
        self.inner.p_dst_size = p_dst_size;
        self
    }
    pub fn dst_data(mut self, dst_data: vk::DeviceOrHostAddressKHR) -> Self {
        self.inner.dst_data = dst_data;
        self
    }
    pub fn src_component_type(mut self, src_component_type: vk::ComponentTypeKHR) -> Self {
        self.inner.src_component_type = src_component_type;
        self
    }
    pub fn dst_component_type(mut self, dst_component_type: vk::ComponentTypeKHR) -> Self {
        self.inner.dst_component_type = dst_component_type;
        self
    }
    pub fn num_rows(mut self, num_rows: u32) -> Self {
        self.inner.num_rows = num_rows;
        self
    }
    pub fn num_columns(mut self, num_columns: u32) -> Self {
        self.inner.num_columns = num_columns;
        self
    }
    pub fn src_layout(mut self, src_layout: vk::CooperativeVectorMatrixLayoutNV) -> Self {
        self.inner.src_layout = src_layout;
        self
    }
    pub fn src_stride(mut self, src_stride: usize) -> Self {
        self.inner.src_stride = src_stride;
        self
    }
    pub fn dst_layout(mut self, dst_layout: vk::CooperativeVectorMatrixLayoutNV) -> Self {
        self.inner.dst_layout = dst_layout;
        self
    }
    pub fn dst_stride(mut self, dst_stride: usize) -> Self {
        self.inner.dst_stride = dst_stride;
        self
    }
}
impl<'a> Deref for ConvertCooperativeVectorMatrixInfoNVBuilder<'a> {
    type Target = vk::ConvertCooperativeVectorMatrixInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTileShadingFeaturesQCOM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTileShadingFeaturesQCOM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTileShadingPropertiesQCOM {}
impl RenderPassCreateInfoNext for vk::RenderPassTileShadingCreateInfoQCOM {}
impl RenderPassCreateInfo2Next for vk::RenderPassTileShadingCreateInfoQCOM {}
impl RenderingInfoNext for vk::RenderPassTileShadingCreateInfoQCOM {}
impl CommandBufferInheritanceInfoNext for vk::RenderPassTileShadingCreateInfoQCOM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE {}
impl GraphicsPipelineCreateInfoNext for vk::PipelineFragmentDensityMapLayeredCreateInfoVALVE {}
impl PresentInfoKHRNext for vk::SetPresentConfigNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDevicePresentMeteringFeaturesNV {}
impl DeviceCreateInfoNext for vk::PhysicalDevicePresentMeteringFeaturesNV {}
impl DeviceCreateInfoNext for vk::ExternalComputeQueueDeviceCreateInfoNV {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceExternalComputeQueuePropertiesNV {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceFormatPackFeaturesARM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceFormatPackFeaturesARM {}

#[repr(transparent)]
#[derive(Default)]
pub struct TensorDescriptionARMBuilder<'a> {
    inner: vk::TensorDescriptionARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::TensorDescriptionARM {
    type Type = TensorDescriptionARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> TensorDescriptionARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn tiling(mut self, tiling: vk::TensorTilingARM) -> Self {
        self.inner.tiling = tiling;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn p_dimensions(mut self, p_dimensions: &'a [i64], p_strides: Option<&'a [i64]>) -> Self {
        self.inner.dimension_count = p_dimensions.len() as u32;
        self.inner.p_dimensions = p_dimensions.as_ptr();
        self.inner.p_strides = p_strides.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn usage(mut self, usage: vk::TensorUsageFlagsARM) -> Self {
        self.inner.usage = usage;
        self
    }
}
impl<'a> Deref for TensorDescriptionARMBuilder<'a> {
    type Target = vk::TensorDescriptionARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct TensorCreateInfoARMBuilder<'a> {
    inner: vk::TensorCreateInfoARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::TensorCreateInfoARM {
    type Type = TensorCreateInfoARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait TensorCreateInfoARMNext {}
impl<'a> TensorCreateInfoARMBuilder<'a> {
    pub fn insert_next<T: TensorCreateInfoARMNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::TensorCreateFlagsARM) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_description(mut self, p_description: &'a vk::TensorDescriptionARM) -> Self {
        self.inner.p_description = p_description;
        self
    }
    pub fn sharing_mode(mut self, sharing_mode: vk::SharingMode) -> Self {
        self.inner.sharing_mode = sharing_mode;
        self
    }
    pub fn queue_family_index_count(mut self, queue_family_index_count: u32) -> Self {
        self.inner.queue_family_index_count = queue_family_index_count;
        self
    }
    pub fn p_queue_family_indices(mut self, p_queue_family_indices: &'a [u32]) -> Self {
        self.inner.queue_family_index_count = p_queue_family_indices.len() as u32;
        self.inner.p_queue_family_indices = p_queue_family_indices.as_ptr();
        self
    }
}
impl<'a> Deref for TensorCreateInfoARMBuilder<'a> {
    type Target = vk::TensorCreateInfoARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct TensorViewCreateInfoARMBuilder<'a> {
    inner: vk::TensorViewCreateInfoARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::TensorViewCreateInfoARM {
    type Type = TensorViewCreateInfoARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
pub trait TensorViewCreateInfoARMNext {}
impl<'a> TensorViewCreateInfoARMBuilder<'a> {
    pub fn insert_next<T: TensorViewCreateInfoARMNext>(mut self, next: &'a mut T) -> Self {
        unsafe {
            insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _);
        }
        self
    }
}
impl<'a> Deref for TensorViewCreateInfoARMBuilder<'a> {
    type Target = vk::TensorViewCreateInfoARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct WriteDescriptorSetTensorARMBuilder<'a> {
    inner: vk::WriteDescriptorSetTensorARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetTensorARM {
    type Type = WriteDescriptorSetTensorARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> WriteDescriptorSetTensorARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_tensor_views(mut self, p_tensor_views: &'a [vk::TensorViewARM]) -> Self {
        self.inner.tensor_view_count = p_tensor_views.len() as u32;
        self.inner.p_tensor_views = p_tensor_views.as_ptr();
        self
    }
}
impl<'a> Deref for WriteDescriptorSetTensorARMBuilder<'a> {
    type Target = vk::WriteDescriptorSetTensorARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl WriteDescriptorSetNext for vk::WriteDescriptorSetTensorARM {}
impl FormatProperties2Next for vk::TensorFormatPropertiesARM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceTensorPropertiesARM {}
impl DependencyInfoNext for vk::TensorMemoryBarrierARM {}

#[repr(transparent)]
#[derive(Default)]
pub struct TensorDependencyInfoARMBuilder<'a> {
    inner: vk::TensorDependencyInfoARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::TensorDependencyInfoARM {
    type Type = TensorDependencyInfoARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> TensorDependencyInfoARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn tensor_memory_barrier_count(mut self, tensor_memory_barrier_count: u32) -> Self {
        self.inner.tensor_memory_barrier_count = tensor_memory_barrier_count;
        self
    }
    pub fn p_tensor_memory_barriers(mut self, p_tensor_memory_barriers: &'a vk::TensorMemoryBarrierARM) -> Self {
        self.inner.p_tensor_memory_barriers = p_tensor_memory_barriers;
        self
    }
}
impl<'a> Deref for TensorDependencyInfoARMBuilder<'a> {
    type Target = vk::TensorDependencyInfoARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl DependencyInfoNext for vk::TensorDependencyInfoARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceTensorFeaturesARM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceTensorFeaturesARM {}

#[repr(transparent)]
#[derive(Default)]
pub struct DeviceTensorMemoryRequirementsARMBuilder<'a> {
    inner: vk::DeviceTensorMemoryRequirementsARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::DeviceTensorMemoryRequirementsARM {
    type Type = DeviceTensorMemoryRequirementsARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> DeviceTensorMemoryRequirementsARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_create_info(mut self, p_create_info: &'a vk::TensorCreateInfoARM) -> Self {
        self.inner.p_create_info = p_create_info;
        self
    }
}
impl<'a> Deref for DeviceTensorMemoryRequirementsARMBuilder<'a> {
    type Target = vk::DeviceTensorMemoryRequirementsARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct CopyTensorInfoARMBuilder<'a> {
    inner: vk::CopyTensorInfoARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::CopyTensorInfoARM {
    type Type = CopyTensorInfoARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> CopyTensorInfoARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_tensor(mut self, src_tensor: vk::TensorARM) -> Self {
        self.inner.src_tensor = src_tensor;
        self
    }
    pub fn dst_tensor(mut self, dst_tensor: vk::TensorARM) -> Self {
        self.inner.dst_tensor = dst_tensor;
        self
    }
    pub fn p_regions(mut self, p_regions: &'a [vk::TensorCopyARM]) -> Self {
        self.inner.region_count = p_regions.len() as u32;
        self.inner.p_regions = p_regions.as_ptr();
        self
    }
}
impl<'a> Deref for CopyTensorInfoARMBuilder<'a> {
    type Target = vk::CopyTensorInfoARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct TensorCopyARMBuilder<'a> {
    inner: vk::TensorCopyARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::TensorCopyARM {
    type Type = TensorCopyARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> TensorCopyARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_src_offset(
        mut self,
        p_src_offset: Option<&'a [u64]>,
        p_dst_offset: Option<&'a [u64]>,
        p_extent: Option<&'a [u64]>,
    ) -> Self {
        self.inner.dimension_count = p_src_offset
            .map(|s| s.len() as u32)
            .or(p_dst_offset.map(|s| s.len() as u32))
            .or(p_extent.map(|s| s.len() as u32))
            .unwrap_or(0);
        if let Some(len) = p_src_offset.map(|s| s.len()) {
            assert_eq!(self.inner.dimension_count, len as u32);
        }
        if let Some(len) = p_dst_offset.map(|s| s.len()) {
            assert_eq!(self.inner.dimension_count, len as u32);
        }
        if let Some(len) = p_extent.map(|s| s.len()) {
            assert_eq!(self.inner.dimension_count, len as u32);
        }
        self.inner.p_src_offset = p_src_offset.map_or(ptr::null(), |s| s.as_ptr());
        self.inner.p_dst_offset = p_dst_offset.map_or(ptr::null(), |s| s.as_ptr());
        self.inner.p_extent = p_extent.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
}
impl<'a> Deref for TensorCopyARMBuilder<'a> {
    type Target = vk::TensorCopyARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl MemoryAllocateInfoNext for vk::MemoryDedicatedAllocateInfoTensorARM {}
impl PhysicalDeviceProperties2Next for vk::PhysicalDeviceDescriptorBufferTensorPropertiesARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceDescriptorBufferTensorFeaturesARM {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceDescriptorBufferTensorFeaturesARM {}
impl DescriptorGetInfoEXTNext for vk::DescriptorGetTensorInfoARM {}

#[repr(transparent)]
#[derive(Default)]
pub struct FrameBoundaryTensorsARMBuilder<'a> {
    inner: vk::FrameBoundaryTensorsARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::FrameBoundaryTensorsARM {
    type Type = FrameBoundaryTensorsARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> FrameBoundaryTensorsARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_tensors(mut self, p_tensors: &'a [vk::TensorARM]) -> Self {
        self.inner.tensor_count = p_tensors.len() as u32;
        self.inner.p_tensors = p_tensors.as_ptr();
        self
    }
}
impl<'a> Deref for FrameBoundaryTensorsARMBuilder<'a> {
    type Target = vk::FrameBoundaryTensorsARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl SubmitInfoNext for vk::FrameBoundaryTensorsARM {}
impl SubmitInfo2Next for vk::FrameBoundaryTensorsARM {}
impl PresentInfoKHRNext for vk::FrameBoundaryTensorsARM {}
impl BindSparseInfoNext for vk::FrameBoundaryTensorsARM {}

#[repr(transparent)]
#[derive(Default)]
pub struct PhysicalDeviceExternalTensorInfoARMBuilder<'a> {
    inner: vk::PhysicalDeviceExternalTensorInfoARM,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::PhysicalDeviceExternalTensorInfoARM {
    type Type = PhysicalDeviceExternalTensorInfoARMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> PhysicalDeviceExternalTensorInfoARMBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::TensorCreateFlagsARM) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn p_description(mut self, p_description: &'a vk::TensorDescriptionARM) -> Self {
        self.inner.p_description = p_description;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for PhysicalDeviceExternalTensorInfoARMBuilder<'a> {
    type Target = vk::PhysicalDeviceExternalTensorInfoARM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl TensorCreateInfoARMNext for vk::ExternalMemoryTensorCreateInfoARM {}
impl PhysicalDeviceFeatures2Next for vk::PhysicalDeviceShaderFloat8FeaturesEXT {}
impl DeviceCreateInfoNext for vk::PhysicalDeviceShaderFloat8FeaturesEXT {}

#[repr(transparent)]
#[derive(Default)]
pub struct OHSurfaceCreateInfoOHOSBuilder<'a> {
    inner: vk::OHSurfaceCreateInfoOHOS,
    phantom: PhantomData<&'a ()>,
}
impl<'a> Builder<'a> for vk::OHSurfaceCreateInfoOHOS {
    type Type = OHSurfaceCreateInfoOHOSBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
impl<'a> OHSurfaceCreateInfoOHOSBuilder<'a> {
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SurfaceCreateFlagsOHOS) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn window(mut self, window: *mut vk::OHNativeWindow) -> Self {
        self.inner.window = window;
        self
    }
}
impl<'a> Deref for OHSurfaceCreateInfoOHOSBuilder<'a> {
    type Target = vk::OHSurfaceCreateInfoOHOS;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
