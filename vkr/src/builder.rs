#![allow(clippy::wrong_self_convention)]

use super::vk;
use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::Deref;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

pub trait Builder<'a> {
    type Type;
    fn builder() -> Self::Type;
}

impl<'a> Builder<'a> for vk::BaseOutStructure {
    type Type = BaseOutStructureBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BaseOutStructureBuilder<'a> {
    inner: vk::BaseOutStructure,
    phantom: PhantomData<&'a vk::BaseOutStructure>,
}
impl<'a> BaseOutStructureBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut vk::BaseOutStructure) -> Self {
        self.inner.p_next = p_next;
        self
    }
}
impl<'a> Deref for BaseOutStructureBuilder<'a> {
    type Target = vk::BaseOutStructure;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BaseInStructure {
    type Type = BaseInStructureBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BaseInStructureBuilder<'a> {
    inner: vk::BaseInStructure,
    phantom: PhantomData<&'a vk::BaseInStructure>,
}
impl<'a> BaseInStructureBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: &'a vk::BaseInStructure) -> Self {
        self.inner.p_next = p_next;
        self
    }
}
impl<'a> Deref for BaseInStructureBuilder<'a> {
    type Target = vk::BaseInStructure;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ApplicationInfo {
    type Type = ApplicationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ApplicationInfoBuilder<'a> {
    inner: vk::ApplicationInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ApplicationInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_application_name(mut self, p_application_name: &'a CStr) -> Self {
        self.inner.p_application_name = p_application_name.as_ptr();
        self
    }
    pub fn application_version(mut self, application_version: u32) -> Self {
        self.inner.application_version = application_version;
        self
    }
    pub fn p_engine_name(mut self, p_engine_name: &'a CStr) -> Self {
        self.inner.p_engine_name = p_engine_name.as_ptr();
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
impl<'a> Builder<'a> for vk::AllocationCallbacks {
    type Type = AllocationCallbacksBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AllocationCallbacksBuilder<'a> {
    inner: vk::AllocationCallbacks,
    phantom: PhantomData<&'a c_void>,
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
impl<'a> Builder<'a> for vk::DeviceQueueCreateInfo {
    type Type = DeviceQueueCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceQueueCreateInfoBuilder<'a> {
    inner: vk::DeviceQueueCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceQueueCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::DeviceCreateInfo {
    type Type = DeviceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceCreateInfoBuilder<'a> {
    inner: vk::DeviceCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
        self.inner.p_enabled_features = p_enabled_features.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for DeviceCreateInfoBuilder<'a> {
    type Target = vk::DeviceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::InstanceCreateInfo {
    type Type = InstanceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct InstanceCreateInfoBuilder<'a> {
    inner: vk::InstanceCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> InstanceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
        self.inner.p_application_info = p_application_info.map_or(ptr::null(), |p| p);
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
impl<'a> Builder<'a> for vk::MemoryAllocateInfo {
    type Type = MemoryAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryAllocateInfoBuilder<'a> {
    inner: vk::MemoryAllocateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn allocation_size(mut self, allocation_size: vk::DeviceSize) -> Self {
        self.inner.allocation_size = allocation_size;
        self
    }
    pub fn memory_type_index(mut self, memory_type_index: u32) -> Self {
        self.inner.memory_type_index = memory_type_index;
        self
    }
}
impl<'a> Deref for MemoryAllocateInfoBuilder<'a> {
    type Target = vk::MemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MappedMemoryRange {
    type Type = MappedMemoryRangeBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MappedMemoryRangeBuilder<'a> {
    inner: vk::MappedMemoryRange,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MappedMemoryRangeBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl<'a> Deref for MappedMemoryRangeBuilder<'a> {
    type Target = vk::MappedMemoryRange;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSet {
    type Type = WriteDescriptorSetBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetBuilder<'a> {
    inner: vk::WriteDescriptorSet,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> WriteDescriptorSetBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dst_set(mut self, dst_set: vk::DescriptorSet) -> Self {
        self.inner.dst_set = Some(dst_set);
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
impl<'a> Builder<'a> for vk::CopyDescriptorSet {
    type Type = CopyDescriptorSetBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyDescriptorSetBuilder<'a> {
    inner: vk::CopyDescriptorSet,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CopyDescriptorSetBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_set(mut self, src_set: vk::DescriptorSet) -> Self {
        self.inner.src_set = Some(src_set);
        self
    }
    pub fn src_binding(mut self, src_binding: u32) -> Self {
        self.inner.src_binding = src_binding;
        self
    }
    pub fn src_array_element(mut self, src_array_element: u32) -> Self {
        self.inner.src_array_element = src_array_element;
        self
    }
    pub fn dst_set(mut self, dst_set: vk::DescriptorSet) -> Self {
        self.inner.dst_set = Some(dst_set);
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
}
impl<'a> Deref for CopyDescriptorSetBuilder<'a> {
    type Target = vk::CopyDescriptorSet;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferCreateInfo {
    type Type = BufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferCreateInfoBuilder<'a> {
    inner: vk::BufferCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BufferCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::BufferViewCreateInfo {
    type Type = BufferViewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferViewCreateInfoBuilder<'a> {
    inner: vk::BufferViewCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BufferViewCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::BufferViewCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn range(mut self, range: vk::DeviceSize) -> Self {
        self.inner.range = range;
        self
    }
}
impl<'a> Deref for BufferViewCreateInfoBuilder<'a> {
    type Target = vk::BufferViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MemoryBarrier {
    type Type = MemoryBarrierBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryBarrierBuilder<'a> {
    inner: vk::MemoryBarrier,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryBarrierBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
}
impl<'a> Deref for MemoryBarrierBuilder<'a> {
    type Target = vk::MemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferMemoryBarrier {
    type Type = BufferMemoryBarrierBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferMemoryBarrierBuilder<'a> {
    inner: vk::BufferMemoryBarrier,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BufferMemoryBarrierBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn size(mut self, size: vk::DeviceSize) -> Self {
        self.inner.size = size;
        self
    }
}
impl<'a> Deref for BufferMemoryBarrierBuilder<'a> {
    type Target = vk::BufferMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageMemoryBarrier {
    type Type = ImageMemoryBarrierBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageMemoryBarrierBuilder<'a> {
    inner: vk::ImageMemoryBarrier,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageMemoryBarrierBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn old_layout(mut self, old_layout: vk::ImageLayout) -> Self {
        self.inner.old_layout = old_layout;
        self
    }
    pub fn new_layout(mut self, new_layout: vk::ImageLayout) -> Self {
        self.inner.new_layout = new_layout;
        self
    }
    pub fn src_queue_family_index(mut self, src_queue_family_index: u32) -> Self {
        self.inner.src_queue_family_index = src_queue_family_index;
        self
    }
    pub fn dst_queue_family_index(mut self, dst_queue_family_index: u32) -> Self {
        self.inner.dst_queue_family_index = dst_queue_family_index;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn subresource_range(mut self, subresource_range: vk::ImageSubresourceRange) -> Self {
        self.inner.subresource_range = subresource_range;
        self
    }
}
impl<'a> Deref for ImageMemoryBarrierBuilder<'a> {
    type Target = vk::ImageMemoryBarrier;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageCreateInfo {
    type Type = ImageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageCreateInfoBuilder<'a> {
    inner: vk::ImageCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::ImageViewCreateInfo {
    type Type = ImageViewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewCreateInfoBuilder<'a> {
    inner: vk::ImageViewCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageViewCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImageViewCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn view_type(mut self, view_type: vk::ImageViewType) -> Self {
        self.inner.view_type = view_type;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn components(mut self, components: vk::ComponentMapping) -> Self {
        self.inner.components = components;
        self
    }
    pub fn subresource_range(mut self, subresource_range: vk::ImageSubresourceRange) -> Self {
        self.inner.subresource_range = subresource_range;
        self
    }
}
impl<'a> Deref for ImageViewCreateInfoBuilder<'a> {
    type Target = vk::ImageViewCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SparseBufferMemoryBindInfo {
    type Type = SparseBufferMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseBufferMemoryBindInfoBuilder<'a> {
    inner: vk::SparseBufferMemoryBindInfo,
    phantom: PhantomData<&'a vk::SparseMemoryBind>,
}
impl<'a> SparseBufferMemoryBindInfoBuilder<'a> {
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
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
impl<'a> Builder<'a> for vk::SparseImageOpaqueMemoryBindInfo {
    type Type = SparseImageOpaqueMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    inner: vk::SparseImageOpaqueMemoryBindInfo,
    phantom: PhantomData<&'a vk::SparseMemoryBind>,
}
impl<'a> SparseImageOpaqueMemoryBindInfoBuilder<'a> {
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
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
impl<'a> Builder<'a> for vk::SparseImageMemoryBindInfo {
    type Type = SparseImageMemoryBindInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SparseImageMemoryBindInfoBuilder<'a> {
    inner: vk::SparseImageMemoryBindInfo,
    phantom: PhantomData<&'a vk::SparseImageMemoryBind>,
}
impl<'a> SparseImageMemoryBindInfoBuilder<'a> {
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
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
impl<'a> Builder<'a> for vk::BindSparseInfo {
    type Type = BindSparseInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindSparseInfoBuilder<'a> {
    inner: vk::BindSparseInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BindSparseInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::ShaderModuleCreateInfo {
    type Type = ShaderModuleCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ShaderModuleCreateInfoBuilder<'a> {
    inner: vk::ShaderModuleCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ShaderModuleCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::DescriptorSetLayoutBinding {
    type Type = DescriptorSetLayoutBindingBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutBindingBuilder<'a> {
    inner: vk::DescriptorSetLayoutBinding,
    phantom: PhantomData<&'a Option<vk::Sampler>>,
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
impl<'a> Builder<'a> for vk::DescriptorSetLayoutCreateInfo {
    type Type = DescriptorSetLayoutCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutCreateInfoBuilder<'a> {
    inner: vk::DescriptorSetLayoutCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DescriptorSetLayoutCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::DescriptorPoolCreateInfo {
    type Type = DescriptorPoolCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorPoolCreateInfoBuilder<'a> {
    inner: vk::DescriptorPoolCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DescriptorPoolCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::DescriptorSetAllocateInfo {
    type Type = DescriptorSetAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetAllocateInfoBuilder<'a> {
    inner: vk::DescriptorSetAllocateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DescriptorSetAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn descriptor_pool(mut self, descriptor_pool: vk::DescriptorPool) -> Self {
        self.inner.descriptor_pool = Some(descriptor_pool);
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
impl<'a> Builder<'a> for vk::SpecializationInfo {
    type Type = SpecializationInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SpecializationInfoBuilder<'a> {
    inner: vk::SpecializationInfo,
    phantom: PhantomData<&'a vk::SpecializationMapEntry>,
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
impl<'a> Builder<'a> for vk::PipelineShaderStageCreateInfo {
    type Type = PipelineShaderStageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineShaderStageCreateInfoBuilder<'a> {
    inner: vk::PipelineShaderStageCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineShaderStageCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
        self.inner.module = Some(module);
        self
    }
    pub fn p_name(mut self, p_name: &'a CStr) -> Self {
        self.inner.p_name = p_name.as_ptr();
        self
    }
    pub fn p_specialization_info(mut self, p_specialization_info: Option<&'a vk::SpecializationInfo>) -> Self {
        self.inner.p_specialization_info = p_specialization_info.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for PipelineShaderStageCreateInfoBuilder<'a> {
    type Target = vk::PipelineShaderStageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ComputePipelineCreateInfo {
    type Type = ComputePipelineCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ComputePipelineCreateInfoBuilder<'a> {
    inner: vk::ComputePipelineCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ComputePipelineCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
    pub fn stage(mut self, stage: vk::PipelineShaderStageCreateInfo) -> Self {
        self.inner.stage = stage;
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
        self.inner.base_pipeline_handle = base_pipeline_handle;
        self
    }
    pub fn base_pipeline_index(mut self, base_pipeline_index: i32) -> Self {
        self.inner.base_pipeline_index = base_pipeline_index;
        self
    }
}
impl<'a> Deref for ComputePipelineCreateInfoBuilder<'a> {
    type Target = vk::ComputePipelineCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineVertexInputStateCreateInfo {
    type Type = PipelineVertexInputStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineVertexInputStateCreateInfoBuilder<'a> {
    inner: vk::PipelineVertexInputStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineVertexInputStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::PipelineInputAssemblyStateCreateInfo {
    type Type = PipelineInputAssemblyStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineInputAssemblyStateCreateInfoBuilder<'a> {
    inner: vk::PipelineInputAssemblyStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineInputAssemblyStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineInputAssemblyStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn topology(mut self, topology: vk::PrimitiveTopology) -> Self {
        self.inner.topology = topology;
        self
    }
    pub fn primitive_restart_enable(mut self, primitive_restart_enable: bool) -> Self {
        self.inner.primitive_restart_enable = if primitive_restart_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PipelineInputAssemblyStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineInputAssemblyStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineTessellationStateCreateInfo {
    type Type = PipelineTessellationStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineTessellationStateCreateInfoBuilder<'a> {
    inner: vk::PipelineTessellationStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineTessellationStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineTessellationStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn patch_control_points(mut self, patch_control_points: u32) -> Self {
        self.inner.patch_control_points = patch_control_points;
        self
    }
}
impl<'a> Deref for PipelineTessellationStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineTessellationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportStateCreateInfo {
    type Type = PipelineViewportStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportStateCreateInfoBuilder<'a> {
    inner: vk::PipelineViewportStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineViewportStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::PipelineRasterizationStateCreateInfo {
    type Type = PipelineRasterizationStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationStateCreateInfoBuilder<'a> {
    inner: vk::PipelineRasterizationStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineRasterizationStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_clamp_enable(mut self, depth_clamp_enable: bool) -> Self {
        self.inner.depth_clamp_enable = if depth_clamp_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn rasterizer_discard_enable(mut self, rasterizer_discard_enable: bool) -> Self {
        self.inner.rasterizer_discard_enable = if rasterizer_discard_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn polygon_mode(mut self, polygon_mode: vk::PolygonMode) -> Self {
        self.inner.polygon_mode = polygon_mode;
        self
    }
    pub fn cull_mode(mut self, cull_mode: vk::CullModeFlags) -> Self {
        self.inner.cull_mode = cull_mode;
        self
    }
    pub fn front_face(mut self, front_face: vk::FrontFace) -> Self {
        self.inner.front_face = front_face;
        self
    }
    pub fn depth_bias_enable(mut self, depth_bias_enable: bool) -> Self {
        self.inner.depth_bias_enable = if depth_bias_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_bias_constant_factor(mut self, depth_bias_constant_factor: f32) -> Self {
        self.inner.depth_bias_constant_factor = depth_bias_constant_factor;
        self
    }
    pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.inner.depth_bias_clamp = depth_bias_clamp;
        self
    }
    pub fn depth_bias_slope_factor(mut self, depth_bias_slope_factor: f32) -> Self {
        self.inner.depth_bias_slope_factor = depth_bias_slope_factor;
        self
    }
    pub fn line_width(mut self, line_width: f32) -> Self {
        self.inner.line_width = line_width;
        self
    }
}
impl<'a> Deref for PipelineRasterizationStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineRasterizationStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineMultisampleStateCreateInfo {
    type Type = PipelineMultisampleStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineMultisampleStateCreateInfoBuilder<'a> {
    inner: vk::PipelineMultisampleStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineMultisampleStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::PipelineColorBlendStateCreateInfo {
    type Type = PipelineColorBlendStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineColorBlendStateCreateInfoBuilder<'a> {
    inner: vk::PipelineColorBlendStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineColorBlendStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
}
impl<'a> Deref for PipelineColorBlendStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineColorBlendStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineDynamicStateCreateInfo {
    type Type = PipelineDynamicStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDynamicStateCreateInfoBuilder<'a> {
    inner: vk::PipelineDynamicStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineDynamicStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PipelineDepthStencilStateCreateInfo {
    type Type = PipelineDepthStencilStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDepthStencilStateCreateInfoBuilder<'a> {
    inner: vk::PipelineDepthStencilStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineDepthStencilStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineDepthStencilStateCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_test_enable(mut self, depth_test_enable: bool) -> Self {
        self.inner.depth_test_enable = if depth_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_write_enable(mut self, depth_write_enable: bool) -> Self {
        self.inner.depth_write_enable = if depth_write_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn depth_compare_op(mut self, depth_compare_op: vk::CompareOp) -> Self {
        self.inner.depth_compare_op = depth_compare_op;
        self
    }
    pub fn depth_bounds_test_enable(mut self, depth_bounds_test_enable: bool) -> Self {
        self.inner.depth_bounds_test_enable = if depth_bounds_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stencil_test_enable(mut self, stencil_test_enable: bool) -> Self {
        self.inner.stencil_test_enable = if stencil_test_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn front(mut self, front: vk::StencilOpState) -> Self {
        self.inner.front = front;
        self
    }
    pub fn back(mut self, back: vk::StencilOpState) -> Self {
        self.inner.back = back;
        self
    }
    pub fn min_depth_bounds(mut self, min_depth_bounds: f32) -> Self {
        self.inner.min_depth_bounds = min_depth_bounds;
        self
    }
    pub fn max_depth_bounds(mut self, max_depth_bounds: f32) -> Self {
        self.inner.max_depth_bounds = max_depth_bounds;
        self
    }
}
impl<'a> Deref for PipelineDepthStencilStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineDepthStencilStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GraphicsPipelineCreateInfo {
    type Type = GraphicsPipelineCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsPipelineCreateInfoBuilder<'a> {
    inner: vk::GraphicsPipelineCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> GraphicsPipelineCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
    pub fn p_vertex_input_state(
        mut self,
        p_vertex_input_state: Option<&'a vk::PipelineVertexInputStateCreateInfo>,
    ) -> Self {
        self.inner.p_vertex_input_state = p_vertex_input_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_input_assembly_state(
        mut self,
        p_input_assembly_state: Option<&'a vk::PipelineInputAssemblyStateCreateInfo>,
    ) -> Self {
        self.inner.p_input_assembly_state = p_input_assembly_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_tessellation_state(
        mut self,
        p_tessellation_state: Option<&'a vk::PipelineTessellationStateCreateInfo>,
    ) -> Self {
        self.inner.p_tessellation_state = p_tessellation_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_viewport_state(mut self, p_viewport_state: Option<&'a vk::PipelineViewportStateCreateInfo>) -> Self {
        self.inner.p_viewport_state = p_viewport_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_rasterization_state(
        mut self,
        p_rasterization_state: &'a vk::PipelineRasterizationStateCreateInfo,
    ) -> Self {
        self.inner.p_rasterization_state = p_rasterization_state;
        self
    }
    pub fn p_multisample_state(
        mut self,
        p_multisample_state: Option<&'a vk::PipelineMultisampleStateCreateInfo>,
    ) -> Self {
        self.inner.p_multisample_state = p_multisample_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_depth_stencil_state(
        mut self,
        p_depth_stencil_state: Option<&'a vk::PipelineDepthStencilStateCreateInfo>,
    ) -> Self {
        self.inner.p_depth_stencil_state = p_depth_stencil_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_color_blend_state(
        mut self,
        p_color_blend_state: Option<&'a vk::PipelineColorBlendStateCreateInfo>,
    ) -> Self {
        self.inner.p_color_blend_state = p_color_blend_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_dynamic_state(mut self, p_dynamic_state: Option<&'a vk::PipelineDynamicStateCreateInfo>) -> Self {
        self.inner.p_dynamic_state = p_dynamic_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = Some(render_pass);
        self
    }
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.inner.subpass = subpass;
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
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
impl<'a> Builder<'a> for vk::PipelineCacheCreateInfo {
    type Type = PipelineCacheCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCacheCreateInfoBuilder<'a> {
    inner: vk::PipelineCacheCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineCacheCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PipelineLayoutCreateInfo {
    type Type = PipelineLayoutCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineLayoutCreateInfoBuilder<'a> {
    inner: vk::PipelineLayoutCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineLayoutCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::SamplerCreateInfo {
    type Type = SamplerCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerCreateInfoBuilder<'a> {
    inner: vk::SamplerCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SamplerCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SamplerCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn mag_filter(mut self, mag_filter: vk::Filter) -> Self {
        self.inner.mag_filter = mag_filter;
        self
    }
    pub fn min_filter(mut self, min_filter: vk::Filter) -> Self {
        self.inner.min_filter = min_filter;
        self
    }
    pub fn mipmap_mode(mut self, mipmap_mode: vk::SamplerMipmapMode) -> Self {
        self.inner.mipmap_mode = mipmap_mode;
        self
    }
    pub fn address_mode_u(mut self, address_mode_u: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_u = address_mode_u;
        self
    }
    pub fn address_mode_v(mut self, address_mode_v: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_v = address_mode_v;
        self
    }
    pub fn address_mode_w(mut self, address_mode_w: vk::SamplerAddressMode) -> Self {
        self.inner.address_mode_w = address_mode_w;
        self
    }
    pub fn mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.inner.mip_lod_bias = mip_lod_bias;
        self
    }
    pub fn anisotropy_enable(mut self, anisotropy_enable: bool) -> Self {
        self.inner.anisotropy_enable = if anisotropy_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn max_anisotropy(mut self, max_anisotropy: f32) -> Self {
        self.inner.max_anisotropy = max_anisotropy;
        self
    }
    pub fn compare_enable(mut self, compare_enable: bool) -> Self {
        self.inner.compare_enable = if compare_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn compare_op(mut self, compare_op: vk::CompareOp) -> Self {
        self.inner.compare_op = compare_op;
        self
    }
    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.inner.min_lod = min_lod;
        self
    }
    pub fn max_lod(mut self, max_lod: f32) -> Self {
        self.inner.max_lod = max_lod;
        self
    }
    pub fn border_color(mut self, border_color: vk::BorderColor) -> Self {
        self.inner.border_color = border_color;
        self
    }
    pub fn unnormalized_coordinates(mut self, unnormalized_coordinates: bool) -> Self {
        self.inner.unnormalized_coordinates = if unnormalized_coordinates { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for SamplerCreateInfoBuilder<'a> {
    type Target = vk::SamplerCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandPoolCreateInfo {
    type Type = CommandPoolCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandPoolCreateInfoBuilder<'a> {
    inner: vk::CommandPoolCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CommandPoolCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::CommandPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn queue_family_index(mut self, queue_family_index: u32) -> Self {
        self.inner.queue_family_index = queue_family_index;
        self
    }
}
impl<'a> Deref for CommandPoolCreateInfoBuilder<'a> {
    type Target = vk::CommandPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandBufferAllocateInfo {
    type Type = CommandBufferAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferAllocateInfoBuilder<'a> {
    inner: vk::CommandBufferAllocateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CommandBufferAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn command_pool(mut self, command_pool: vk::CommandPool) -> Self {
        self.inner.command_pool = Some(command_pool);
        self
    }
    pub fn level(mut self, level: vk::CommandBufferLevel) -> Self {
        self.inner.level = level;
        self
    }
    pub fn command_buffer_count(mut self, command_buffer_count: u32) -> Self {
        self.inner.command_buffer_count = command_buffer_count;
        self
    }
}
impl<'a> Deref for CommandBufferAllocateInfoBuilder<'a> {
    type Target = vk::CommandBufferAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceInfo {
    type Type = CommandBufferInheritanceInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceInfoBuilder<'a> {
    inner: vk::CommandBufferInheritanceInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CommandBufferInheritanceInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn render_pass(mut self, render_pass: Option<vk::RenderPass>) -> Self {
        self.inner.render_pass = render_pass;
        self
    }
    pub fn subpass(mut self, subpass: u32) -> Self {
        self.inner.subpass = subpass;
        self
    }
    pub fn framebuffer(mut self, framebuffer: Option<vk::Framebuffer>) -> Self {
        self.inner.framebuffer = framebuffer;
        self
    }
    pub fn occlusion_query_enable(mut self, occlusion_query_enable: bool) -> Self {
        self.inner.occlusion_query_enable = if occlusion_query_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn query_flags(mut self, query_flags: vk::QueryControlFlags) -> Self {
        self.inner.query_flags = query_flags;
        self
    }
    pub fn pipeline_statistics(mut self, pipeline_statistics: vk::QueryPipelineStatisticFlags) -> Self {
        self.inner.pipeline_statistics = pipeline_statistics;
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceInfoBuilder<'a> {
    type Target = vk::CommandBufferInheritanceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandBufferBeginInfo {
    type Type = CommandBufferBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferBeginInfoBuilder<'a> {
    inner: vk::CommandBufferBeginInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CommandBufferBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
        self.inner.p_inheritance_info = p_inheritance_info.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for CommandBufferBeginInfoBuilder<'a> {
    type Target = vk::CommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassBeginInfo {
    type Type = RenderPassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassBeginInfoBuilder<'a> {
    inner: vk::RenderPassBeginInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn render_pass(mut self, render_pass: vk::RenderPass) -> Self {
        self.inner.render_pass = Some(render_pass);
        self
    }
    pub fn framebuffer(mut self, framebuffer: vk::Framebuffer) -> Self {
        self.inner.framebuffer = Some(framebuffer);
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
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
impl<'a> Builder<'a> for vk::SubpassDescription {
    type Type = SubpassDescriptionBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDescriptionBuilder<'a> {
    inner: vk::SubpassDescription,
    phantom: PhantomData<&'a vk::AttachmentReference>,
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
        if let Some(s) = p_resolve_attachments {
            assert_eq!(self.inner.color_attachment_count, s.len() as u32);
        }
        self.inner.p_color_attachments = p_color_attachments.as_ptr();
        self.inner.p_resolve_attachments = p_resolve_attachments.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn p_depth_stencil_attachment(
        mut self,
        p_depth_stencil_attachment: Option<&'a vk::AttachmentReference>,
    ) -> Self {
        self.inner.p_depth_stencil_attachment = p_depth_stencil_attachment.map_or(ptr::null(), |p| p);
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
impl<'a> Builder<'a> for vk::RenderPassCreateInfo {
    type Type = RenderPassCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassCreateInfoBuilder<'a> {
    inner: vk::RenderPassCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::EventCreateInfo {
    type Type = EventCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct EventCreateInfoBuilder<'a> {
    inner: vk::EventCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> EventCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::EventCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for EventCreateInfoBuilder<'a> {
    type Target = vk::EventCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FenceCreateInfo {
    type Type = FenceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FenceCreateInfoBuilder<'a> {
    inner: vk::FenceCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> FenceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::FenceCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for FenceCreateInfoBuilder<'a> {
    type Target = vk::FenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SemaphoreCreateInfo {
    type Type = SemaphoreCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreCreateInfoBuilder<'a> {
    inner: vk::SemaphoreCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SemaphoreCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for SemaphoreCreateInfoBuilder<'a> {
    type Target = vk::SemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::QueryPoolCreateInfo {
    type Type = QueryPoolCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueryPoolCreateInfoBuilder<'a> {
    inner: vk::QueryPoolCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> QueryPoolCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::QueryPoolCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn query_type(mut self, query_type: vk::QueryType) -> Self {
        self.inner.query_type = query_type;
        self
    }
    pub fn query_count(mut self, query_count: u32) -> Self {
        self.inner.query_count = query_count;
        self
    }
    pub fn pipeline_statistics(mut self, pipeline_statistics: vk::QueryPipelineStatisticFlags) -> Self {
        self.inner.pipeline_statistics = pipeline_statistics;
        self
    }
}
impl<'a> Deref for QueryPoolCreateInfoBuilder<'a> {
    type Target = vk::QueryPoolCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FramebufferCreateInfo {
    type Type = FramebufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FramebufferCreateInfoBuilder<'a> {
    inner: vk::FramebufferCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> FramebufferCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
        self.inner.render_pass = Some(render_pass);
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
impl<'a> Builder<'a> for vk::SubmitInfo {
    type Type = SubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubmitInfoBuilder<'a> {
    inner: vk::SubmitInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::DisplayModeCreateInfoKHR {
    type Type = DisplayModeCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayModeCreateInfoKHRBuilder<'a> {
    inner: vk::DisplayModeCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DisplayModeCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DisplayModeCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn parameters(mut self, parameters: vk::DisplayModeParametersKHR) -> Self {
        self.inner.parameters = parameters;
        self
    }
}
impl<'a> Deref for DisplayModeCreateInfoKHRBuilder<'a> {
    type Target = vk::DisplayModeCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DisplaySurfaceCreateInfoKHR {
    type Type = DisplaySurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplaySurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::DisplaySurfaceCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DisplaySurfaceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DisplaySurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn display_mode(mut self, display_mode: vk::DisplayModeKHR) -> Self {
        self.inner.display_mode = Some(display_mode);
        self
    }
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.inner.plane_index = plane_index;
        self
    }
    pub fn plane_stack_index(mut self, plane_stack_index: u32) -> Self {
        self.inner.plane_stack_index = plane_stack_index;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
    pub fn global_alpha(mut self, global_alpha: f32) -> Self {
        self.inner.global_alpha = global_alpha;
        self
    }
    pub fn alpha_mode(mut self, alpha_mode: vk::DisplayPlaneAlphaFlagsKHR) -> Self {
        self.inner.alpha_mode = alpha_mode;
        self
    }
    pub fn image_extent(mut self, image_extent: vk::Extent2D) -> Self {
        self.inner.image_extent = image_extent;
        self
    }
}
impl<'a> Deref for DisplaySurfaceCreateInfoKHRBuilder<'a> {
    type Target = vk::DisplaySurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DisplayPresentInfoKHR {
    type Type = DisplayPresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPresentInfoKHRBuilder<'a> {
    inner: vk::DisplayPresentInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DisplayPresentInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_rect(mut self, src_rect: vk::Rect2D) -> Self {
        self.inner.src_rect = src_rect;
        self
    }
    pub fn dst_rect(mut self, dst_rect: vk::Rect2D) -> Self {
        self.inner.dst_rect = dst_rect;
        self
    }
    pub fn persistent(mut self, persistent: bool) -> Self {
        self.inner.persistent = if persistent { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for DisplayPresentInfoKHRBuilder<'a> {
    type Target = vk::DisplayPresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AndroidSurfaceCreateInfoKHR {
    type Type = AndroidSurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AndroidSurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::AndroidSurfaceCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AndroidSurfaceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ViSurfaceCreateInfoNN {
    type Type = ViSurfaceCreateInfoNNBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ViSurfaceCreateInfoNNBuilder<'a> {
    inner: vk::ViSurfaceCreateInfoNN,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ViSurfaceCreateInfoNNBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::WaylandSurfaceCreateInfoKHR {
    type Type = WaylandSurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WaylandSurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::WaylandSurfaceCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> WaylandSurfaceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::Win32SurfaceCreateInfoKHR {
    type Type = Win32SurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32SurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::Win32SurfaceCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> Win32SurfaceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::Win32SurfaceCreateFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn hinstance(mut self, hinstance: vk::HINSTANCE) -> Self {
        self.inner.hinstance = hinstance;
        self
    }
    pub fn hwnd(mut self, hwnd: vk::HWND) -> Self {
        self.inner.hwnd = hwnd;
        self
    }
}
impl<'a> Deref for Win32SurfaceCreateInfoKHRBuilder<'a> {
    type Target = vk::Win32SurfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::XlibSurfaceCreateInfoKHR {
    type Type = XlibSurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct XlibSurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::XlibSurfaceCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> XlibSurfaceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::XcbSurfaceCreateInfoKHR {
    type Type = XcbSurfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct XcbSurfaceCreateInfoKHRBuilder<'a> {
    inner: vk::XcbSurfaceCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> XcbSurfaceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ImagePipeSurfaceCreateInfoFUCHSIA {
    type Type = ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    inner: vk::ImagePipeSurfaceCreateInfoFUCHSIA,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::ImagePipeSurfaceCreateFlagsFUCHSIA) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn image_pipe_handle(mut self, image_pipe_handle: vk::zx_handle_t) -> Self {
        self.inner.image_pipe_handle = image_pipe_handle;
        self
    }
}
impl<'a> Deref for ImagePipeSurfaceCreateInfoFUCHSIABuilder<'a> {
    type Target = vk::ImagePipeSurfaceCreateInfoFUCHSIA;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SwapchainCreateInfoKHR {
    type Type = SwapchainCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainCreateInfoKHRBuilder<'a> {
    inner: vk::SwapchainCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SwapchainCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
        self.inner.surface = Some(surface);
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
    pub fn old_swapchain(mut self, old_swapchain: Option<vk::SwapchainKHR>) -> Self {
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
impl<'a> Builder<'a> for vk::PresentInfoKHR {
    type Type = PresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentInfoKHRBuilder<'a> {
    inner: vk::PresentInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PresentInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
    pub fn p_swapchains(mut self, p_swapchains: &'a [vk::SwapchainKHR], p_image_indices: &'a [u32]) -> Self {
        self.inner.swapchain_count = p_swapchains.len() as u32;
        assert_eq!(self.inner.swapchain_count, p_image_indices.len() as u32);
        self.inner.p_swapchains = p_swapchains.as_ptr();
        self.inner.p_image_indices = p_image_indices.as_ptr();
        self
    }
    pub fn p_results(mut self, p_results: *mut vk::Result) -> Self {
        self.inner.p_results = p_results;
        self
    }
}
impl<'a> Deref for PresentInfoKHRBuilder<'a> {
    type Target = vk::PresentInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugReportCallbackCreateInfoEXT {
    type Type = DebugReportCallbackCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugReportCallbackCreateInfoEXTBuilder<'a> {
    inner: vk::DebugReportCallbackCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugReportCallbackCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ValidationFlagsEXT {
    type Type = ValidationFlagsEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationFlagsEXTBuilder<'a> {
    inner: vk::ValidationFlagsEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ValidationFlagsEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ValidationFeaturesEXT {
    type Type = ValidationFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationFeaturesEXTBuilder<'a> {
    inner: vk::ValidationFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ValidationFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PipelineRasterizationStateRasterizationOrderAMD {
    type Type = PipelineRasterizationStateRasterizationOrderAMDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    inner: vk::PipelineRasterizationStateRasterizationOrderAMD,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn rasterization_order(mut self, rasterization_order: vk::RasterizationOrderAMD) -> Self {
        self.inner.rasterization_order = rasterization_order;
        self
    }
}
impl<'a> Deref for PipelineRasterizationStateRasterizationOrderAMDBuilder<'a> {
    type Target = vk::PipelineRasterizationStateRasterizationOrderAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugMarkerObjectNameInfoEXT {
    type Type = DebugMarkerObjectNameInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerObjectNameInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerObjectNameInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugMarkerObjectNameInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DebugMarkerObjectTagInfoEXT {
    type Type = DebugMarkerObjectTagInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerObjectTagInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerObjectTagInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugMarkerObjectTagInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DebugMarkerMarkerInfoEXT {
    type Type = DebugMarkerMarkerInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugMarkerMarkerInfoEXTBuilder<'a> {
    inner: vk::DebugMarkerMarkerInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugMarkerMarkerInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_marker_name(mut self, p_marker_name: &'a CStr) -> Self {
        self.inner.p_marker_name = p_marker_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugMarkerMarkerInfoEXTBuilder<'a> {
    type Target = vk::DebugMarkerMarkerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DedicatedAllocationImageCreateInfoNV {
    type Type = DedicatedAllocationImageCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    inner: vk::DedicatedAllocationImageCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.inner.dedicated_allocation = if dedicated_allocation { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for DedicatedAllocationImageCreateInfoNVBuilder<'a> {
    type Target = vk::DedicatedAllocationImageCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DedicatedAllocationBufferCreateInfoNV {
    type Type = DedicatedAllocationBufferCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    inner: vk::DedicatedAllocationBufferCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation(mut self, dedicated_allocation: bool) -> Self {
        self.inner.dedicated_allocation = if dedicated_allocation { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for DedicatedAllocationBufferCreateInfoNVBuilder<'a> {
    type Target = vk::DedicatedAllocationBufferCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DedicatedAllocationMemoryAllocateInfoNV {
    type Type = DedicatedAllocationMemoryAllocateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    inner: vk::DedicatedAllocationMemoryAllocateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: Option<vk::Image>) -> Self {
        self.inner.image = image;
        self
    }
    pub fn buffer(mut self, buffer: Option<vk::Buffer>) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl<'a> Deref for DedicatedAllocationMemoryAllocateInfoNVBuilder<'a> {
    type Target = vk::DedicatedAllocationMemoryAllocateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExternalMemoryImageCreateInfoNV {
    type Type = ExternalMemoryImageCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryImageCreateInfoNVBuilder<'a> {
    inner: vk::ExternalMemoryImageCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExternalMemoryImageCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl<'a> Deref for ExternalMemoryImageCreateInfoNVBuilder<'a> {
    type Target = vk::ExternalMemoryImageCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportMemoryAllocateInfoNV {
    type Type = ExportMemoryAllocateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryAllocateInfoNVBuilder<'a> {
    inner: vk::ExportMemoryAllocateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExportMemoryAllocateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl<'a> Deref for ExportMemoryAllocateInfoNVBuilder<'a> {
    type Target = vk::ExportMemoryAllocateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImportMemoryWin32HandleInfoNV {
    type Type = ImportMemoryWin32HandleInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryWin32HandleInfoNVBuilder<'a> {
    inner: vk::ImportMemoryWin32HandleInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportMemoryWin32HandleInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlagsNV) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
}
impl<'a> Deref for ImportMemoryWin32HandleInfoNVBuilder<'a> {
    type Target = vk::ImportMemoryWin32HandleInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportMemoryWin32HandleInfoNV {
    type Type = ExportMemoryWin32HandleInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryWin32HandleInfoNVBuilder<'a> {
    inner: vk::ExportMemoryWin32HandleInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExportMemoryWin32HandleInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
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
impl<'a> Builder<'a> for vk::Win32KeyedMutexAcquireReleaseInfoNV {
    type Type = Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    inner: vk::Win32KeyedMutexAcquireReleaseInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> Win32KeyedMutexAcquireReleaseInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    type Type = PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_generated_commands(mut self, device_generated_commands: bool) -> Self {
        self.inner.device_generated_commands = if device_generated_commands { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceDeviceGeneratedCommandsFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceDeviceGeneratedCommandsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GraphicsShaderGroupCreateInfoNV {
    type Type = GraphicsShaderGroupCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    inner: vk::GraphicsShaderGroupCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
        self.inner.p_vertex_input_state = p_vertex_input_state.map_or(ptr::null(), |p| p);
        self
    }
    pub fn p_tessellation_state(
        mut self,
        p_tessellation_state: Option<&'a vk::PipelineTessellationStateCreateInfo>,
    ) -> Self {
        self.inner.p_tessellation_state = p_tessellation_state.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for GraphicsShaderGroupCreateInfoNVBuilder<'a> {
    type Target = vk::GraphicsShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GraphicsPipelineShaderGroupsCreateInfoNV {
    type Type = GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    inner: vk::GraphicsPipelineShaderGroupsCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> GraphicsPipelineShaderGroupsCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutTokenNV {
    type Type = IndirectCommandsLayoutTokenNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IndirectCommandsLayoutTokenNVBuilder<'a> {
    inner: vk::IndirectCommandsLayoutTokenNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> IndirectCommandsLayoutTokenNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
    pub fn pushconstant_pipeline_layout(mut self, pushconstant_pipeline_layout: Option<vk::PipelineLayout>) -> Self {
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
impl<'a> Builder<'a> for vk::IndirectCommandsLayoutCreateInfoNV {
    type Type = IndirectCommandsLayoutCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    inner: vk::IndirectCommandsLayoutCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> IndirectCommandsLayoutCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::GeneratedCommandsInfoNV {
    type Type = GeneratedCommandsInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeneratedCommandsInfoNVBuilder<'a> {
    inner: vk::GeneratedCommandsInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> GeneratedCommandsInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn indirect_commands_layout(mut self, indirect_commands_layout: vk::IndirectCommandsLayoutNV) -> Self {
        self.inner.indirect_commands_layout = Some(indirect_commands_layout);
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
        self.inner.preprocess_buffer = Some(preprocess_buffer);
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
    pub fn sequences_count_buffer(mut self, sequences_count_buffer: Option<vk::Buffer>) -> Self {
        self.inner.sequences_count_buffer = sequences_count_buffer;
        self
    }
    pub fn sequences_count_offset(mut self, sequences_count_offset: vk::DeviceSize) -> Self {
        self.inner.sequences_count_offset = sequences_count_offset;
        self
    }
    pub fn sequences_index_buffer(mut self, sequences_index_buffer: Option<vk::Buffer>) -> Self {
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
impl<'a> Builder<'a> for vk::GeneratedCommandsMemoryRequirementsInfoNV {
    type Type = GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
    inner: vk::GeneratedCommandsMemoryRequirementsInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn indirect_commands_layout(mut self, indirect_commands_layout: vk::IndirectCommandsLayoutNV) -> Self {
        self.inner.indirect_commands_layout = Some(indirect_commands_layout);
        self
    }
    pub fn max_sequences_count(mut self, max_sequences_count: u32) -> Self {
        self.inner.max_sequences_count = max_sequences_count;
        self
    }
}
impl<'a> Deref for GeneratedCommandsMemoryRequirementsInfoNVBuilder<'a> {
    type Target = vk::GeneratedCommandsMemoryRequirementsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceFeatures2 {
    type Type = PhysicalDeviceFeatures2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFeatures2Builder<'a> {
    inner: vk::PhysicalDeviceFeatures2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceFeatures2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn features(mut self, features: vk::PhysicalDeviceFeatures) -> Self {
        self.inner.features = features;
        self
    }
}
impl<'a> Deref for PhysicalDeviceFeatures2Builder<'a> {
    type Target = vk::PhysicalDeviceFeatures2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceImageFormatInfo2 {
    type Type = PhysicalDeviceImageFormatInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageFormatInfo2Builder<'a> {
    inner: vk::PhysicalDeviceImageFormatInfo2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceImageFormatInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ty(mut self, ty: vk::ImageType) -> Self {
        self.inner.ty = ty;
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
    pub fn flags(mut self, flags: vk::ImageCreateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for PhysicalDeviceImageFormatInfo2Builder<'a> {
    type Target = vk::PhysicalDeviceImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceSparseImageFormatInfo2 {
    type Type = PhysicalDeviceSparseImageFormatInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    inner: vk::PhysicalDeviceSparseImageFormatInfo2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ty(mut self, ty: vk::ImageType) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn tiling(mut self, tiling: vk::ImageTiling) -> Self {
        self.inner.tiling = tiling;
        self
    }
}
impl<'a> Deref for PhysicalDeviceSparseImageFormatInfo2Builder<'a> {
    type Target = vk::PhysicalDeviceSparseImageFormatInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PresentRegionsKHR {
    type Type = PresentRegionsKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentRegionsKHRBuilder<'a> {
    inner: vk::PresentRegionsKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PresentRegionsKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PresentRegionKHR {
    type Type = PresentRegionKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentRegionKHRBuilder<'a> {
    inner: vk::PresentRegionKHR,
    phantom: PhantomData<&'a vk::RectLayerKHR>,
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
impl<'a> Builder<'a> for vk::PhysicalDeviceVariablePointersFeatures {
    type Type = PhysicalDeviceVariablePointersFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceVariablePointersFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.inner.variable_pointers_storage_buffer = if variable_pointers_storage_buffer {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.inner.variable_pointers = if variable_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceVariablePointersFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceVariablePointersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceExternalImageFormatInfo {
    type Type = PhysicalDeviceExternalImageFormatInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    inner: vk::PhysicalDeviceExternalImageFormatInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for PhysicalDeviceExternalImageFormatInfoBuilder<'a> {
    type Target = vk::PhysicalDeviceExternalImageFormatInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceExternalBufferInfo {
    type Type = PhysicalDeviceExternalBufferInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalBufferInfoBuilder<'a> {
    inner: vk::PhysicalDeviceExternalBufferInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceExternalBufferInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
    pub fn usage(mut self, usage: vk::BufferUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for PhysicalDeviceExternalBufferInfoBuilder<'a> {
    type Target = vk::PhysicalDeviceExternalBufferInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExternalMemoryImageCreateInfo {
    type Type = ExternalMemoryImageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryImageCreateInfoBuilder<'a> {
    inner: vk::ExternalMemoryImageCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExternalMemoryImageCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl<'a> Deref for ExternalMemoryImageCreateInfoBuilder<'a> {
    type Target = vk::ExternalMemoryImageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExternalMemoryBufferCreateInfo {
    type Type = ExternalMemoryBufferCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalMemoryBufferCreateInfoBuilder<'a> {
    inner: vk::ExternalMemoryBufferCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExternalMemoryBufferCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl<'a> Deref for ExternalMemoryBufferCreateInfoBuilder<'a> {
    type Target = vk::ExternalMemoryBufferCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportMemoryAllocateInfo {
    type Type = ExportMemoryAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryAllocateInfoBuilder<'a> {
    inner: vk::ExportMemoryAllocateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExportMemoryAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl<'a> Deref for ExportMemoryAllocateInfoBuilder<'a> {
    type Target = vk::ExportMemoryAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImportMemoryWin32HandleInfoKHR {
    type Type = ImportMemoryWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ImportMemoryWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ImportMemoryWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ImportMemoryWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportMemoryWin32HandleInfoKHR {
    type Type = ExportMemoryWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportMemoryWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExportMemoryWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
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
impl<'a> Builder<'a> for vk::MemoryGetWin32HandleInfoKHR {
    type Type = MemoryGetWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetWin32HandleInfoKHRBuilder<'a> {
    inner: vk::MemoryGetWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryGetWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for MemoryGetWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::MemoryGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImportMemoryFdInfoKHR {
    type Type = ImportMemoryFdInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryFdInfoKHRBuilder<'a> {
    inner: vk::ImportMemoryFdInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportMemoryFdInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl<'a> Deref for ImportMemoryFdInfoKHRBuilder<'a> {
    type Target = vk::ImportMemoryFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MemoryGetFdInfoKHR {
    type Type = MemoryGetFdInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetFdInfoKHRBuilder<'a> {
    inner: vk::MemoryGetFdInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryGetFdInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalMemoryHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for MemoryGetFdInfoKHRBuilder<'a> {
    type Target = vk::MemoryGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::Win32KeyedMutexAcquireReleaseInfoKHR {
    type Type = Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    inner: vk::Win32KeyedMutexAcquireReleaseInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> Win32KeyedMutexAcquireReleaseInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PhysicalDeviceExternalSemaphoreInfo {
    type Type = PhysicalDeviceExternalSemaphoreInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    inner: vk::PhysicalDeviceExternalSemaphoreInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for PhysicalDeviceExternalSemaphoreInfoBuilder<'a> {
    type Target = vk::PhysicalDeviceExternalSemaphoreInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportSemaphoreCreateInfo {
    type Type = ExportSemaphoreCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportSemaphoreCreateInfoBuilder<'a> {
    inner: vk::ExportSemaphoreCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExportSemaphoreCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl<'a> Deref for ExportSemaphoreCreateInfoBuilder<'a> {
    type Target = vk::ExportSemaphoreCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImportSemaphoreWin32HandleInfoKHR {
    type Type = ImportSemaphoreWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ImportSemaphoreWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ImportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ImportSemaphoreWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportSemaphoreWin32HandleInfoKHR {
    type Type = ExportSemaphoreWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportSemaphoreWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExportSemaphoreWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
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
impl<'a> Builder<'a> for vk::D3D12FenceSubmitInfoKHR {
    type Type = D3D12FenceSubmitInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct D3D12FenceSubmitInfoKHRBuilder<'a> {
    inner: vk::D3D12FenceSubmitInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> D3D12FenceSubmitInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::SemaphoreGetWin32HandleInfoKHR {
    type Type = SemaphoreGetWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    inner: vk::SemaphoreGetWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for SemaphoreGetWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::SemaphoreGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImportSemaphoreFdInfoKHR {
    type Type = ImportSemaphoreFdInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportSemaphoreFdInfoKHRBuilder<'a> {
    inner: vk::ImportSemaphoreFdInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportSemaphoreFdInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn flags(mut self, flags: vk::SemaphoreImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl<'a> Deref for ImportSemaphoreFdInfoKHRBuilder<'a> {
    type Target = vk::ImportSemaphoreFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SemaphoreGetFdInfoKHR {
    type Type = SemaphoreGetFdInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreGetFdInfoKHRBuilder<'a> {
    inner: vk::SemaphoreGetFdInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SemaphoreGetFdInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalSemaphoreHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for SemaphoreGetFdInfoKHRBuilder<'a> {
    type Target = vk::SemaphoreGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceExternalFenceInfo {
    type Type = PhysicalDeviceExternalFenceInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExternalFenceInfoBuilder<'a> {
    inner: vk::PhysicalDeviceExternalFenceInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceExternalFenceInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for PhysicalDeviceExternalFenceInfoBuilder<'a> {
    type Target = vk::PhysicalDeviceExternalFenceInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportFenceCreateInfo {
    type Type = ExportFenceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportFenceCreateInfoBuilder<'a> {
    inner: vk::ExportFenceCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExportFenceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle_types(mut self, handle_types: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_types = handle_types;
        self
    }
}
impl<'a> Deref for ExportFenceCreateInfoBuilder<'a> {
    type Target = vk::ExportFenceCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImportFenceWin32HandleInfoKHR {
    type Type = ImportFenceWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportFenceWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ImportFenceWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportFenceWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn flags(mut self, flags: vk::FenceImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn handle(mut self, handle: vk::HANDLE) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn name(mut self, name: vk::LPCWSTR) -> Self {
        self.inner.name = name;
        self
    }
}
impl<'a> Deref for ImportFenceWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::ImportFenceWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExportFenceWin32HandleInfoKHR {
    type Type = ExportFenceWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExportFenceWin32HandleInfoKHRBuilder<'a> {
    inner: vk::ExportFenceWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExportFenceWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_attributes(mut self, p_attributes: Option<&'a vk::SECURITY_ATTRIBUTES>) -> Self {
        self.inner.p_attributes = p_attributes.map_or(ptr::null(), |p| p);
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
impl<'a> Builder<'a> for vk::FenceGetWin32HandleInfoKHR {
    type Type = FenceGetWin32HandleInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FenceGetWin32HandleInfoKHRBuilder<'a> {
    inner: vk::FenceGetWin32HandleInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> FenceGetWin32HandleInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for FenceGetWin32HandleInfoKHRBuilder<'a> {
    type Target = vk::FenceGetWin32HandleInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImportFenceFdInfoKHR {
    type Type = ImportFenceFdInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportFenceFdInfoKHRBuilder<'a> {
    inner: vk::ImportFenceFdInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportFenceFdInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn flags(mut self, flags: vk::FenceImportFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
    pub fn fd(mut self, fd: c_int) -> Self {
        self.inner.fd = fd;
        self
    }
}
impl<'a> Deref for ImportFenceFdInfoKHRBuilder<'a> {
    type Target = vk::ImportFenceFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FenceGetFdInfoKHR {
    type Type = FenceGetFdInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FenceGetFdInfoKHRBuilder<'a> {
    inner: vk::FenceGetFdInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> FenceGetFdInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fence(mut self, fence: vk::Fence) -> Self {
        self.inner.fence = Some(fence);
        self
    }
    pub fn handle_type(mut self, handle_type: vk::ExternalFenceHandleTypeFlags) -> Self {
        self.inner.handle_type = handle_type;
        self
    }
}
impl<'a> Deref for FenceGetFdInfoKHRBuilder<'a> {
    type Target = vk::FenceGetFdInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceMultiviewFeatures {
    type Type = PhysicalDeviceMultiviewFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceMultiviewFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.inner.multiview = if multiview { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.inner.multiview_geometry_shader = if multiview_geometry_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.inner.multiview_tessellation_shader = if multiview_tessellation_shader {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceMultiviewFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceMultiviewFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassMultiviewCreateInfo {
    type Type = RenderPassMultiviewCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassMultiviewCreateInfoBuilder<'a> {
    inner: vk::RenderPassMultiviewCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassMultiviewCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DisplayPowerInfoEXT {
    type Type = DisplayPowerInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPowerInfoEXTBuilder<'a> {
    inner: vk::DisplayPowerInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DisplayPowerInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn power_state(mut self, power_state: vk::DisplayPowerStateEXT) -> Self {
        self.inner.power_state = power_state;
        self
    }
}
impl<'a> Deref for DisplayPowerInfoEXTBuilder<'a> {
    type Target = vk::DisplayPowerInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceEventInfoEXT {
    type Type = DeviceEventInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceEventInfoEXTBuilder<'a> {
    inner: vk::DeviceEventInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceEventInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_event(mut self, device_event: vk::DeviceEventTypeEXT) -> Self {
        self.inner.device_event = device_event;
        self
    }
}
impl<'a> Deref for DeviceEventInfoEXTBuilder<'a> {
    type Target = vk::DeviceEventInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DisplayEventInfoEXT {
    type Type = DisplayEventInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayEventInfoEXTBuilder<'a> {
    inner: vk::DisplayEventInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DisplayEventInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn display_event(mut self, display_event: vk::DisplayEventTypeEXT) -> Self {
        self.inner.display_event = display_event;
        self
    }
}
impl<'a> Deref for DisplayEventInfoEXTBuilder<'a> {
    type Target = vk::DisplayEventInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SwapchainCounterCreateInfoEXT {
    type Type = SwapchainCounterCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainCounterCreateInfoEXTBuilder<'a> {
    inner: vk::SwapchainCounterCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SwapchainCounterCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn surface_counters(mut self, surface_counters: vk::SurfaceCounterFlagsEXT) -> Self {
        self.inner.surface_counters = surface_counters;
        self
    }
}
impl<'a> Deref for SwapchainCounterCreateInfoEXTBuilder<'a> {
    type Target = vk::SwapchainCounterCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MemoryAllocateFlagsInfo {
    type Type = MemoryAllocateFlagsInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryAllocateFlagsInfoBuilder<'a> {
    inner: vk::MemoryAllocateFlagsInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryAllocateFlagsInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::MemoryAllocateFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl<'a> Deref for MemoryAllocateFlagsInfoBuilder<'a> {
    type Target = vk::MemoryAllocateFlagsInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindBufferMemoryInfo {
    type Type = BindBufferMemoryInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindBufferMemoryInfoBuilder<'a> {
    inner: vk::BindBufferMemoryInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BindBufferMemoryInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
}
impl<'a> Deref for BindBufferMemoryInfoBuilder<'a> {
    type Target = vk::BindBufferMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindBufferMemoryDeviceGroupInfo {
    type Type = BindBufferMemoryDeviceGroupInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    inner: vk::BindBufferMemoryDeviceGroupInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BindBufferMemoryDeviceGroupInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::BindImageMemoryInfo {
    type Type = BindImageMemoryInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImageMemoryInfoBuilder<'a> {
    inner: vk::BindImageMemoryInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BindImageMemoryInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
    pub fn memory_offset(mut self, memory_offset: vk::DeviceSize) -> Self {
        self.inner.memory_offset = memory_offset;
        self
    }
}
impl<'a> Deref for BindImageMemoryInfoBuilder<'a> {
    type Target = vk::BindImageMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindImageMemoryDeviceGroupInfo {
    type Type = BindImageMemoryDeviceGroupInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImageMemoryDeviceGroupInfoBuilder<'a> {
    inner: vk::BindImageMemoryDeviceGroupInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BindImageMemoryDeviceGroupInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DeviceGroupRenderPassBeginInfo {
    type Type = DeviceGroupRenderPassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupRenderPassBeginInfoBuilder<'a> {
    inner: vk::DeviceGroupRenderPassBeginInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceGroupRenderPassBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DeviceGroupCommandBufferBeginInfo {
    type Type = DeviceGroupCommandBufferBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    inner: vk::DeviceGroupCommandBufferBeginInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl<'a> Deref for DeviceGroupCommandBufferBeginInfoBuilder<'a> {
    type Target = vk::DeviceGroupCommandBufferBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceGroupSubmitInfo {
    type Type = DeviceGroupSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupSubmitInfoBuilder<'a> {
    inner: vk::DeviceGroupSubmitInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceGroupSubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DeviceGroupBindSparseInfo {
    type Type = DeviceGroupBindSparseInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupBindSparseInfoBuilder<'a> {
    inner: vk::DeviceGroupBindSparseInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceGroupBindSparseInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn resource_device_index(mut self, resource_device_index: u32) -> Self {
        self.inner.resource_device_index = resource_device_index;
        self
    }
    pub fn memory_device_index(mut self, memory_device_index: u32) -> Self {
        self.inner.memory_device_index = memory_device_index;
        self
    }
}
impl<'a> Deref for DeviceGroupBindSparseInfoBuilder<'a> {
    type Target = vk::DeviceGroupBindSparseInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageSwapchainCreateInfoKHR {
    type Type = ImageSwapchainCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageSwapchainCreateInfoKHRBuilder<'a> {
    inner: vk::ImageSwapchainCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageSwapchainCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: Option<vk::SwapchainKHR>) -> Self {
        self.inner.swapchain = swapchain;
        self
    }
}
impl<'a> Deref for ImageSwapchainCreateInfoKHRBuilder<'a> {
    type Target = vk::ImageSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindImageMemorySwapchainInfoKHR {
    type Type = BindImageMemorySwapchainInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImageMemorySwapchainInfoKHRBuilder<'a> {
    inner: vk::BindImageMemorySwapchainInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BindImageMemorySwapchainInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: vk::SwapchainKHR) -> Self {
        self.inner.swapchain = Some(swapchain);
        self
    }
    pub fn image_index(mut self, image_index: u32) -> Self {
        self.inner.image_index = image_index;
        self
    }
}
impl<'a> Deref for BindImageMemorySwapchainInfoKHRBuilder<'a> {
    type Target = vk::BindImageMemorySwapchainInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AcquireNextImageInfoKHR {
    type Type = AcquireNextImageInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AcquireNextImageInfoKHRBuilder<'a> {
    inner: vk::AcquireNextImageInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AcquireNextImageInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn swapchain(mut self, swapchain: vk::SwapchainKHR) -> Self {
        self.inner.swapchain = Some(swapchain);
        self
    }
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.inner.timeout = timeout;
        self
    }
    pub fn semaphore(mut self, semaphore: Option<vk::Semaphore>) -> Self {
        self.inner.semaphore = semaphore;
        self
    }
    pub fn fence(mut self, fence: Option<vk::Fence>) -> Self {
        self.inner.fence = fence;
        self
    }
    pub fn device_mask(mut self, device_mask: u32) -> Self {
        self.inner.device_mask = device_mask;
        self
    }
}
impl<'a> Deref for AcquireNextImageInfoKHRBuilder<'a> {
    type Target = vk::AcquireNextImageInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceGroupPresentInfoKHR {
    type Type = DeviceGroupPresentInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupPresentInfoKHRBuilder<'a> {
    inner: vk::DeviceGroupPresentInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceGroupPresentInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DeviceGroupDeviceCreateInfo {
    type Type = DeviceGroupDeviceCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupDeviceCreateInfoBuilder<'a> {
    inner: vk::DeviceGroupDeviceCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceGroupDeviceCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DeviceGroupSwapchainCreateInfoKHR {
    type Type = DeviceGroupSwapchainCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    inner: vk::DeviceGroupSwapchainCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn modes(mut self, modes: vk::DeviceGroupPresentModeFlagsKHR) -> Self {
        self.inner.modes = modes;
        self
    }
}
impl<'a> Deref for DeviceGroupSwapchainCreateInfoKHRBuilder<'a> {
    type Target = vk::DeviceGroupSwapchainCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorUpdateTemplateCreateInfo {
    type Type = DescriptorUpdateTemplateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    inner: vk::DescriptorUpdateTemplateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DescriptorUpdateTemplateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
        self.inner.descriptor_set_layout = Some(descriptor_set_layout);
        self
    }
    pub fn pipeline_bind_point(mut self, pipeline_bind_point: vk::PipelineBindPoint) -> Self {
        self.inner.pipeline_bind_point = pipeline_bind_point;
        self
    }
    pub fn pipeline_layout(mut self, pipeline_layout: vk::PipelineLayout) -> Self {
        self.inner.pipeline_layout = Some(pipeline_layout);
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
impl<'a> Builder<'a> for vk::HdrMetadataEXT {
    type Type = HdrMetadataEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct HdrMetadataEXTBuilder<'a> {
    inner: vk::HdrMetadataEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> HdrMetadataEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn display_primary_red(mut self, display_primary_red: vk::XYColorEXT) -> Self {
        self.inner.display_primary_red = display_primary_red;
        self
    }
    pub fn display_primary_green(mut self, display_primary_green: vk::XYColorEXT) -> Self {
        self.inner.display_primary_green = display_primary_green;
        self
    }
    pub fn display_primary_blue(mut self, display_primary_blue: vk::XYColorEXT) -> Self {
        self.inner.display_primary_blue = display_primary_blue;
        self
    }
    pub fn white_point(mut self, white_point: vk::XYColorEXT) -> Self {
        self.inner.white_point = white_point;
        self
    }
    pub fn max_luminance(mut self, max_luminance: f32) -> Self {
        self.inner.max_luminance = max_luminance;
        self
    }
    pub fn min_luminance(mut self, min_luminance: f32) -> Self {
        self.inner.min_luminance = min_luminance;
        self
    }
    pub fn max_content_light_level(mut self, max_content_light_level: f32) -> Self {
        self.inner.max_content_light_level = max_content_light_level;
        self
    }
    pub fn max_frame_average_light_level(mut self, max_frame_average_light_level: f32) -> Self {
        self.inner.max_frame_average_light_level = max_frame_average_light_level;
        self
    }
}
impl<'a> Deref for HdrMetadataEXTBuilder<'a> {
    type Target = vk::HdrMetadataEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SwapchainDisplayNativeHdrCreateInfoAMD {
    type Type = SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    inner: vk::SwapchainDisplayNativeHdrCreateInfoAMD,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn local_dimming_enable(mut self, local_dimming_enable: bool) -> Self {
        self.inner.local_dimming_enable = if local_dimming_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for SwapchainDisplayNativeHdrCreateInfoAMDBuilder<'a> {
    type Target = vk::SwapchainDisplayNativeHdrCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PresentTimesInfoGOOGLE {
    type Type = PresentTimesInfoGOOGLEBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PresentTimesInfoGOOGLEBuilder<'a> {
    inner: vk::PresentTimesInfoGOOGLE,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PresentTimesInfoGOOGLEBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::IOSSurfaceCreateInfoMVK {
    type Type = IOSSurfaceCreateInfoMVKBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct IOSSurfaceCreateInfoMVKBuilder<'a> {
    inner: vk::IOSSurfaceCreateInfoMVK,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> IOSSurfaceCreateInfoMVKBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::MacOSSurfaceCreateInfoMVK {
    type Type = MacOSSurfaceCreateInfoMVKBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MacOSSurfaceCreateInfoMVKBuilder<'a> {
    inner: vk::MacOSSurfaceCreateInfoMVK,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MacOSSurfaceCreateInfoMVKBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::MetalSurfaceCreateInfoEXT {
    type Type = MetalSurfaceCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MetalSurfaceCreateInfoEXTBuilder<'a> {
    inner: vk::MetalSurfaceCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MetalSurfaceCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PipelineViewportWScalingStateCreateInfoNV {
    type Type = PipelineViewportWScalingStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportWScalingStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineViewportWScalingStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PipelineViewportSwizzleStateCreateInfoNV {
    type Type = PipelineViewportSwizzleStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportSwizzleStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineViewportSwizzleStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PipelineDiscardRectangleStateCreateInfoEXT {
    type Type = PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineDiscardRectangleStateCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineDiscardRectangleStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::RenderPassInputAttachmentAspectCreateInfo {
    type Type = RenderPassInputAttachmentAspectCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    inner: vk::RenderPassInputAttachmentAspectCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassInputAttachmentAspectCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PhysicalDeviceSurfaceInfo2KHR {
    type Type = PhysicalDeviceSurfaceInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    inner: vk::PhysicalDeviceSurfaceInfo2KHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn surface(mut self, surface: vk::SurfaceKHR) -> Self {
        self.inner.surface = Some(surface);
        self
    }
}
impl<'a> Deref for PhysicalDeviceSurfaceInfo2KHRBuilder<'a> {
    type Target = vk::PhysicalDeviceSurfaceInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DisplayPlaneInfo2KHR {
    type Type = DisplayPlaneInfo2KHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DisplayPlaneInfo2KHRBuilder<'a> {
    inner: vk::DisplayPlaneInfo2KHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DisplayPlaneInfo2KHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn mode(mut self, mode: vk::DisplayModeKHR) -> Self {
        self.inner.mode = Some(mode);
        self
    }
    pub fn plane_index(mut self, plane_index: u32) -> Self {
        self.inner.plane_index = plane_index;
        self
    }
}
impl<'a> Deref for DisplayPlaneInfo2KHRBuilder<'a> {
    type Target = vk::DisplayPlaneInfo2KHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDevice16BitStorageFeatures {
    type Type = PhysicalDevice16BitStorageFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    inner: vk::PhysicalDevice16BitStorageFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.inner.storage_buffer16_bit_access = if storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer16_bit_access(mut self, uniform_and_storage_buffer16_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer16_bit_access = if uniform_and_storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.inner.storage_push_constant16 = if storage_push_constant16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.inner.storage_input_output16 = if storage_input_output16 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDevice16BitStorageFeaturesBuilder<'a> {
    type Target = vk::PhysicalDevice16BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    type Type = PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.inner.shader_subgroup_extended_types = if shader_subgroup_extended_types {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderSubgroupExtendedTypesFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderSubgroupExtendedTypesFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferMemoryRequirementsInfo2 {
    type Type = BufferMemoryRequirementsInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferMemoryRequirementsInfo2Builder<'a> {
    inner: vk::BufferMemoryRequirementsInfo2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BufferMemoryRequirementsInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
}
impl<'a> Deref for BufferMemoryRequirementsInfo2Builder<'a> {
    type Target = vk::BufferMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageMemoryRequirementsInfo2 {
    type Type = ImageMemoryRequirementsInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageMemoryRequirementsInfo2Builder<'a> {
    inner: vk::ImageMemoryRequirementsInfo2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageMemoryRequirementsInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
}
impl<'a> Deref for ImageMemoryRequirementsInfo2Builder<'a> {
    type Target = vk::ImageMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageSparseMemoryRequirementsInfo2 {
    type Type = ImageSparseMemoryRequirementsInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageSparseMemoryRequirementsInfo2Builder<'a> {
    inner: vk::ImageSparseMemoryRequirementsInfo2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageSparseMemoryRequirementsInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: vk::Image) -> Self {
        self.inner.image = Some(image);
        self
    }
}
impl<'a> Deref for ImageSparseMemoryRequirementsInfo2Builder<'a> {
    type Target = vk::ImageSparseMemoryRequirementsInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MemoryDedicatedAllocateInfo {
    type Type = MemoryDedicatedAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryDedicatedAllocateInfoBuilder<'a> {
    inner: vk::MemoryDedicatedAllocateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryDedicatedAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image(mut self, image: Option<vk::Image>) -> Self {
        self.inner.image = image;
        self
    }
    pub fn buffer(mut self, buffer: Option<vk::Buffer>) -> Self {
        self.inner.buffer = buffer;
        self
    }
}
impl<'a> Deref for MemoryDedicatedAllocateInfoBuilder<'a> {
    type Target = vk::MemoryDedicatedAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageViewUsageCreateInfo {
    type Type = ImageViewUsageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewUsageCreateInfoBuilder<'a> {
    inner: vk::ImageViewUsageCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageViewUsageCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn usage(mut self, usage: vk::ImageUsageFlags) -> Self {
        self.inner.usage = usage;
        self
    }
}
impl<'a> Deref for ImageViewUsageCreateInfoBuilder<'a> {
    type Target = vk::ImageViewUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineTessellationDomainOriginStateCreateInfo {
    type Type = PipelineTessellationDomainOriginStateCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    inner: vk::PipelineTessellationDomainOriginStateCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn domain_origin(mut self, domain_origin: vk::TessellationDomainOrigin) -> Self {
        self.inner.domain_origin = domain_origin;
        self
    }
}
impl<'a> Deref for PipelineTessellationDomainOriginStateCreateInfoBuilder<'a> {
    type Target = vk::PipelineTessellationDomainOriginStateCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SamplerYcbcrConversionInfo {
    type Type = SamplerYcbcrConversionInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerYcbcrConversionInfoBuilder<'a> {
    inner: vk::SamplerYcbcrConversionInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SamplerYcbcrConversionInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conversion(mut self, conversion: vk::SamplerYcbcrConversion) -> Self {
        self.inner.conversion = Some(conversion);
        self
    }
}
impl<'a> Deref for SamplerYcbcrConversionInfoBuilder<'a> {
    type Target = vk::SamplerYcbcrConversionInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SamplerYcbcrConversionCreateInfo {
    type Type = SamplerYcbcrConversionCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerYcbcrConversionCreateInfoBuilder<'a> {
    inner: vk::SamplerYcbcrConversionCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SamplerYcbcrConversionCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn ycbcr_model(mut self, ycbcr_model: vk::SamplerYcbcrModelConversion) -> Self {
        self.inner.ycbcr_model = ycbcr_model;
        self
    }
    pub fn ycbcr_range(mut self, ycbcr_range: vk::SamplerYcbcrRange) -> Self {
        self.inner.ycbcr_range = ycbcr_range;
        self
    }
    pub fn components(mut self, components: vk::ComponentMapping) -> Self {
        self.inner.components = components;
        self
    }
    pub fn x_chroma_offset(mut self, x_chroma_offset: vk::ChromaLocation) -> Self {
        self.inner.x_chroma_offset = x_chroma_offset;
        self
    }
    pub fn y_chroma_offset(mut self, y_chroma_offset: vk::ChromaLocation) -> Self {
        self.inner.y_chroma_offset = y_chroma_offset;
        self
    }
    pub fn chroma_filter(mut self, chroma_filter: vk::Filter) -> Self {
        self.inner.chroma_filter = chroma_filter;
        self
    }
    pub fn force_explicit_reconstruction(mut self, force_explicit_reconstruction: bool) -> Self {
        self.inner.force_explicit_reconstruction = if force_explicit_reconstruction {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for SamplerYcbcrConversionCreateInfoBuilder<'a> {
    type Target = vk::SamplerYcbcrConversionCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindImagePlaneMemoryInfo {
    type Type = BindImagePlaneMemoryInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindImagePlaneMemoryInfoBuilder<'a> {
    inner: vk::BindImagePlaneMemoryInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BindImagePlaneMemoryInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn plane_aspect(mut self, plane_aspect: vk::ImageAspectFlags) -> Self {
        self.inner.plane_aspect = plane_aspect;
        self
    }
}
impl<'a> Deref for BindImagePlaneMemoryInfoBuilder<'a> {
    type Target = vk::BindImagePlaneMemoryInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImagePlaneMemoryRequirementsInfo {
    type Type = ImagePlaneMemoryRequirementsInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    inner: vk::ImagePlaneMemoryRequirementsInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn plane_aspect(mut self, plane_aspect: vk::ImageAspectFlags) -> Self {
        self.inner.plane_aspect = plane_aspect;
        self
    }
}
impl<'a> Deref for ImagePlaneMemoryRequirementsInfoBuilder<'a> {
    type Target = vk::ImagePlaneMemoryRequirementsInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceSamplerYcbcrConversionFeatures {
    type Type = PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceSamplerYcbcrConversionFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.inner.sampler_ycbcr_conversion = if sampler_ycbcr_conversion { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceSamplerYcbcrConversionFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceSamplerYcbcrConversionFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ConditionalRenderingBeginInfoEXT {
    type Type = ConditionalRenderingBeginInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ConditionalRenderingBeginInfoEXTBuilder<'a> {
    inner: vk::ConditionalRenderingBeginInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ConditionalRenderingBeginInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
    pub fn flags(mut self, flags: vk::ConditionalRenderingFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for ConditionalRenderingBeginInfoEXTBuilder<'a> {
    type Target = vk::ConditionalRenderingBeginInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ProtectedSubmitInfo {
    type Type = ProtectedSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ProtectedSubmitInfoBuilder<'a> {
    inner: vk::ProtectedSubmitInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ProtectedSubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn protected_submit(mut self, protected_submit: bool) -> Self {
        self.inner.protected_submit = if protected_submit { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for ProtectedSubmitInfoBuilder<'a> {
    type Target = vk::ProtectedSubmitInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceProtectedMemoryFeatures {
    type Type = PhysicalDeviceProtectedMemoryFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceProtectedMemoryFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.inner.protected_memory = if protected_memory { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceProtectedMemoryFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceProtectedMemoryFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceQueueInfo2 {
    type Type = DeviceQueueInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceQueueInfo2Builder<'a> {
    inner: vk::DeviceQueueInfo2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceQueueInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
    pub fn queue_index(mut self, queue_index: u32) -> Self {
        self.inner.queue_index = queue_index;
        self
    }
}
impl<'a> Deref for DeviceQueueInfo2Builder<'a> {
    type Target = vk::DeviceQueueInfo2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineCoverageToColorStateCreateInfoNV {
    type Type = PipelineCoverageToColorStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineCoverageToColorStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageToColorStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_to_color_enable(mut self, coverage_to_color_enable: bool) -> Self {
        self.inner.coverage_to_color_enable = if coverage_to_color_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn coverage_to_color_location(mut self, coverage_to_color_location: u32) -> Self {
        self.inner.coverage_to_color_location = coverage_to_color_location;
        self
    }
}
impl<'a> Deref for PipelineCoverageToColorStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineCoverageToColorStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SampleLocationsInfoEXT {
    type Type = SampleLocationsInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SampleLocationsInfoEXTBuilder<'a> {
    inner: vk::SampleLocationsInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SampleLocationsInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::RenderPassSampleLocationsBeginInfoEXT {
    type Type = RenderPassSampleLocationsBeginInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    inner: vk::RenderPassSampleLocationsBeginInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassSampleLocationsBeginInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PipelineSampleLocationsStateCreateInfoEXT {
    type Type = PipelineSampleLocationsStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineSampleLocationsStateCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sample_locations_enable(mut self, sample_locations_enable: bool) -> Self {
        self.inner.sample_locations_enable = if sample_locations_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sample_locations_info(mut self, sample_locations_info: vk::SampleLocationsInfoEXT) -> Self {
        self.inner.sample_locations_info = sample_locations_info;
        self
    }
}
impl<'a> Deref for PipelineSampleLocationsStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineSampleLocationsStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SamplerReductionModeCreateInfo {
    type Type = SamplerReductionModeCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SamplerReductionModeCreateInfoBuilder<'a> {
    inner: vk::SamplerReductionModeCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SamplerReductionModeCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn reduction_mode(mut self, reduction_mode: vk::SamplerReductionMode) -> Self {
        self.inner.reduction_mode = reduction_mode;
        self
    }
}
impl<'a> Deref for SamplerReductionModeCreateInfoBuilder<'a> {
    type Target = vk::SamplerReductionModeCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    type Type = PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn advanced_blend_coherent_operations(mut self, advanced_blend_coherent_operations: bool) -> Self {
        self.inner.advanced_blend_coherent_operations = if advanced_blend_coherent_operations {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceBlendOperationAdvancedFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceBlendOperationAdvancedFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineColorBlendAdvancedStateCreateInfoEXT {
    type Type = PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineColorBlendAdvancedStateCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_premultiplied(mut self, src_premultiplied: bool) -> Self {
        self.inner.src_premultiplied = if src_premultiplied { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn dst_premultiplied(mut self, dst_premultiplied: bool) -> Self {
        self.inner.dst_premultiplied = if dst_premultiplied { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn blend_overlap(mut self, blend_overlap: vk::BlendOverlapEXT) -> Self {
        self.inner.blend_overlap = blend_overlap;
        self
    }
}
impl<'a> Deref for PipelineColorBlendAdvancedStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineColorBlendAdvancedStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceInlineUniformBlockFeaturesEXT {
    type Type = PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceInlineUniformBlockFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn inline_uniform_block(mut self, inline_uniform_block: bool) -> Self {
        self.inner.inline_uniform_block = if inline_uniform_block { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn descriptor_binding_inline_uniform_block_update_after_bind(
        mut self,
        descriptor_binding_inline_uniform_block_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_inline_uniform_block_update_after_bind =
            if descriptor_binding_inline_uniform_block_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl<'a> Deref for PhysicalDeviceInlineUniformBlockFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceInlineUniformBlockFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetInlineUniformBlockEXT {
    type Type = WriteDescriptorSetInlineUniformBlockEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    inner: vk::WriteDescriptorSetInlineUniformBlockEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Deref for WriteDescriptorSetInlineUniformBlockEXTBuilder<'a> {
    type Target = vk::WriteDescriptorSetInlineUniformBlockEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorPoolInlineUniformBlockCreateInfoEXT {
    type Type = DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    inner: vk::DescriptorPoolInlineUniformBlockCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_inline_uniform_block_bindings(mut self, max_inline_uniform_block_bindings: u32) -> Self {
        self.inner.max_inline_uniform_block_bindings = max_inline_uniform_block_bindings;
        self
    }
}
impl<'a> Deref for DescriptorPoolInlineUniformBlockCreateInfoEXTBuilder<'a> {
    type Target = vk::DescriptorPoolInlineUniformBlockCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineCoverageModulationStateCreateInfoNV {
    type Type = PipelineCoverageModulationStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineCoverageModulationStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineCoverageModulationStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ImageFormatListCreateInfo {
    type Type = ImageFormatListCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageFormatListCreateInfoBuilder<'a> {
    inner: vk::ImageFormatListCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageFormatListCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ValidationCacheCreateInfoEXT {
    type Type = ValidationCacheCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ValidationCacheCreateInfoEXTBuilder<'a> {
    inner: vk::ValidationCacheCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ValidationCacheCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ShaderModuleValidationCacheCreateInfoEXT {
    type Type = ShaderModuleValidationCacheCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    inner: vk::ShaderModuleValidationCacheCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn validation_cache(mut self, validation_cache: vk::ValidationCacheEXT) -> Self {
        self.inner.validation_cache = Some(validation_cache);
        self
    }
}
impl<'a> Deref for ShaderModuleValidationCacheCreateInfoEXTBuilder<'a> {
    type Target = vk::ShaderModuleValidationCacheCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderDrawParametersFeatures {
    type Type = PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceShaderDrawParametersFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.inner.shader_draw_parameters = if shader_draw_parameters { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderDrawParametersFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderDrawParametersFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderFloat16Int8Features {
    type Type = PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceShaderFloat16Int8Features,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_float16(mut self, shader_float16: bool) -> Self {
        self.inner.shader_float16 = if shader_float16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_int8(mut self, shader_int8: bool) -> Self {
        self.inner.shader_int8 = if shader_int8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderFloat16Int8FeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderFloat16Int8Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceHostQueryResetFeatures {
    type Type = PhysicalDeviceHostQueryResetFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceHostQueryResetFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.inner.host_query_reset = if host_query_reset { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceHostQueryResetFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceHostQueryResetFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::NativeBufferANDROID {
    type Type = NativeBufferANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct NativeBufferANDROIDBuilder<'a> {
    inner: vk::NativeBufferANDROID,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> NativeBufferANDROIDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn handle(mut self, handle: *const c_void) -> Self {
        self.inner.handle = handle;
        self
    }
    pub fn stride(mut self, stride: c_int) -> Self {
        self.inner.stride = stride;
        self
    }
    pub fn format(mut self, format: c_int) -> Self {
        self.inner.format = format;
        self
    }
    pub fn usage(mut self, usage: c_int) -> Self {
        self.inner.usage = usage;
        self
    }
    pub fn usage2(mut self, usage2: vk::NativeBufferUsage2ANDROID) -> Self {
        self.inner.usage2 = usage2;
        self
    }
}
impl<'a> Deref for NativeBufferANDROIDBuilder<'a> {
    type Target = vk::NativeBufferANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SwapchainImageCreateInfoANDROID {
    type Type = SwapchainImageCreateInfoANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SwapchainImageCreateInfoANDROIDBuilder<'a> {
    inner: vk::SwapchainImageCreateInfoANDROID,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SwapchainImageCreateInfoANDROIDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn usage(mut self, usage: vk::SwapchainImageUsageFlagsANDROID) -> Self {
        self.inner.usage = usage;
        self
    }
}
impl<'a> Deref for SwapchainImageCreateInfoANDROIDBuilder<'a> {
    type Target = vk::SwapchainImageCreateInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDevicePresentationPropertiesANDROID {
    type Type = PhysicalDevicePresentationPropertiesANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePresentationPropertiesANDROIDBuilder<'a> {
    inner: vk::PhysicalDevicePresentationPropertiesANDROID,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDevicePresentationPropertiesANDROIDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shared_image(mut self, shared_image: bool) -> Self {
        self.inner.shared_image = if shared_image { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDevicePresentationPropertiesANDROIDBuilder<'a> {
    type Target = vk::PhysicalDevicePresentationPropertiesANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceQueueGlobalPriorityCreateInfoEXT {
    type Type = DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    inner: vk::DeviceQueueGlobalPriorityCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn global_priority(mut self, global_priority: vk::QueueGlobalPriorityEXT) -> Self {
        self.inner.global_priority = global_priority;
        self
    }
}
impl<'a> Deref for DeviceQueueGlobalPriorityCreateInfoEXTBuilder<'a> {
    type Target = vk::DeviceQueueGlobalPriorityCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugUtilsObjectNameInfoEXT {
    type Type = DebugUtilsObjectNameInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsObjectNameInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsObjectNameInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugUtilsObjectNameInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
    pub fn p_object_name(mut self, p_object_name: &'a CStr) -> Self {
        self.inner.p_object_name = p_object_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugUtilsObjectNameInfoEXTBuilder<'a> {
    type Target = vk::DebugUtilsObjectNameInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugUtilsObjectTagInfoEXT {
    type Type = DebugUtilsObjectTagInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsObjectTagInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsObjectTagInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugUtilsObjectTagInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DebugUtilsLabelEXT {
    type Type = DebugUtilsLabelEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsLabelEXTBuilder<'a> {
    inner: vk::DebugUtilsLabelEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugUtilsLabelEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_label_name(mut self, p_label_name: &'a CStr) -> Self {
        self.inner.p_label_name = p_label_name.as_ptr();
        self
    }
}
impl<'a> Deref for DebugUtilsLabelEXTBuilder<'a> {
    type Target = vk::DebugUtilsLabelEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DebugUtilsMessengerCreateInfoEXT {
    type Type = DebugUtilsMessengerCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    inner: vk::DebugUtilsMessengerCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugUtilsMessengerCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DebugUtilsMessengerCallbackDataEXT {
    type Type = DebugUtilsMessengerCallbackDataEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    inner: vk::DebugUtilsMessengerCallbackDataEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DebugUtilsMessengerCallbackDataEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
    pub fn p_message_id_name(mut self, p_message_id_name: &'a CStr) -> Self {
        self.inner.p_message_id_name = p_message_id_name.as_ptr();
        self
    }
    pub fn message_id_number(mut self, message_id_number: i32) -> Self {
        self.inner.message_id_number = message_id_number;
        self
    }
    pub fn p_message(mut self, p_message: &'a CStr) -> Self {
        self.inner.p_message = p_message.as_ptr();
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
impl<'a> Builder<'a> for vk::ImportMemoryHostPointerInfoEXT {
    type Type = ImportMemoryHostPointerInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportMemoryHostPointerInfoEXTBuilder<'a> {
    inner: vk::ImportMemoryHostPointerInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportMemoryHostPointerInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::CalibratedTimestampInfoEXT {
    type Type = CalibratedTimestampInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CalibratedTimestampInfoEXTBuilder<'a> {
    inner: vk::CalibratedTimestampInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CalibratedTimestampInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn time_domain(mut self, time_domain: vk::TimeDomainEXT) -> Self {
        self.inner.time_domain = time_domain;
        self
    }
}
impl<'a> Deref for CalibratedTimestampInfoEXTBuilder<'a> {
    type Target = vk::CalibratedTimestampInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineRasterizationConservativeStateCreateInfoEXT {
    type Type = PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineRasterizationConservativeStateCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationConservativeStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn conservative_rasterization_mode(
        mut self,
        conservative_rasterization_mode: vk::ConservativeRasterizationModeEXT,
    ) -> Self {
        self.inner.conservative_rasterization_mode = conservative_rasterization_mode;
        self
    }
    pub fn extra_primitive_overestimation_size(mut self, extra_primitive_overestimation_size: f32) -> Self {
        self.inner.extra_primitive_overestimation_size = extra_primitive_overestimation_size;
        self
    }
}
impl<'a> Deref for PipelineRasterizationConservativeStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineRasterizationConservativeStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceDescriptorIndexingFeatures {
    type Type = PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceDescriptorIndexingFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_dynamic_indexing = if shader_input_attachment_array_dynamic_indexing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_dynamic_indexing =
            if shader_uniform_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_dynamic_indexing =
            if shader_storage_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing =
            if shader_uniform_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_sampled_image_array_non_uniform_indexing = if shader_sampled_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_buffer_array_non_uniform_indexing =
            if shader_storage_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_image_array_non_uniform_indexing = if shader_storage_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_non_uniform_indexing =
            if shader_input_attachment_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_non_uniform_indexing =
            if shader_uniform_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_non_uniform_indexing =
            if shader_storage_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_buffer_update_after_bind =
            if descriptor_binding_uniform_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_sampled_image_update_after_bind =
            if descriptor_binding_sampled_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_image_update_after_bind =
            if descriptor_binding_storage_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_buffer_update_after_bind =
            if descriptor_binding_storage_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_texel_buffer_update_after_bind =
            if descriptor_binding_uniform_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_texel_buffer_update_after_bind =
            if descriptor_binding_storage_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.inner.descriptor_binding_update_unused_while_pending = if descriptor_binding_update_unused_while_pending {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_partially_bound(mut self, descriptor_binding_partially_bound: bool) -> Self {
        self.inner.descriptor_binding_partially_bound = if descriptor_binding_partially_bound {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.inner.descriptor_binding_variable_descriptor_count = if descriptor_binding_variable_descriptor_count {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.inner.runtime_descriptor_array = if runtime_descriptor_array { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceDescriptorIndexingFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceDescriptorIndexingFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DescriptorSetLayoutBindingFlagsCreateInfo {
    type Type = DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    inner: vk::DescriptorSetLayoutBindingFlagsCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DescriptorSetLayoutBindingFlagsCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::DescriptorSetVariableDescriptorCountAllocateInfo {
    type Type = DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    inner: vk::DescriptorSetVariableDescriptorCountAllocateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DescriptorSetVariableDescriptorCountAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::AttachmentDescription2 {
    type Type = AttachmentDescription2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentDescription2Builder<'a> {
    inner: vk::AttachmentDescription2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AttachmentDescription2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AttachmentDescriptionFlags) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn format(mut self, format: vk::Format) -> Self {
        self.inner.format = format;
        self
    }
    pub fn samples(mut self, samples: vk::SampleCountFlags) -> Self {
        self.inner.samples = samples;
        self
    }
    pub fn load_op(mut self, load_op: vk::AttachmentLoadOp) -> Self {
        self.inner.load_op = load_op;
        self
    }
    pub fn store_op(mut self, store_op: vk::AttachmentStoreOp) -> Self {
        self.inner.store_op = store_op;
        self
    }
    pub fn stencil_load_op(mut self, stencil_load_op: vk::AttachmentLoadOp) -> Self {
        self.inner.stencil_load_op = stencil_load_op;
        self
    }
    pub fn stencil_store_op(mut self, stencil_store_op: vk::AttachmentStoreOp) -> Self {
        self.inner.stencil_store_op = stencil_store_op;
        self
    }
    pub fn initial_layout(mut self, initial_layout: vk::ImageLayout) -> Self {
        self.inner.initial_layout = initial_layout;
        self
    }
    pub fn final_layout(mut self, final_layout: vk::ImageLayout) -> Self {
        self.inner.final_layout = final_layout;
        self
    }
}
impl<'a> Deref for AttachmentDescription2Builder<'a> {
    type Target = vk::AttachmentDescription2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AttachmentReference2 {
    type Type = AttachmentReference2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentReference2Builder<'a> {
    inner: vk::AttachmentReference2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AttachmentReference2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn attachment(mut self, attachment: u32) -> Self {
        self.inner.attachment = attachment;
        self
    }
    pub fn layout(mut self, layout: vk::ImageLayout) -> Self {
        self.inner.layout = layout;
        self
    }
    pub fn aspect_mask(mut self, aspect_mask: vk::ImageAspectFlags) -> Self {
        self.inner.aspect_mask = aspect_mask;
        self
    }
}
impl<'a> Deref for AttachmentReference2Builder<'a> {
    type Target = vk::AttachmentReference2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassDescription2 {
    type Type = SubpassDescription2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDescription2Builder<'a> {
    inner: vk::SubpassDescription2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SubpassDescription2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
        if let Some(s) = p_resolve_attachments {
            assert_eq!(self.inner.color_attachment_count, s.len() as u32);
        }
        self.inner.p_color_attachments = p_color_attachments.as_ptr();
        self.inner.p_resolve_attachments = p_resolve_attachments.map_or(ptr::null(), |s| s.as_ptr());
        self
    }
    pub fn p_depth_stencil_attachment(
        mut self,
        p_depth_stencil_attachment: Option<&'a vk::AttachmentReference2>,
    ) -> Self {
        self.inner.p_depth_stencil_attachment = p_depth_stencil_attachment.map_or(ptr::null(), |p| p);
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
impl<'a> Builder<'a> for vk::SubpassDependency2 {
    type Type = SubpassDependency2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDependency2Builder<'a> {
    inner: vk::SubpassDependency2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SubpassDependency2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src_subpass(mut self, src_subpass: u32) -> Self {
        self.inner.src_subpass = src_subpass;
        self
    }
    pub fn dst_subpass(mut self, dst_subpass: u32) -> Self {
        self.inner.dst_subpass = dst_subpass;
        self
    }
    pub fn src_stage_mask(mut self, src_stage_mask: vk::PipelineStageFlags) -> Self {
        self.inner.src_stage_mask = src_stage_mask;
        self
    }
    pub fn dst_stage_mask(mut self, dst_stage_mask: vk::PipelineStageFlags) -> Self {
        self.inner.dst_stage_mask = dst_stage_mask;
        self
    }
    pub fn src_access_mask(mut self, src_access_mask: vk::AccessFlags) -> Self {
        self.inner.src_access_mask = src_access_mask;
        self
    }
    pub fn dst_access_mask(mut self, dst_access_mask: vk::AccessFlags) -> Self {
        self.inner.dst_access_mask = dst_access_mask;
        self
    }
    pub fn dependency_flags(mut self, dependency_flags: vk::DependencyFlags) -> Self {
        self.inner.dependency_flags = dependency_flags;
        self
    }
    pub fn view_offset(mut self, view_offset: i32) -> Self {
        self.inner.view_offset = view_offset;
        self
    }
}
impl<'a> Deref for SubpassDependency2Builder<'a> {
    type Target = vk::SubpassDependency2;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassCreateInfo2 {
    type Type = RenderPassCreateInfo2Builder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassCreateInfo2Builder<'a> {
    inner: vk::RenderPassCreateInfo2,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassCreateInfo2Builder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::SubpassBeginInfo {
    type Type = SubpassBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassBeginInfoBuilder<'a> {
    inner: vk::SubpassBeginInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SubpassBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn contents(mut self, contents: vk::SubpassContents) -> Self {
        self.inner.contents = contents;
        self
    }
}
impl<'a> Deref for SubpassBeginInfoBuilder<'a> {
    type Target = vk::SubpassBeginInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassEndInfo {
    type Type = SubpassEndInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassEndInfoBuilder<'a> {
    inner: vk::SubpassEndInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SubpassEndInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
}
impl<'a> Deref for SubpassEndInfoBuilder<'a> {
    type Target = vk::SubpassEndInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceTimelineSemaphoreFeatures {
    type Type = PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceTimelineSemaphoreFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.inner.timeline_semaphore = if timeline_semaphore { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceTimelineSemaphoreFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceTimelineSemaphoreFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SemaphoreTypeCreateInfo {
    type Type = SemaphoreTypeCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreTypeCreateInfoBuilder<'a> {
    inner: vk::SemaphoreTypeCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SemaphoreTypeCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore_type(mut self, semaphore_type: vk::SemaphoreType) -> Self {
        self.inner.semaphore_type = semaphore_type;
        self
    }
    pub fn initial_value(mut self, initial_value: u64) -> Self {
        self.inner.initial_value = initial_value;
        self
    }
}
impl<'a> Deref for SemaphoreTypeCreateInfoBuilder<'a> {
    type Target = vk::SemaphoreTypeCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::TimelineSemaphoreSubmitInfo {
    type Type = TimelineSemaphoreSubmitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct TimelineSemaphoreSubmitInfoBuilder<'a> {
    inner: vk::TimelineSemaphoreSubmitInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> TimelineSemaphoreSubmitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::SemaphoreWaitInfo {
    type Type = SemaphoreWaitInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreWaitInfoBuilder<'a> {
    inner: vk::SemaphoreWaitInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SemaphoreWaitInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::SemaphoreSignalInfo {
    type Type = SemaphoreSignalInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SemaphoreSignalInfoBuilder<'a> {
    inner: vk::SemaphoreSignalInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SemaphoreSignalInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn semaphore(mut self, semaphore: vk::Semaphore) -> Self {
        self.inner.semaphore = Some(semaphore);
        self
    }
    pub fn value(mut self, value: u64) -> Self {
        self.inner.value = value;
        self
    }
}
impl<'a> Deref for SemaphoreSignalInfoBuilder<'a> {
    type Target = vk::SemaphoreSignalInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineVertexInputDivisorStateCreateInfoEXT {
    type Type = PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineVertexInputDivisorStateCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_vertex_binding_divisors(
        mut self,
        p_vertex_binding_divisors: &'a [vk::VertexInputBindingDivisorDescriptionEXT],
    ) -> Self {
        self.inner.vertex_binding_divisor_count = p_vertex_binding_divisors.len() as u32;
        self.inner.p_vertex_binding_divisors = p_vertex_binding_divisors.as_ptr();
        self
    }
}
impl<'a> Deref for PipelineVertexInputDivisorStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineVertexInputDivisorStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImportAndroidHardwareBufferInfoANDROID {
    type Type = ImportAndroidHardwareBufferInfoANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    inner: vk::ImportAndroidHardwareBufferInfoANDROID,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImportAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::MemoryGetAndroidHardwareBufferInfoANDROID {
    type Type = MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    inner: vk::MemoryGetAndroidHardwareBufferInfoANDROID,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
}
impl<'a> Deref for MemoryGetAndroidHardwareBufferInfoANDROIDBuilder<'a> {
    type Target = vk::MemoryGetAndroidHardwareBufferInfoANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceConditionalRenderingInfoEXT {
    type Type = CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    inner: vk::CommandBufferInheritanceConditionalRenderingInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conditional_rendering_enable(mut self, conditional_rendering_enable: bool) -> Self {
        self.inner.conditional_rendering_enable = if conditional_rendering_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceConditionalRenderingInfoEXTBuilder<'a> {
    type Target = vk::CommandBufferInheritanceConditionalRenderingInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ExternalFormatANDROID {
    type Type = ExternalFormatANDROIDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ExternalFormatANDROIDBuilder<'a> {
    inner: vk::ExternalFormatANDROID,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ExternalFormatANDROIDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn external_format(mut self, external_format: u64) -> Self {
        self.inner.external_format = external_format;
        self
    }
}
impl<'a> Deref for ExternalFormatANDROIDBuilder<'a> {
    type Target = vk::ExternalFormatANDROID;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDevice8BitStorageFeatures {
    type Type = PhysicalDevice8BitStorageFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevice8BitStorageFeaturesBuilder<'a> {
    inner: vk::PhysicalDevice8BitStorageFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDevice8BitStorageFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.inner.storage_buffer8_bit_access = if storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer8_bit_access(mut self, uniform_and_storage_buffer8_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer8_bit_access = if uniform_and_storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.inner.storage_push_constant8 = if storage_push_constant8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDevice8BitStorageFeaturesBuilder<'a> {
    type Target = vk::PhysicalDevice8BitStorageFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceConditionalRenderingFeaturesEXT {
    type Type = PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceConditionalRenderingFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn conditional_rendering(mut self, conditional_rendering: bool) -> Self {
        self.inner.conditional_rendering = if conditional_rendering { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn inherited_conditional_rendering(mut self, inherited_conditional_rendering: bool) -> Self {
        self.inner.inherited_conditional_rendering = if inherited_conditional_rendering {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceConditionalRenderingFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceConditionalRenderingFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceVulkanMemoryModelFeatures {
    type Type = PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceVulkanMemoryModelFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.inner.vulkan_memory_model = if vulkan_memory_model { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn vulkan_memory_model_device_scope(mut self, vulkan_memory_model_device_scope: bool) -> Self {
        self.inner.vulkan_memory_model_device_scope = if vulkan_memory_model_device_scope {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.inner.vulkan_memory_model_availability_visibility_chains =
            if vulkan_memory_model_availability_visibility_chains {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
}
impl<'a> Deref for PhysicalDeviceVulkanMemoryModelFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceVulkanMemoryModelFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderAtomicInt64Features {
    type Type = PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceShaderAtomicInt64Features,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.inner.shader_buffer_int64_atomics = if shader_buffer_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.inner.shader_shared_int64_atomics = if shader_shared_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderAtomicInt64FeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderAtomicInt64Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT {
    type Type = PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_attribute_instance_rate_divisor(mut self, vertex_attribute_instance_rate_divisor: bool) -> Self {
        self.inner.vertex_attribute_instance_rate_divisor = if vertex_attribute_instance_rate_divisor {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vertex_attribute_instance_rate_zero_divisor(
        mut self,
        vertex_attribute_instance_rate_zero_divisor: bool,
    ) -> Self {
        self.inner.vertex_attribute_instance_rate_zero_divisor = if vertex_attribute_instance_rate_zero_divisor {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceVertexAttributeDivisorFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceVertexAttributeDivisorFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SubpassDescriptionDepthStencilResolve {
    type Type = SubpassDescriptionDepthStencilResolveBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SubpassDescriptionDepthStencilResolveBuilder<'a> {
    inner: vk::SubpassDescriptionDepthStencilResolve,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SubpassDescriptionDepthStencilResolveBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
        self.inner.p_depth_stencil_resolve_attachment = p_depth_stencil_resolve_attachment.map_or(ptr::null(), |p| p);
        self
    }
}
impl<'a> Deref for SubpassDescriptionDepthStencilResolveBuilder<'a> {
    type Target = vk::SubpassDescriptionDepthStencilResolve;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageViewASTCDecodeModeEXT {
    type Type = ImageViewASTCDecodeModeEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewASTCDecodeModeEXTBuilder<'a> {
    inner: vk::ImageViewASTCDecodeModeEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageViewASTCDecodeModeEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn decode_mode(mut self, decode_mode: vk::Format) -> Self {
        self.inner.decode_mode = decode_mode;
        self
    }
}
impl<'a> Deref for ImageViewASTCDecodeModeEXTBuilder<'a> {
    type Target = vk::ImageViewASTCDecodeModeEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceASTCDecodeFeaturesEXT {
    type Type = PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceASTCDecodeFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn decode_mode_shared_exponent(mut self, decode_mode_shared_exponent: bool) -> Self {
        self.inner.decode_mode_shared_exponent = if decode_mode_shared_exponent {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceASTCDecodeFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceASTCDecodeFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceTransformFeedbackFeaturesEXT {
    type Type = PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceTransformFeedbackFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform_feedback(mut self, transform_feedback: bool) -> Self {
        self.inner.transform_feedback = if transform_feedback { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn geometry_streams(mut self, geometry_streams: bool) -> Self {
        self.inner.geometry_streams = if geometry_streams { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceTransformFeedbackFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceTransformFeedbackFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineRasterizationStateStreamCreateInfoEXT {
    type Type = PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineRasterizationStateStreamCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationStateStreamCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn rasterization_stream(mut self, rasterization_stream: u32) -> Self {
        self.inner.rasterization_stream = rasterization_stream;
        self
    }
}
impl<'a> Deref for PipelineRasterizationStateStreamCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineRasterizationStateStreamCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    type Type = PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn representative_fragment_test(mut self, representative_fragment_test: bool) -> Self {
        self.inner.representative_fragment_test = if representative_fragment_test {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceRepresentativeFragmentTestFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceRepresentativeFragmentTestFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineRepresentativeFragmentTestStateCreateInfoNV {
    type Type = PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineRepresentativeFragmentTestStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn representative_fragment_test_enable(mut self, representative_fragment_test_enable: bool) -> Self {
        self.inner.representative_fragment_test_enable = if representative_fragment_test_enable {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PipelineRepresentativeFragmentTestStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineRepresentativeFragmentTestStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceExclusiveScissorFeaturesNV {
    type Type = PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceExclusiveScissorFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn exclusive_scissor(mut self, exclusive_scissor: bool) -> Self {
        self.inner.exclusive_scissor = if exclusive_scissor { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceExclusiveScissorFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceExclusiveScissorFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineViewportExclusiveScissorStateCreateInfoNV {
    type Type = PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportExclusiveScissorStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineViewportExclusiveScissorStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
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
impl<'a> Builder<'a> for vk::PhysicalDeviceCornerSampledImageFeaturesNV {
    type Type = PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceCornerSampledImageFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn corner_sampled_image(mut self, corner_sampled_image: bool) -> Self {
        self.inner.corner_sampled_image = if corner_sampled_image { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceCornerSampledImageFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceCornerSampledImageFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    type Type = PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compute_derivative_group_quads(mut self, compute_derivative_group_quads: bool) -> Self {
        self.inner.compute_derivative_group_quads = if compute_derivative_group_quads {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn compute_derivative_group_linear(mut self, compute_derivative_group_linear: bool) -> Self {
        self.inner.compute_derivative_group_linear = if compute_derivative_group_linear {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceComputeShaderDerivativesFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceComputeShaderDerivativesFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    type Type = PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_shader_barycentric(mut self, fragment_shader_barycentric: bool) -> Self {
        self.inner.fragment_shader_barycentric = if fragment_shader_barycentric {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceFragmentShaderBarycentricFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceFragmentShaderBarycentricFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderImageFootprintFeaturesNV {
    type Type = PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceShaderImageFootprintFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_footprint(mut self, image_footprint: bool) -> Self {
        self.inner.image_footprint = if image_footprint { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderImageFootprintFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderImageFootprintFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    type Type = PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn dedicated_allocation_image_aliasing(mut self, dedicated_allocation_image_aliasing: bool) -> Self {
        self.inner.dedicated_allocation_image_aliasing = if dedicated_allocation_image_aliasing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ShadingRatePaletteNV {
    type Type = ShadingRatePaletteNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ShadingRatePaletteNVBuilder<'a> {
    inner: vk::ShadingRatePaletteNV,
    phantom: PhantomData<&'a vk::ShadingRatePaletteEntryNV>,
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
impl<'a> Builder<'a> for vk::PipelineViewportShadingRateImageStateCreateInfoNV {
    type Type = PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportShadingRateImageStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineViewportShadingRateImageStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_image_enable(mut self, shading_rate_image_enable: bool) -> Self {
        self.inner.shading_rate_image_enable = if shading_rate_image_enable { vk::TRUE } else { vk::FALSE };
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
impl<'a> Builder<'a> for vk::PhysicalDeviceShadingRateImageFeaturesNV {
    type Type = PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceShadingRateImageFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shading_rate_image(mut self, shading_rate_image: bool) -> Self {
        self.inner.shading_rate_image = if shading_rate_image { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shading_rate_coarse_sample_order(mut self, shading_rate_coarse_sample_order: bool) -> Self {
        self.inner.shading_rate_coarse_sample_order = if shading_rate_coarse_sample_order {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShadingRateImageFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceShadingRateImageFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CoarseSampleOrderCustomNV {
    type Type = CoarseSampleOrderCustomNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CoarseSampleOrderCustomNVBuilder<'a> {
    inner: vk::CoarseSampleOrderCustomNV,
    phantom: PhantomData<&'a vk::CoarseSampleLocationNV>,
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
impl<'a> Builder<'a> for vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV {
    type Type = PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineViewportCoarseSampleOrderStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineViewportCoarseSampleOrderStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PhysicalDeviceMeshShaderFeaturesNV {
    type Type = PhysicalDeviceMeshShaderFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceMeshShaderFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn task_shader(mut self, task_shader: bool) -> Self {
        self.inner.task_shader = if task_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn mesh_shader(mut self, mesh_shader: bool) -> Self {
        self.inner.mesh_shader = if mesh_shader { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceMeshShaderFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceMeshShaderFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RayTracingShaderGroupCreateInfoNV {
    type Type = RayTracingShaderGroupCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    inner: vk::RayTracingShaderGroupCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
}
impl<'a> Deref for RayTracingShaderGroupCreateInfoNVBuilder<'a> {
    type Target = vk::RayTracingShaderGroupCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RayTracingShaderGroupCreateInfoKHR {
    type Type = RayTracingShaderGroupCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    inner: vk::RayTracingShaderGroupCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RayTracingShaderGroupCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::RayTracingPipelineCreateInfoNV {
    type Type = RayTracingPipelineCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingPipelineCreateInfoNVBuilder<'a> {
    inner: vk::RayTracingPipelineCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RayTracingPipelineCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
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
impl<'a> Builder<'a> for vk::RayTracingPipelineCreateInfoKHR {
    type Type = RayTracingPipelineCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingPipelineCreateInfoKHRBuilder<'a> {
    inner: vk::RayTracingPipelineCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RayTracingPipelineCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
    pub fn max_recursion_depth(mut self, max_recursion_depth: u32) -> Self {
        self.inner.max_recursion_depth = max_recursion_depth;
        self
    }
    pub fn libraries(mut self, libraries: vk::PipelineLibraryCreateInfoKHR) -> Self {
        self.inner.libraries = libraries;
        self
    }
    pub fn p_library_interface(
        mut self,
        p_library_interface: Option<&'a vk::RayTracingPipelineInterfaceCreateInfoKHR>,
    ) -> Self {
        self.inner.p_library_interface = p_library_interface.map_or(ptr::null(), |p| p);
        self
    }
    pub fn layout(mut self, layout: vk::PipelineLayout) -> Self {
        self.inner.layout = Some(layout);
        self
    }
    pub fn base_pipeline_handle(mut self, base_pipeline_handle: Option<vk::Pipeline>) -> Self {
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
impl<'a> Builder<'a> for vk::GeometryTrianglesNV {
    type Type = GeometryTrianglesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryTrianglesNVBuilder<'a> {
    inner: vk::GeometryTrianglesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> GeometryTrianglesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_data(mut self, vertex_data: Option<vk::Buffer>) -> Self {
        self.inner.vertex_data = vertex_data;
        self
    }
    pub fn vertex_offset(mut self, vertex_offset: vk::DeviceSize) -> Self {
        self.inner.vertex_offset = vertex_offset;
        self
    }
    pub fn vertex_count(mut self, vertex_count: u32) -> Self {
        self.inner.vertex_count = vertex_count;
        self
    }
    pub fn vertex_stride(mut self, vertex_stride: vk::DeviceSize) -> Self {
        self.inner.vertex_stride = vertex_stride;
        self
    }
    pub fn vertex_format(mut self, vertex_format: vk::Format) -> Self {
        self.inner.vertex_format = vertex_format;
        self
    }
    pub fn index_data(mut self, index_data: Option<vk::Buffer>) -> Self {
        self.inner.index_data = index_data;
        self
    }
    pub fn index_offset(mut self, index_offset: vk::DeviceSize) -> Self {
        self.inner.index_offset = index_offset;
        self
    }
    pub fn index_count(mut self, index_count: u32) -> Self {
        self.inner.index_count = index_count;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn transform_data(mut self, transform_data: Option<vk::Buffer>) -> Self {
        self.inner.transform_data = transform_data;
        self
    }
    pub fn transform_offset(mut self, transform_offset: vk::DeviceSize) -> Self {
        self.inner.transform_offset = transform_offset;
        self
    }
}
impl<'a> Deref for GeometryTrianglesNVBuilder<'a> {
    type Target = vk::GeometryTrianglesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GeometryAABBNV {
    type Type = GeometryAABBNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryAABBNVBuilder<'a> {
    inner: vk::GeometryAABBNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> GeometryAABBNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn aabb_data(mut self, aabb_data: Option<vk::Buffer>) -> Self {
        self.inner.aabb_data = aabb_data;
        self
    }
    pub fn num_aab_bs(mut self, num_aab_bs: u32) -> Self {
        self.inner.num_aab_bs = num_aab_bs;
        self
    }
    pub fn stride(mut self, stride: u32) -> Self {
        self.inner.stride = stride;
        self
    }
    pub fn offset(mut self, offset: vk::DeviceSize) -> Self {
        self.inner.offset = offset;
        self
    }
}
impl<'a> Deref for GeometryAABBNVBuilder<'a> {
    type Target = vk::GeometryAABBNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::GeometryNV {
    type Type = GeometryNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct GeometryNVBuilder<'a> {
    inner: vk::GeometryNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> GeometryNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn geometry_type(mut self, geometry_type: vk::GeometryTypeKHR) -> Self {
        self.inner.geometry_type = geometry_type;
        self
    }
    pub fn geometry(mut self, geometry: vk::GeometryDataNV) -> Self {
        self.inner.geometry = geometry;
        self
    }
    pub fn flags(mut self, flags: vk::GeometryFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for GeometryNVBuilder<'a> {
    type Target = vk::GeometryNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureInfoNV {
    type Type = AccelerationStructureInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureInfoNVBuilder<'a> {
    inner: vk::AccelerationStructureInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::AccelerationStructureCreateInfoNV {
    type Type = AccelerationStructureCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureCreateInfoNVBuilder<'a> {
    inner: vk::AccelerationStructureCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compacted_size(mut self, compacted_size: vk::DeviceSize) -> Self {
        self.inner.compacted_size = compacted_size;
        self
    }
    pub fn info(mut self, info: vk::AccelerationStructureInfoNV) -> Self {
        self.inner.info = info;
        self
    }
}
impl<'a> Deref for AccelerationStructureCreateInfoNVBuilder<'a> {
    type Target = vk::AccelerationStructureCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BindAccelerationStructureMemoryInfoKHR {
    type Type = BindAccelerationStructureMemoryInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
    inner: vk::BindAccelerationStructureMemoryInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureKHR) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
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
impl<'a> Deref for BindAccelerationStructureMemoryInfoKHRBuilder<'a> {
    type Target = vk::BindAccelerationStructureMemoryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::WriteDescriptorSetAccelerationStructureKHR {
    type Type = WriteDescriptorSetAccelerationStructureKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    inner: vk::WriteDescriptorSetAccelerationStructureKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> WriteDescriptorSetAccelerationStructureKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::AccelerationStructureMemoryRequirementsInfoKHR {
    type Type = AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureMemoryRequirementsInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureMemoryRequirementsTypeKHR) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn build_type(mut self, build_type: vk::AccelerationStructureBuildTypeKHR) -> Self {
        self.inner.build_type = build_type;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureKHR) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
}
impl<'a> Deref for AccelerationStructureMemoryRequirementsInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureMemoryRequirementsInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureMemoryRequirementsInfoNV {
    type Type = AccelerationStructureMemoryRequirementsInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    inner: vk::AccelerationStructureMemoryRequirementsInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::AccelerationStructureMemoryRequirementsTypeNV) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureNV) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
}
impl<'a> Deref for AccelerationStructureMemoryRequirementsInfoNVBuilder<'a> {
    type Target = vk::AccelerationStructureMemoryRequirementsInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceRayTracingFeaturesKHR {
    type Type = PhysicalDeviceRayTracingFeaturesKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
    inner: vk::PhysicalDeviceRayTracingFeaturesKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ray_tracing(mut self, ray_tracing: bool) -> Self {
        self.inner.ray_tracing = if ray_tracing { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn ray_tracing_shader_group_handle_capture_replay(
        mut self,
        ray_tracing_shader_group_handle_capture_replay: bool,
    ) -> Self {
        self.inner.ray_tracing_shader_group_handle_capture_replay = if ray_tracing_shader_group_handle_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn ray_tracing_shader_group_handle_capture_replay_mixed(
        mut self,
        ray_tracing_shader_group_handle_capture_replay_mixed: bool,
    ) -> Self {
        self.inner.ray_tracing_shader_group_handle_capture_replay_mixed =
            if ray_tracing_shader_group_handle_capture_replay_mixed {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn ray_tracing_acceleration_structure_capture_replay(
        mut self,
        ray_tracing_acceleration_structure_capture_replay: bool,
    ) -> Self {
        self.inner.ray_tracing_acceleration_structure_capture_replay =
            if ray_tracing_acceleration_structure_capture_replay {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn ray_tracing_indirect_trace_rays(mut self, ray_tracing_indirect_trace_rays: bool) -> Self {
        self.inner.ray_tracing_indirect_trace_rays = if ray_tracing_indirect_trace_rays {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn ray_tracing_indirect_acceleration_structure_build(
        mut self,
        ray_tracing_indirect_acceleration_structure_build: bool,
    ) -> Self {
        self.inner.ray_tracing_indirect_acceleration_structure_build =
            if ray_tracing_indirect_acceleration_structure_build {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn ray_tracing_host_acceleration_structure_commands(
        mut self,
        ray_tracing_host_acceleration_structure_commands: bool,
    ) -> Self {
        self.inner.ray_tracing_host_acceleration_structure_commands =
            if ray_tracing_host_acceleration_structure_commands {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn ray_query(mut self, ray_query: bool) -> Self {
        self.inner.ray_query = if ray_query { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn ray_tracing_primitive_culling(mut self, ray_tracing_primitive_culling: bool) -> Self {
        self.inner.ray_tracing_primitive_culling = if ray_tracing_primitive_culling {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceRayTracingFeaturesKHRBuilder<'a> {
    type Target = vk::PhysicalDeviceRayTracingFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceImageDrmFormatModifierInfoEXT {
    type Type = PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    inner: vk::PhysicalDeviceImageDrmFormatModifierInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceImageDrmFormatModifierInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ImageDrmFormatModifierListCreateInfoEXT {
    type Type = ImageDrmFormatModifierListCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    inner: vk::ImageDrmFormatModifierListCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageDrmFormatModifierListCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ImageDrmFormatModifierExplicitCreateInfoEXT {
    type Type = ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    inner: vk::ImageDrmFormatModifierExplicitCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageDrmFormatModifierExplicitCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::ImageStencilUsageCreateInfo {
    type Type = ImageStencilUsageCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageStencilUsageCreateInfoBuilder<'a> {
    inner: vk::ImageStencilUsageCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageStencilUsageCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_usage(mut self, stencil_usage: vk::ImageUsageFlags) -> Self {
        self.inner.stencil_usage = stencil_usage;
        self
    }
}
impl<'a> Deref for ImageStencilUsageCreateInfoBuilder<'a> {
    type Target = vk::ImageStencilUsageCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceMemoryOverallocationCreateInfoAMD {
    type Type = DeviceMemoryOverallocationCreateInfoAMDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    inner: vk::DeviceMemoryOverallocationCreateInfoAMD,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn overallocation_behavior(mut self, overallocation_behavior: vk::MemoryOverallocationBehaviorAMD) -> Self {
        self.inner.overallocation_behavior = overallocation_behavior;
        self
    }
}
impl<'a> Deref for DeviceMemoryOverallocationCreateInfoAMDBuilder<'a> {
    type Target = vk::DeviceMemoryOverallocationCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RenderPassFragmentDensityMapCreateInfoEXT {
    type Type = RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    inner: vk::RenderPassFragmentDensityMapCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_density_map_attachment(mut self, fragment_density_map_attachment: vk::AttachmentReference) -> Self {
        self.inner.fragment_density_map_attachment = fragment_density_map_attachment;
        self
    }
}
impl<'a> Deref for RenderPassFragmentDensityMapCreateInfoEXTBuilder<'a> {
    type Target = vk::RenderPassFragmentDensityMapCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceScalarBlockLayoutFeatures {
    type Type = PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceScalarBlockLayoutFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.inner.scalar_block_layout = if scalar_block_layout { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceScalarBlockLayoutFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceScalarBlockLayoutFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SurfaceProtectedCapabilitiesKHR {
    type Type = SurfaceProtectedCapabilitiesKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    inner: vk::SurfaceProtectedCapabilitiesKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn supports_protected(mut self, supports_protected: bool) -> Self {
        self.inner.supports_protected = if supports_protected { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for SurfaceProtectedCapabilitiesKHRBuilder<'a> {
    type Target = vk::SurfaceProtectedCapabilitiesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceUniformBufferStandardLayoutFeatures {
    type Type = PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceUniformBufferStandardLayoutFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.inner.uniform_buffer_standard_layout = if uniform_buffer_standard_layout {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceUniformBufferStandardLayoutFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceUniformBufferStandardLayoutFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceDepthClipEnableFeaturesEXT {
    type Type = PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceDepthClipEnableFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.inner.depth_clip_enable = if depth_clip_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceDepthClipEnableFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceDepthClipEnableFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineRasterizationDepthClipStateCreateInfoEXT {
    type Type = PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineRasterizationDepthClipStateCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineRasterizationDepthClipStateCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.inner.depth_clip_enable = if depth_clip_enable { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PipelineRasterizationDepthClipStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineRasterizationDepthClipStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceMemoryPriorityFeaturesEXT {
    type Type = PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceMemoryPriorityFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory_priority(mut self, memory_priority: bool) -> Self {
        self.inner.memory_priority = if memory_priority { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceMemoryPriorityFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceMemoryPriorityFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MemoryPriorityAllocateInfoEXT {
    type Type = MemoryPriorityAllocateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryPriorityAllocateInfoEXTBuilder<'a> {
    inner: vk::MemoryPriorityAllocateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryPriorityAllocateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn priority(mut self, priority: f32) -> Self {
        self.inner.priority = priority;
        self
    }
}
impl<'a> Deref for MemoryPriorityAllocateInfoEXTBuilder<'a> {
    type Target = vk::MemoryPriorityAllocateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceBufferDeviceAddressFeatures {
    type Type = PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceBufferDeviceAddressFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceBufferDeviceAddressFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceBufferDeviceAddressFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT {
    type Type = PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceBufferDeviceAddressFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceBufferDeviceAddressFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferDeviceAddressInfo {
    type Type = BufferDeviceAddressInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferDeviceAddressInfoBuilder<'a> {
    inner: vk::BufferDeviceAddressInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BufferDeviceAddressInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn buffer(mut self, buffer: vk::Buffer) -> Self {
        self.inner.buffer = Some(buffer);
        self
    }
}
impl<'a> Deref for BufferDeviceAddressInfoBuilder<'a> {
    type Target = vk::BufferDeviceAddressInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferOpaqueCaptureAddressCreateInfo {
    type Type = BufferOpaqueCaptureAddressCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
    inner: vk::BufferOpaqueCaptureAddressCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.inner.opaque_capture_address = opaque_capture_address;
        self
    }
}
impl<'a> Deref for BufferOpaqueCaptureAddressCreateInfoBuilder<'a> {
    type Target = vk::BufferOpaqueCaptureAddressCreateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::BufferDeviceAddressCreateInfoEXT {
    type Type = BufferDeviceAddressCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    inner: vk::BufferDeviceAddressCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_address(mut self, device_address: vk::DeviceAddress) -> Self {
        self.inner.device_address = device_address;
        self
    }
}
impl<'a> Deref for BufferDeviceAddressCreateInfoEXTBuilder<'a> {
    type Target = vk::BufferDeviceAddressCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceImageViewImageFormatInfoEXT {
    type Type = PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    inner: vk::PhysicalDeviceImageViewImageFormatInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view_type(mut self, image_view_type: vk::ImageViewType) -> Self {
        self.inner.image_view_type = image_view_type;
        self
    }
}
impl<'a> Deref for PhysicalDeviceImageViewImageFormatInfoEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceImageViewImageFormatInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceImagelessFramebufferFeatures {
    type Type = PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceImagelessFramebufferFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.inner.imageless_framebuffer = if imageless_framebuffer { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceImagelessFramebufferFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceImagelessFramebufferFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::FramebufferAttachmentsCreateInfo {
    type Type = FramebufferAttachmentsCreateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FramebufferAttachmentsCreateInfoBuilder<'a> {
    inner: vk::FramebufferAttachmentsCreateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> FramebufferAttachmentsCreateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::FramebufferAttachmentImageInfo {
    type Type = FramebufferAttachmentImageInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct FramebufferAttachmentImageInfoBuilder<'a> {
    inner: vk::FramebufferAttachmentImageInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> FramebufferAttachmentImageInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
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
impl<'a> Builder<'a> for vk::RenderPassAttachmentBeginInfo {
    type Type = RenderPassAttachmentBeginInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassAttachmentBeginInfoBuilder<'a> {
    inner: vk::RenderPassAttachmentBeginInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassAttachmentBeginInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
    type Type = PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn texture_compression_astc_hdr(mut self, texture_compression_astc_hdr: bool) -> Self {
        self.inner.texture_compression_astc_hdr = if texture_compression_astc_hdr {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceTextureCompressionASTCHDRFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceTextureCompressionASTCHDRFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceCooperativeMatrixFeaturesNV {
    type Type = PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceCooperativeMatrixFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn cooperative_matrix(mut self, cooperative_matrix: bool) -> Self {
        self.inner.cooperative_matrix = if cooperative_matrix { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn cooperative_matrix_robust_buffer_access(mut self, cooperative_matrix_robust_buffer_access: bool) -> Self {
        self.inner.cooperative_matrix_robust_buffer_access = if cooperative_matrix_robust_buffer_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceCooperativeMatrixFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceCooperativeMatrixFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CooperativeMatrixPropertiesNV {
    type Type = CooperativeMatrixPropertiesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CooperativeMatrixPropertiesNVBuilder<'a> {
    inner: vk::CooperativeMatrixPropertiesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CooperativeMatrixPropertiesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn m_size(mut self, m_size: u32) -> Self {
        self.inner.m_size = m_size;
        self
    }
    pub fn n_size(mut self, n_size: u32) -> Self {
        self.inner.n_size = n_size;
        self
    }
    pub fn k_size(mut self, k_size: u32) -> Self {
        self.inner.k_size = k_size;
        self
    }
    pub fn a_type(mut self, a_type: vk::ComponentTypeNV) -> Self {
        self.inner.a_type = a_type;
        self
    }
    pub fn b_type(mut self, b_type: vk::ComponentTypeNV) -> Self {
        self.inner.b_type = b_type;
        self
    }
    pub fn c_type(mut self, c_type: vk::ComponentTypeNV) -> Self {
        self.inner.c_type = c_type;
        self
    }
    pub fn d_type(mut self, d_type: vk::ComponentTypeNV) -> Self {
        self.inner.d_type = d_type;
        self
    }
    pub fn scope(mut self, scope: vk::ScopeNV) -> Self {
        self.inner.scope = scope;
        self
    }
}
impl<'a> Deref for CooperativeMatrixPropertiesNVBuilder<'a> {
    type Target = vk::CooperativeMatrixPropertiesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT {
    type Type = PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ycbcr_image_arrays(mut self, ycbcr_image_arrays: bool) -> Self {
        self.inner.ycbcr_image_arrays = if ycbcr_image_arrays { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceYcbcrImageArraysFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceYcbcrImageArraysFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::ImageViewHandleInfoNVX {
    type Type = ImageViewHandleInfoNVXBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct ImageViewHandleInfoNVXBuilder<'a> {
    inner: vk::ImageViewHandleInfoNVX,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> ImageViewHandleInfoNVXBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn image_view(mut self, image_view: vk::ImageView) -> Self {
        self.inner.image_view = Some(image_view);
        self
    }
    pub fn descriptor_type(mut self, descriptor_type: vk::DescriptorType) -> Self {
        self.inner.descriptor_type = descriptor_type;
        self
    }
    pub fn sampler(mut self, sampler: Option<vk::Sampler>) -> Self {
        self.inner.sampler = sampler;
        self
    }
}
impl<'a> Deref for ImageViewHandleInfoNVXBuilder<'a> {
    type Target = vk::ImageViewHandleInfoNVX;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineCreationFeedbackCreateInfoEXT {
    type Type = PipelineCreationFeedbackCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineCreationFeedbackCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn p_pipeline_creation_feedback(
        mut self,
        p_pipeline_creation_feedback: *mut vk::PipelineCreationFeedbackEXT,
    ) -> Self {
        self.inner.p_pipeline_creation_feedback = p_pipeline_creation_feedback;
        self
    }
    pub fn pipeline_stage_creation_feedback_count(mut self, pipeline_stage_creation_feedback_count: u32) -> Self {
        self.inner.pipeline_stage_creation_feedback_count = pipeline_stage_creation_feedback_count;
        self
    }
    pub fn p_pipeline_stage_creation_feedbacks(
        mut self,
        p_pipeline_stage_creation_feedbacks: *mut vk::PipelineCreationFeedbackEXT,
    ) -> Self {
        self.inner.p_pipeline_stage_creation_feedbacks = p_pipeline_stage_creation_feedbacks;
        self
    }
}
impl<'a> Deref for PipelineCreationFeedbackCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineCreationFeedbackCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SurfaceFullScreenExclusiveInfoEXT {
    type Type = SurfaceFullScreenExclusiveInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    inner: vk::SurfaceFullScreenExclusiveInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn full_screen_exclusive(mut self, full_screen_exclusive: vk::FullScreenExclusiveEXT) -> Self {
        self.inner.full_screen_exclusive = full_screen_exclusive;
        self
    }
}
impl<'a> Deref for SurfaceFullScreenExclusiveInfoEXTBuilder<'a> {
    type Target = vk::SurfaceFullScreenExclusiveInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SurfaceFullScreenExclusiveWin32InfoEXT {
    type Type = SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    inner: vk::SurfaceFullScreenExclusiveWin32InfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn hmonitor(mut self, hmonitor: vk::HMONITOR) -> Self {
        self.inner.hmonitor = hmonitor;
        self
    }
}
impl<'a> Deref for SurfaceFullScreenExclusiveWin32InfoEXTBuilder<'a> {
    type Target = vk::SurfaceFullScreenExclusiveWin32InfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::SurfaceCapabilitiesFullScreenExclusiveEXT {
    type Type = SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    inner: vk::SurfaceCapabilitiesFullScreenExclusiveEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn full_screen_exclusive_supported(mut self, full_screen_exclusive_supported: bool) -> Self {
        self.inner.full_screen_exclusive_supported = if full_screen_exclusive_supported {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for SurfaceCapabilitiesFullScreenExclusiveEXTBuilder<'a> {
    type Target = vk::SurfaceCapabilitiesFullScreenExclusiveEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDevicePerformanceQueryFeaturesKHR {
    type Type = PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    inner: vk::PhysicalDevicePerformanceQueryFeaturesKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn performance_counter_query_pools(mut self, performance_counter_query_pools: bool) -> Self {
        self.inner.performance_counter_query_pools = if performance_counter_query_pools {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn performance_counter_multiple_query_pools(mut self, performance_counter_multiple_query_pools: bool) -> Self {
        self.inner.performance_counter_multiple_query_pools = if performance_counter_multiple_query_pools {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDevicePerformanceQueryFeaturesKHRBuilder<'a> {
    type Target = vk::PhysicalDevicePerformanceQueryFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::QueryPoolPerformanceCreateInfoKHR {
    type Type = QueryPoolPerformanceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    inner: vk::QueryPoolPerformanceCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> QueryPoolPerformanceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::AcquireProfilingLockInfoKHR {
    type Type = AcquireProfilingLockInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AcquireProfilingLockInfoKHRBuilder<'a> {
    inner: vk::AcquireProfilingLockInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AcquireProfilingLockInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::AcquireProfilingLockFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn timeout(mut self, timeout: u64) -> Self {
        self.inner.timeout = timeout;
        self
    }
}
impl<'a> Deref for AcquireProfilingLockInfoKHRBuilder<'a> {
    type Target = vk::AcquireProfilingLockInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PerformanceQuerySubmitInfoKHR {
    type Type = PerformanceQuerySubmitInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceQuerySubmitInfoKHRBuilder<'a> {
    inner: vk::PerformanceQuerySubmitInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PerformanceQuerySubmitInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn counter_pass_index(mut self, counter_pass_index: u32) -> Self {
        self.inner.counter_pass_index = counter_pass_index;
        self
    }
}
impl<'a> Deref for PerformanceQuerySubmitInfoKHRBuilder<'a> {
    type Target = vk::PerformanceQuerySubmitInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::HeadlessSurfaceCreateInfoEXT {
    type Type = HeadlessSurfaceCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    inner: vk::HeadlessSurfaceCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::HeadlessSurfaceCreateFlagsEXT) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for HeadlessSurfaceCreateInfoEXTBuilder<'a> {
    type Target = vk::HeadlessSurfaceCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceCoverageReductionModeFeaturesNV {
    type Type = PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceCoverageReductionModeFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: bool) -> Self {
        self.inner.coverage_reduction_mode = if coverage_reduction_mode { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceCoverageReductionModeFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceCoverageReductionModeFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineCoverageReductionStateCreateInfoNV {
    type Type = PipelineCoverageReductionStateCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    inner: vk::PipelineCoverageReductionStateCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::PipelineCoverageReductionStateCreateFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
    pub fn coverage_reduction_mode(mut self, coverage_reduction_mode: vk::CoverageReductionModeNV) -> Self {
        self.inner.coverage_reduction_mode = coverage_reduction_mode;
        self
    }
}
impl<'a> Deref for PipelineCoverageReductionStateCreateInfoNVBuilder<'a> {
    type Target = vk::PipelineCoverageReductionStateCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    type Type = PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    inner: vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_integer_functions2(mut self, shader_integer_functions2: bool) -> Self {
        self.inner.shader_integer_functions2 = if shader_integer_functions2 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderIntegerFunctions2FeaturesINTELBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::InitializePerformanceApiInfoINTEL {
    type Type = InitializePerformanceApiInfoINTELBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct InitializePerformanceApiInfoINTELBuilder<'a> {
    inner: vk::InitializePerformanceApiInfoINTEL,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> InitializePerformanceApiInfoINTELBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::QueryPoolPerformanceQueryCreateInfoINTEL {
    type Type = QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    inner: vk::QueryPoolPerformanceQueryCreateInfoINTEL,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn performance_counters_sampling(
        mut self,
        performance_counters_sampling: vk::QueryPoolSamplingModeINTEL,
    ) -> Self {
        self.inner.performance_counters_sampling = performance_counters_sampling;
        self
    }
}
impl<'a> Deref for QueryPoolPerformanceQueryCreateInfoINTELBuilder<'a> {
    type Target = vk::QueryPoolPerformanceQueryCreateInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PerformanceMarkerInfoINTEL {
    type Type = PerformanceMarkerInfoINTELBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceMarkerInfoINTELBuilder<'a> {
    inner: vk::PerformanceMarkerInfoINTEL,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PerformanceMarkerInfoINTELBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn marker(mut self, marker: u64) -> Self {
        self.inner.marker = marker;
        self
    }
}
impl<'a> Deref for PerformanceMarkerInfoINTELBuilder<'a> {
    type Target = vk::PerformanceMarkerInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PerformanceStreamMarkerInfoINTEL {
    type Type = PerformanceStreamMarkerInfoINTELBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceStreamMarkerInfoINTELBuilder<'a> {
    inner: vk::PerformanceStreamMarkerInfoINTEL,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PerformanceStreamMarkerInfoINTELBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn marker(mut self, marker: u32) -> Self {
        self.inner.marker = marker;
        self
    }
}
impl<'a> Deref for PerformanceStreamMarkerInfoINTELBuilder<'a> {
    type Target = vk::PerformanceStreamMarkerInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PerformanceOverrideInfoINTEL {
    type Type = PerformanceOverrideInfoINTELBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceOverrideInfoINTELBuilder<'a> {
    inner: vk::PerformanceOverrideInfoINTEL,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PerformanceOverrideInfoINTELBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::PerformanceOverrideTypeINTEL) -> Self {
        self.inner.ty = ty;
        self
    }
    pub fn enable(mut self, enable: bool) -> Self {
        self.inner.enable = if enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn parameter(mut self, parameter: u64) -> Self {
        self.inner.parameter = parameter;
        self
    }
}
impl<'a> Deref for PerformanceOverrideInfoINTELBuilder<'a> {
    type Target = vk::PerformanceOverrideInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PerformanceConfigurationAcquireInfoINTEL {
    type Type = PerformanceConfigurationAcquireInfoINTELBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    inner: vk::PerformanceConfigurationAcquireInfoINTEL,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn ty(mut self, ty: vk::PerformanceConfigurationTypeINTEL) -> Self {
        self.inner.ty = ty;
        self
    }
}
impl<'a> Deref for PerformanceConfigurationAcquireInfoINTELBuilder<'a> {
    type Target = vk::PerformanceConfigurationAcquireInfoINTEL;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderClockFeaturesKHR {
    type Type = PhysicalDeviceShaderClockFeaturesKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    inner: vk::PhysicalDeviceShaderClockFeaturesKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_subgroup_clock(mut self, shader_subgroup_clock: bool) -> Self {
        self.inner.shader_subgroup_clock = if shader_subgroup_clock { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_device_clock(mut self, shader_device_clock: bool) -> Self {
        self.inner.shader_device_clock = if shader_device_clock { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderClockFeaturesKHRBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderClockFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceIndexTypeUint8FeaturesEXT {
    type Type = PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceIndexTypeUint8FeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn index_type_uint8(mut self, index_type_uint8: bool) -> Self {
        self.inner.index_type_uint8 = if index_type_uint8 { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceIndexTypeUint8FeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceIndexTypeUint8FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV {
    type Type = PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_sm_builtins(mut self, shader_sm_builtins: bool) -> Self {
        self.inner.shader_sm_builtins = if shader_sm_builtins { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderSMBuiltinsFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderSMBuiltinsFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    type Type = PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn fragment_shader_sample_interlock(mut self, fragment_shader_sample_interlock: bool) -> Self {
        self.inner.fragment_shader_sample_interlock = if fragment_shader_sample_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn fragment_shader_pixel_interlock(mut self, fragment_shader_pixel_interlock: bool) -> Self {
        self.inner.fragment_shader_pixel_interlock = if fragment_shader_pixel_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn fragment_shader_shading_rate_interlock(mut self, fragment_shader_shading_rate_interlock: bool) -> Self {
        self.inner.fragment_shader_shading_rate_interlock = if fragment_shader_shading_rate_interlock {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceFragmentShaderInterlockFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceFragmentShaderInterlockFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    type Type = PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.inner.separate_depth_stencil_layouts = if separate_depth_stencil_layouts {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceSeparateDepthStencilLayoutsFeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AttachmentReferenceStencilLayout {
    type Type = AttachmentReferenceStencilLayoutBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentReferenceStencilLayoutBuilder<'a> {
    inner: vk::AttachmentReferenceStencilLayout,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AttachmentReferenceStencilLayoutBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_layout(mut self, stencil_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_layout = stencil_layout;
        self
    }
}
impl<'a> Deref for AttachmentReferenceStencilLayoutBuilder<'a> {
    type Target = vk::AttachmentReferenceStencilLayout;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AttachmentDescriptionStencilLayout {
    type Type = AttachmentDescriptionStencilLayoutBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AttachmentDescriptionStencilLayoutBuilder<'a> {
    inner: vk::AttachmentDescriptionStencilLayout,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AttachmentDescriptionStencilLayoutBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn stencil_initial_layout(mut self, stencil_initial_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_initial_layout = stencil_initial_layout;
        self
    }
    pub fn stencil_final_layout(mut self, stencil_final_layout: vk::ImageLayout) -> Self {
        self.inner.stencil_final_layout = stencil_final_layout;
        self
    }
}
impl<'a> Deref for AttachmentDescriptionStencilLayoutBuilder<'a> {
    type Target = vk::AttachmentDescriptionStencilLayout;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    type Type = PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
    inner: vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_executable_info(mut self, pipeline_executable_info: bool) -> Self {
        self.inner.pipeline_executable_info = if pipeline_executable_info { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDevicePipelineExecutablePropertiesFeaturesKHRBuilder<'a> {
    type Target = vk::PhysicalDevicePipelineExecutablePropertiesFeaturesKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineInfoKHR {
    type Type = PipelineInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineInfoKHRBuilder<'a> {
    inner: vk::PipelineInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
}
impl<'a> Deref for PipelineInfoKHRBuilder<'a> {
    type Target = vk::PipelineInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineExecutableInfoKHR {
    type Type = PipelineExecutableInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineExecutableInfoKHRBuilder<'a> {
    inner: vk::PipelineExecutableInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineExecutableInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline(mut self, pipeline: vk::Pipeline) -> Self {
        self.inner.pipeline = Some(pipeline);
        self
    }
    pub fn executable_index(mut self, executable_index: u32) -> Self {
        self.inner.executable_index = executable_index;
        self
    }
}
impl<'a> Deref for PipelineExecutableInfoKHRBuilder<'a> {
    type Target = vk::PipelineExecutableInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
    type Type = PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn shader_demote_to_helper_invocation(mut self, shader_demote_to_helper_invocation: bool) -> Self {
        self.inner.shader_demote_to_helper_invocation = if shader_demote_to_helper_invocation {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    type Type = PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn texel_buffer_alignment(mut self, texel_buffer_alignment: bool) -> Self {
        self.inner.texel_buffer_alignment = if texel_buffer_alignment { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceTexelBufferAlignmentFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceTexelBufferAlignmentFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceSubgroupSizeControlFeaturesEXT {
    type Type = PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceSubgroupSizeControlFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn subgroup_size_control(mut self, subgroup_size_control: bool) -> Self {
        self.inner.subgroup_size_control = if subgroup_size_control { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn compute_full_subgroups(mut self, compute_full_subgroups: bool) -> Self {
        self.inner.compute_full_subgroups = if compute_full_subgroups { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceSubgroupSizeControlFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceSubgroupSizeControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::MemoryOpaqueCaptureAddressAllocateInfo {
    type Type = MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
    inner: vk::MemoryOpaqueCaptureAddressAllocateInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn opaque_capture_address(mut self, opaque_capture_address: u64) -> Self {
        self.inner.opaque_capture_address = opaque_capture_address;
        self
    }
}
impl<'a> Deref for MemoryOpaqueCaptureAddressAllocateInfoBuilder<'a> {
    type Target = vk::MemoryOpaqueCaptureAddressAllocateInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceMemoryOpaqueCaptureAddressInfo {
    type Type = DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
    inner: vk::DeviceMemoryOpaqueCaptureAddressInfo,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn memory(mut self, memory: vk::DeviceMemory) -> Self {
        self.inner.memory = Some(memory);
        self
    }
}
impl<'a> Deref for DeviceMemoryOpaqueCaptureAddressInfoBuilder<'a> {
    type Target = vk::DeviceMemoryOpaqueCaptureAddressInfo;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceLineRasterizationFeaturesEXT {
    type Type = PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceLineRasterizationFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn rectangular_lines(mut self, rectangular_lines: bool) -> Self {
        self.inner.rectangular_lines = if rectangular_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn bresenham_lines(mut self, bresenham_lines: bool) -> Self {
        self.inner.bresenham_lines = if bresenham_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn smooth_lines(mut self, smooth_lines: bool) -> Self {
        self.inner.smooth_lines = if smooth_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stippled_rectangular_lines(mut self, stippled_rectangular_lines: bool) -> Self {
        self.inner.stippled_rectangular_lines = if stippled_rectangular_lines {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn stippled_bresenham_lines(mut self, stippled_bresenham_lines: bool) -> Self {
        self.inner.stippled_bresenham_lines = if stippled_bresenham_lines { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn stippled_smooth_lines(mut self, stippled_smooth_lines: bool) -> Self {
        self.inner.stippled_smooth_lines = if stippled_smooth_lines { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceLineRasterizationFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceLineRasterizationFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineRasterizationLineStateCreateInfoEXT {
    type Type = PipelineRasterizationLineStateCreateInfoEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    inner: vk::PipelineRasterizationLineStateCreateInfoEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn line_rasterization_mode(mut self, line_rasterization_mode: vk::LineRasterizationModeEXT) -> Self {
        self.inner.line_rasterization_mode = line_rasterization_mode;
        self
    }
    pub fn stippled_line_enable(mut self, stippled_line_enable: bool) -> Self {
        self.inner.stippled_line_enable = if stippled_line_enable { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn line_stipple_factor(mut self, line_stipple_factor: u32) -> Self {
        self.inner.line_stipple_factor = line_stipple_factor;
        self
    }
    pub fn line_stipple_pattern(mut self, line_stipple_pattern: u16) -> Self {
        self.inner.line_stipple_pattern = line_stipple_pattern;
        self
    }
}
impl<'a> Deref for PipelineRasterizationLineStateCreateInfoEXTBuilder<'a> {
    type Target = vk::PipelineRasterizationLineStateCreateInfoEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDevicePipelineCreationCacheControlFeaturesEXT {
    type Type = PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDevicePipelineCreationCacheControlFeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn pipeline_creation_cache_control(mut self, pipeline_creation_cache_control: bool) -> Self {
        self.inner.pipeline_creation_cache_control = if pipeline_creation_cache_control {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDevicePipelineCreationCacheControlFeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDevicePipelineCreationCacheControlFeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceVulkan11Features {
    type Type = PhysicalDeviceVulkan11FeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkan11FeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceVulkan11Features,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceVulkan11FeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn storage_buffer16_bit_access(mut self, storage_buffer16_bit_access: bool) -> Self {
        self.inner.storage_buffer16_bit_access = if storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer16_bit_access(mut self, uniform_and_storage_buffer16_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer16_bit_access = if uniform_and_storage_buffer16_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant16(mut self, storage_push_constant16: bool) -> Self {
        self.inner.storage_push_constant16 = if storage_push_constant16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_input_output16(mut self, storage_input_output16: bool) -> Self {
        self.inner.storage_input_output16 = if storage_input_output16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview(mut self, multiview: bool) -> Self {
        self.inner.multiview = if multiview { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_geometry_shader(mut self, multiview_geometry_shader: bool) -> Self {
        self.inner.multiview_geometry_shader = if multiview_geometry_shader { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn multiview_tessellation_shader(mut self, multiview_tessellation_shader: bool) -> Self {
        self.inner.multiview_tessellation_shader = if multiview_tessellation_shader {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers_storage_buffer(mut self, variable_pointers_storage_buffer: bool) -> Self {
        self.inner.variable_pointers_storage_buffer = if variable_pointers_storage_buffer {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn variable_pointers(mut self, variable_pointers: bool) -> Self {
        self.inner.variable_pointers = if variable_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn protected_memory(mut self, protected_memory: bool) -> Self {
        self.inner.protected_memory = if protected_memory { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sampler_ycbcr_conversion(mut self, sampler_ycbcr_conversion: bool) -> Self {
        self.inner.sampler_ycbcr_conversion = if sampler_ycbcr_conversion { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_draw_parameters(mut self, shader_draw_parameters: bool) -> Self {
        self.inner.shader_draw_parameters = if shader_draw_parameters { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceVulkan11FeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceVulkan11Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceVulkan12Features {
    type Type = PhysicalDeviceVulkan12FeaturesBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceVulkan12FeaturesBuilder<'a> {
    inner: vk::PhysicalDeviceVulkan12Features,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceVulkan12FeaturesBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn sampler_mirror_clamp_to_edge(mut self, sampler_mirror_clamp_to_edge: bool) -> Self {
        self.inner.sampler_mirror_clamp_to_edge = if sampler_mirror_clamp_to_edge {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn draw_indirect_count(mut self, draw_indirect_count: bool) -> Self {
        self.inner.draw_indirect_count = if draw_indirect_count { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn storage_buffer8_bit_access(mut self, storage_buffer8_bit_access: bool) -> Self {
        self.inner.storage_buffer8_bit_access = if storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn uniform_and_storage_buffer8_bit_access(mut self, uniform_and_storage_buffer8_bit_access: bool) -> Self {
        self.inner.uniform_and_storage_buffer8_bit_access = if uniform_and_storage_buffer8_bit_access {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn storage_push_constant8(mut self, storage_push_constant8: bool) -> Self {
        self.inner.storage_push_constant8 = if storage_push_constant8 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_buffer_int64_atomics(mut self, shader_buffer_int64_atomics: bool) -> Self {
        self.inner.shader_buffer_int64_atomics = if shader_buffer_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_shared_int64_atomics(mut self, shader_shared_int64_atomics: bool) -> Self {
        self.inner.shader_shared_int64_atomics = if shader_shared_int64_atomics {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_float16(mut self, shader_float16: bool) -> Self {
        self.inner.shader_float16 = if shader_float16 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_int8(mut self, shader_int8: bool) -> Self {
        self.inner.shader_int8 = if shader_int8 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn descriptor_indexing(mut self, descriptor_indexing: bool) -> Self {
        self.inner.descriptor_indexing = if descriptor_indexing { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn shader_input_attachment_array_dynamic_indexing(
        mut self,
        shader_input_attachment_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_dynamic_indexing = if shader_input_attachment_array_dynamic_indexing {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_uniform_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_uniform_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_dynamic_indexing =
            if shader_uniform_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_dynamic_indexing(
        mut self,
        shader_storage_texel_buffer_array_dynamic_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_dynamic_indexing =
            if shader_storage_texel_buffer_array_dynamic_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_buffer_array_non_uniform_indexing =
            if shader_uniform_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_sampled_image_array_non_uniform_indexing(
        mut self,
        shader_sampled_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_sampled_image_array_non_uniform_indexing = if shader_sampled_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_storage_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_buffer_array_non_uniform_indexing =
            if shader_storage_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_image_array_non_uniform_indexing(
        mut self,
        shader_storage_image_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_image_array_non_uniform_indexing = if shader_storage_image_array_non_uniform_indexing
        {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_input_attachment_array_non_uniform_indexing(
        mut self,
        shader_input_attachment_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_input_attachment_array_non_uniform_indexing =
            if shader_input_attachment_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_uniform_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_uniform_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_uniform_texel_buffer_array_non_uniform_indexing =
            if shader_uniform_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_storage_texel_buffer_array_non_uniform_indexing(
        mut self,
        shader_storage_texel_buffer_array_non_uniform_indexing: bool,
    ) -> Self {
        self.inner.shader_storage_texel_buffer_array_non_uniform_indexing =
            if shader_storage_texel_buffer_array_non_uniform_indexing {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_buffer_update_after_bind =
            if descriptor_binding_uniform_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_sampled_image_update_after_bind(
        mut self,
        descriptor_binding_sampled_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_sampled_image_update_after_bind =
            if descriptor_binding_sampled_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_image_update_after_bind(
        mut self,
        descriptor_binding_storage_image_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_image_update_after_bind =
            if descriptor_binding_storage_image_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_buffer_update_after_bind =
            if descriptor_binding_storage_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_uniform_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_uniform_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_uniform_texel_buffer_update_after_bind =
            if descriptor_binding_uniform_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_storage_texel_buffer_update_after_bind(
        mut self,
        descriptor_binding_storage_texel_buffer_update_after_bind: bool,
    ) -> Self {
        self.inner.descriptor_binding_storage_texel_buffer_update_after_bind =
            if descriptor_binding_storage_texel_buffer_update_after_bind {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn descriptor_binding_update_unused_while_pending(
        mut self,
        descriptor_binding_update_unused_while_pending: bool,
    ) -> Self {
        self.inner.descriptor_binding_update_unused_while_pending = if descriptor_binding_update_unused_while_pending {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_partially_bound(mut self, descriptor_binding_partially_bound: bool) -> Self {
        self.inner.descriptor_binding_partially_bound = if descriptor_binding_partially_bound {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn descriptor_binding_variable_descriptor_count(
        mut self,
        descriptor_binding_variable_descriptor_count: bool,
    ) -> Self {
        self.inner.descriptor_binding_variable_descriptor_count = if descriptor_binding_variable_descriptor_count {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn runtime_descriptor_array(mut self, runtime_descriptor_array: bool) -> Self {
        self.inner.runtime_descriptor_array = if runtime_descriptor_array { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn sampler_filter_minmax(mut self, sampler_filter_minmax: bool) -> Self {
        self.inner.sampler_filter_minmax = if sampler_filter_minmax { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn scalar_block_layout(mut self, scalar_block_layout: bool) -> Self {
        self.inner.scalar_block_layout = if scalar_block_layout { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn imageless_framebuffer(mut self, imageless_framebuffer: bool) -> Self {
        self.inner.imageless_framebuffer = if imageless_framebuffer { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn uniform_buffer_standard_layout(mut self, uniform_buffer_standard_layout: bool) -> Self {
        self.inner.uniform_buffer_standard_layout = if uniform_buffer_standard_layout {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_subgroup_extended_types(mut self, shader_subgroup_extended_types: bool) -> Self {
        self.inner.shader_subgroup_extended_types = if shader_subgroup_extended_types {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn separate_depth_stencil_layouts(mut self, separate_depth_stencil_layouts: bool) -> Self {
        self.inner.separate_depth_stencil_layouts = if separate_depth_stencil_layouts {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn host_query_reset(mut self, host_query_reset: bool) -> Self {
        self.inner.host_query_reset = if host_query_reset { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn timeline_semaphore(mut self, timeline_semaphore: bool) -> Self {
        self.inner.timeline_semaphore = if timeline_semaphore { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address(mut self, buffer_device_address: bool) -> Self {
        self.inner.buffer_device_address = if buffer_device_address { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn buffer_device_address_capture_replay(mut self, buffer_device_address_capture_replay: bool) -> Self {
        self.inner.buffer_device_address_capture_replay = if buffer_device_address_capture_replay {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn buffer_device_address_multi_device(mut self, buffer_device_address_multi_device: bool) -> Self {
        self.inner.buffer_device_address_multi_device = if buffer_device_address_multi_device {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model(mut self, vulkan_memory_model: bool) -> Self {
        self.inner.vulkan_memory_model = if vulkan_memory_model { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn vulkan_memory_model_device_scope(mut self, vulkan_memory_model_device_scope: bool) -> Self {
        self.inner.vulkan_memory_model_device_scope = if vulkan_memory_model_device_scope {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn vulkan_memory_model_availability_visibility_chains(
        mut self,
        vulkan_memory_model_availability_visibility_chains: bool,
    ) -> Self {
        self.inner.vulkan_memory_model_availability_visibility_chains =
            if vulkan_memory_model_availability_visibility_chains {
                vk::TRUE
            } else {
                vk::FALSE
            };
        self
    }
    pub fn shader_output_viewport_index(mut self, shader_output_viewport_index: bool) -> Self {
        self.inner.shader_output_viewport_index = if shader_output_viewport_index {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn shader_output_layer(mut self, shader_output_layer: bool) -> Self {
        self.inner.shader_output_layer = if shader_output_layer { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn subgroup_broadcast_dynamic_id(mut self, subgroup_broadcast_dynamic_id: bool) -> Self {
        self.inner.subgroup_broadcast_dynamic_id = if subgroup_broadcast_dynamic_id {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
}
impl<'a> Deref for PhysicalDeviceVulkan12FeaturesBuilder<'a> {
    type Target = vk::PhysicalDeviceVulkan12Features;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineCompilerControlCreateInfoAMD {
    type Type = PipelineCompilerControlCreateInfoAMDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    inner: vk::PipelineCompilerControlCreateInfoAMD,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compiler_control_flags(mut self, compiler_control_flags: vk::PipelineCompilerControlFlagsAMD) -> Self {
        self.inner.compiler_control_flags = compiler_control_flags;
        self
    }
}
impl<'a> Deref for PipelineCompilerControlCreateInfoAMDBuilder<'a> {
    type Target = vk::PipelineCompilerControlCreateInfoAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceCoherentMemoryFeaturesAMD {
    type Type = PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    inner: vk::PhysicalDeviceCoherentMemoryFeaturesAMD,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn device_coherent_memory(mut self, device_coherent_memory: bool) -> Self {
        self.inner.device_coherent_memory = if device_coherent_memory { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceCoherentMemoryFeaturesAMDBuilder<'a> {
    type Target = vk::PhysicalDeviceCoherentMemoryFeaturesAMD;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureGeometryTrianglesDataKHR {
    type Type = AccelerationStructureGeometryTrianglesDataKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    inner: vk::AccelerationStructureGeometryTrianglesDataKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn vertex_format(mut self, vertex_format: vk::Format) -> Self {
        self.inner.vertex_format = vertex_format;
        self
    }
    pub fn vertex_data(mut self, vertex_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.vertex_data = vertex_data;
        self
    }
    pub fn vertex_stride(mut self, vertex_stride: vk::DeviceSize) -> Self {
        self.inner.vertex_stride = vertex_stride;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn index_data(mut self, index_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.index_data = index_data;
        self
    }
    pub fn transform_data(mut self, transform_data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.transform_data = transform_data;
        self
    }
}
impl<'a> Deref for AccelerationStructureGeometryTrianglesDataKHRBuilder<'a> {
    type Target = vk::AccelerationStructureGeometryTrianglesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureGeometryAabbsDataKHR {
    type Type = AccelerationStructureGeometryAabbsDataKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    inner: vk::AccelerationStructureGeometryAabbsDataKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn data(mut self, data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.data = data;
        self
    }
    pub fn stride(mut self, stride: vk::DeviceSize) -> Self {
        self.inner.stride = stride;
        self
    }
}
impl<'a> Deref for AccelerationStructureGeometryAabbsDataKHRBuilder<'a> {
    type Target = vk::AccelerationStructureGeometryAabbsDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureGeometryInstancesDataKHR {
    type Type = AccelerationStructureGeometryInstancesDataKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    inner: vk::AccelerationStructureGeometryInstancesDataKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn array_of_pointers(mut self, array_of_pointers: bool) -> Self {
        self.inner.array_of_pointers = if array_of_pointers { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn data(mut self, data: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.data = data;
        self
    }
}
impl<'a> Deref for AccelerationStructureGeometryInstancesDataKHRBuilder<'a> {
    type Target = vk::AccelerationStructureGeometryInstancesDataKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureGeometryKHR {
    type Type = AccelerationStructureGeometryKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureGeometryKHRBuilder<'a> {
    inner: vk::AccelerationStructureGeometryKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureGeometryKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn geometry_type(mut self, geometry_type: vk::GeometryTypeKHR) -> Self {
        self.inner.geometry_type = geometry_type;
        self
    }
    pub fn geometry(mut self, geometry: vk::AccelerationStructureGeometryDataKHR) -> Self {
        self.inner.geometry = geometry;
        self
    }
    pub fn flags(mut self, flags: vk::GeometryFlagsKHR) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for AccelerationStructureGeometryKHRBuilder<'a> {
    type Target = vk::AccelerationStructureGeometryKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureBuildGeometryInfoKHR {
    type Type = AccelerationStructureBuildGeometryInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureBuildGeometryInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureBuildGeometryInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
    pub fn update(mut self, update: bool) -> Self {
        self.inner.update = if update { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn src_acceleration_structure(
        mut self,
        src_acceleration_structure: Option<vk::AccelerationStructureKHR>,
    ) -> Self {
        self.inner.src_acceleration_structure = src_acceleration_structure;
        self
    }
    pub fn dst_acceleration_structure(mut self, dst_acceleration_structure: vk::AccelerationStructureKHR) -> Self {
        self.inner.dst_acceleration_structure = Some(dst_acceleration_structure);
        self
    }
    pub fn geometry_array_of_pointers(mut self, geometry_array_of_pointers: bool) -> Self {
        self.inner.geometry_array_of_pointers = if geometry_array_of_pointers {
            vk::TRUE
        } else {
            vk::FALSE
        };
        self
    }
    pub fn geometry_count(mut self, geometry_count: u32) -> Self {
        self.inner.geometry_count = geometry_count;
        self
    }
    pub fn pp_geometries(mut self, pp_geometries: *const *const vk::AccelerationStructureGeometryKHR) -> Self {
        self.inner.pp_geometries = pp_geometries;
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
impl<'a> Builder<'a> for vk::AccelerationStructureCreateGeometryTypeInfoKHR {
    type Type = AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureCreateGeometryTypeInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn geometry_type(mut self, geometry_type: vk::GeometryTypeKHR) -> Self {
        self.inner.geometry_type = geometry_type;
        self
    }
    pub fn max_primitive_count(mut self, max_primitive_count: u32) -> Self {
        self.inner.max_primitive_count = max_primitive_count;
        self
    }
    pub fn index_type(mut self, index_type: vk::IndexType) -> Self {
        self.inner.index_type = index_type;
        self
    }
    pub fn max_vertex_count(mut self, max_vertex_count: u32) -> Self {
        self.inner.max_vertex_count = max_vertex_count;
        self
    }
    pub fn vertex_format(mut self, vertex_format: vk::Format) -> Self {
        self.inner.vertex_format = vertex_format;
        self
    }
    pub fn allows_transforms(mut self, allows_transforms: bool) -> Self {
        self.inner.allows_transforms = if allows_transforms { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for AccelerationStructureCreateGeometryTypeInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureCreateGeometryTypeInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureCreateInfoKHR {
    type Type = AccelerationStructureCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureCreateInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn compacted_size(mut self, compacted_size: vk::DeviceSize) -> Self {
        self.inner.compacted_size = compacted_size;
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
    pub fn p_geometry_infos(
        mut self,
        p_geometry_infos: &'a [vk::AccelerationStructureCreateGeometryTypeInfoKHR],
    ) -> Self {
        self.inner.max_geometry_count = p_geometry_infos.len() as u32;
        self.inner.p_geometry_infos = p_geometry_infos.as_ptr();
        self
    }
    pub fn device_address(mut self, device_address: vk::DeviceAddress) -> Self {
        self.inner.device_address = device_address;
        self
    }
}
impl<'a> Deref for AccelerationStructureCreateInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureDeviceAddressInfoKHR {
    type Type = AccelerationStructureDeviceAddressInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    inner: vk::AccelerationStructureDeviceAddressInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn acceleration_structure(mut self, acceleration_structure: vk::AccelerationStructureKHR) -> Self {
        self.inner.acceleration_structure = Some(acceleration_structure);
        self
    }
}
impl<'a> Deref for AccelerationStructureDeviceAddressInfoKHRBuilder<'a> {
    type Target = vk::AccelerationStructureDeviceAddressInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::AccelerationStructureVersionKHR {
    type Type = AccelerationStructureVersionKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct AccelerationStructureVersionKHRBuilder<'a> {
    inner: vk::AccelerationStructureVersionKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> AccelerationStructureVersionKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn version_data(mut self, version_data: *const u8) -> Self {
        self.inner.version_data = version_data;
        self
    }
}
impl<'a> Deref for AccelerationStructureVersionKHRBuilder<'a> {
    type Target = vk::AccelerationStructureVersionKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyAccelerationStructureInfoKHR {
    type Type = CopyAccelerationStructureInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyAccelerationStructureInfoKHRBuilder<'a> {
    inner: vk::CopyAccelerationStructureInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CopyAccelerationStructureInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::AccelerationStructureKHR) -> Self {
        self.inner.src = Some(src);
        self
    }
    pub fn dst(mut self, dst: vk::AccelerationStructureKHR) -> Self {
        self.inner.dst = Some(dst);
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl<'a> Deref for CopyAccelerationStructureInfoKHRBuilder<'a> {
    type Target = vk::CopyAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyAccelerationStructureToMemoryInfoKHR {
    type Type = CopyAccelerationStructureToMemoryInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    inner: vk::CopyAccelerationStructureToMemoryInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::AccelerationStructureKHR) -> Self {
        self.inner.src = Some(src);
        self
    }
    pub fn dst(mut self, dst: vk::DeviceOrHostAddressKHR) -> Self {
        self.inner.dst = dst;
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl<'a> Deref for CopyAccelerationStructureToMemoryInfoKHRBuilder<'a> {
    type Target = vk::CopyAccelerationStructureToMemoryInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CopyMemoryToAccelerationStructureInfoKHR {
    type Type = CopyMemoryToAccelerationStructureInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    inner: vk::CopyMemoryToAccelerationStructureInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn src(mut self, src: vk::DeviceOrHostAddressConstKHR) -> Self {
        self.inner.src = src;
        self
    }
    pub fn dst(mut self, dst: vk::AccelerationStructureKHR) -> Self {
        self.inner.dst = Some(dst);
        self
    }
    pub fn mode(mut self, mode: vk::CopyAccelerationStructureModeKHR) -> Self {
        self.inner.mode = mode;
        self
    }
}
impl<'a> Deref for CopyMemoryToAccelerationStructureInfoKHRBuilder<'a> {
    type Target = vk::CopyMemoryToAccelerationStructureInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::RayTracingPipelineInterfaceCreateInfoKHR {
    type Type = RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    inner: vk::RayTracingPipelineInterfaceCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn max_payload_size(mut self, max_payload_size: u32) -> Self {
        self.inner.max_payload_size = max_payload_size;
        self
    }
    pub fn max_attribute_size(mut self, max_attribute_size: u32) -> Self {
        self.inner.max_attribute_size = max_attribute_size;
        self
    }
    pub fn max_callable_size(mut self, max_callable_size: u32) -> Self {
        self.inner.max_callable_size = max_callable_size;
        self
    }
}
impl<'a> Deref for RayTracingPipelineInterfaceCreateInfoKHRBuilder<'a> {
    type Target = vk::RayTracingPipelineInterfaceCreateInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeferredOperationInfoKHR {
    type Type = DeferredOperationInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeferredOperationInfoKHRBuilder<'a> {
    inner: vk::DeferredOperationInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeferredOperationInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn operation_handle(mut self, operation_handle: vk::DeferredOperationKHR) -> Self {
        self.inner.operation_handle = Some(operation_handle);
        self
    }
}
impl<'a> Deref for DeferredOperationInfoKHRBuilder<'a> {
    type Target = vk::DeferredOperationInfoKHR;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PipelineLibraryCreateInfoKHR {
    type Type = PipelineLibraryCreateInfoKHRBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PipelineLibraryCreateInfoKHRBuilder<'a> {
    inner: vk::PipelineLibraryCreateInfoKHR,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PipelineLibraryCreateInfoKHRBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
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
impl<'a> Builder<'a> for vk::RenderPassTransformBeginInfoQCOM {
    type Type = RenderPassTransformBeginInfoQCOMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct RenderPassTransformBeginInfoQCOMBuilder<'a> {
    inner: vk::RenderPassTransformBeginInfoQCOM,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> RenderPassTransformBeginInfoQCOMBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
}
impl<'a> Deref for RenderPassTransformBeginInfoQCOMBuilder<'a> {
    type Target = vk::RenderPassTransformBeginInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::CommandBufferInheritanceRenderPassTransformInfoQCOM {
    type Type = CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    inner: vk::CommandBufferInheritanceRenderPassTransformInfoQCOM,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn transform(mut self, transform: vk::SurfaceTransformFlagsKHR) -> Self {
        self.inner.transform = transform;
        self
    }
    pub fn render_area(mut self, render_area: vk::Rect2D) -> Self {
        self.inner.render_area = render_area;
        self
    }
}
impl<'a> Deref for CommandBufferInheritanceRenderPassTransformInfoQCOMBuilder<'a> {
    type Target = vk::CommandBufferInheritanceRenderPassTransformInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceDiagnosticsConfigFeaturesNV {
    type Type = PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    inner: vk::PhysicalDeviceDiagnosticsConfigFeaturesNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn diagnostics_config(mut self, diagnostics_config: bool) -> Self {
        self.inner.diagnostics_config = if diagnostics_config { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceDiagnosticsConfigFeaturesNVBuilder<'a> {
    type Target = vk::PhysicalDeviceDiagnosticsConfigFeaturesNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::DeviceDiagnosticsConfigCreateInfoNV {
    type Type = DeviceDiagnosticsConfigCreateInfoNVBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    inner: vk::DeviceDiagnosticsConfigCreateInfoNV,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *const c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn flags(mut self, flags: vk::DeviceDiagnosticsConfigFlagsNV) -> Self {
        self.inner.flags = flags;
        self
    }
}
impl<'a> Deref for DeviceDiagnosticsConfigCreateInfoNVBuilder<'a> {
    type Target = vk::DeviceDiagnosticsConfigCreateInfoNV;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<'a> Builder<'a> for vk::PhysicalDeviceRobustness2FeaturesEXT {
    type Type = PhysicalDeviceRobustness2FeaturesEXTBuilder<'a>;
    fn builder() -> Self::Type {
        Default::default()
    }
}
#[derive(Default)]
pub struct PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    inner: vk::PhysicalDeviceRobustness2FeaturesEXT,
    phantom: PhantomData<&'a c_void>,
}
impl<'a> PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    pub fn s_type(mut self, s_type: vk::StructureType) -> Self {
        self.inner.s_type = s_type;
        self
    }
    pub fn p_next(mut self, p_next: *mut c_void) -> Self {
        self.inner.p_next = p_next;
        self
    }
    pub fn robust_buffer_access2(mut self, robust_buffer_access2: bool) -> Self {
        self.inner.robust_buffer_access2 = if robust_buffer_access2 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn robust_image_access2(mut self, robust_image_access2: bool) -> Self {
        self.inner.robust_image_access2 = if robust_image_access2 { vk::TRUE } else { vk::FALSE };
        self
    }
    pub fn null_descriptor(mut self, null_descriptor: bool) -> Self {
        self.inner.null_descriptor = if null_descriptor { vk::TRUE } else { vk::FALSE };
        self
    }
}
impl<'a> Deref for PhysicalDeviceRobustness2FeaturesEXTBuilder<'a> {
    type Target = vk::PhysicalDeviceRobustness2FeaturesEXT;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
