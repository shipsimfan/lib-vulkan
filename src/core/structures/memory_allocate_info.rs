use crate::{VkDeviceSize, VkStructureType, util::NextChain};
use std::{ffi::c_void, ptr::null};

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    VK_NULL_HANDLE, VK_VERSION_1_0, VkAllocateMemory, VkBuffer, VkBufferCreateInfo, VkFormat,
    VkMemoryPropertyFlag, VkMemoryRequirements, VkPhysicalDeviceLimits,
    VkPhysicalDeviceMemoryProperties, VkResult,
};
#[allow(unused_imports)]
use std::ptr::null_mut;

/// Structure containing parameters of a memory allocation
///
/// # Description
/// The internal data of an allocated device memory object must include a reference to
/// implementation-specific resources, referred to as the memory object’s payload. Applications can
/// also import and export that internal data to and from device memory objects to share data
/// between Vulkan instances and other compatible APIs. A [`VkMemoryAllocateInfo`] structure
/// defines a memory import operation if its `next` chain includes one of the following structures:
///  - [`VkImportMemoryWin32HandleInfoKhr`] with a non-zero `handle_type` value
///  - [`VkImportMemoryFdInfoKhr`] with a non-zero `handle_type` value
///  - [`VkImportMemoryHostPointerInfoExt`] with a non-zero `handle_type` value
///  - [`VkImportAndroidHardwareBufferInfoAndroid`] with a non-[`null_mut`] `buffer` value
///  - [`VkImportMemoryZirconHandleInfoFuchsia`] with a non-zero `handle_type` value
///  - [`VkImportMemoryBufferCollectionFuchsia`]
///  - [`VkImportScreenBufferInfoQnx`] with a non-[`null_mut`] `buffer` value
///  - [`VkImportMemoryMetalHandleInfoExt`] with a non-zero `handle_type` value
///
/// If the parameters define an import operation and the external handle type is
/// [`VkExternalMemoryHandleTypeFlag::D3D11Texture`],
/// [`VkExternalMemoryHandleTypeFlag::D3D11TextureKmt`], or
/// [`VkExternalMemoryHandleTypeFlag::D3D12Resource`], `allocation_size` is ignored. The
/// implementation must query the size of these allocations from the OS.
///
/// If the parameters define an import operation and the external handle type is
/// [`VkExternalMemoryHandleTypeFlag::MtlTextureExt`], `allocation_size` is ignored. The
/// implementation must query the size of these allocations from the OS.
///
/// Whether device memory objects constructed via a memory import operation hold a reference to
/// their payload depends on the properties of the handle type used to perform the import, as
/// defined below for each valid handle type. Importing memory must not modify the content of the
/// memory. Implementations must ensure that importing memory does not enable the importing Vulkan
/// instance to access any memory or resources in other Vulkan instances other than that
/// corresponding to the memory object imported. Implementations must also ensure accessing
/// imported memory which has not been initialized does not allow the importing Vulkan instance to
/// obtain data from the exporting Vulkan instance or vice-versa.
///
/// Importing memory must not increase overall heap usage within a system. However, it must affect
/// the following per-process values:
///  - [`VkPhysicalDeviceLimits::max_memory_allocation_count`]
///  - [`VkPhysicalDeviceMemoryBudgetPropertiesExt::heap_usage`]
///
/// When performing a memory import operation, it is the responsibility of the application to
/// ensure the external handles and their associated payloads meet all valid usage requirements.
/// However, implementations must perform sufficient validation of external handles and payloads to
/// ensure that the operation results in a valid memory object which will not cause program
/// termination, device loss, queue stalls, or corruption of other resources when used as allowed
/// according to its allocation parameters. If the external handle provided does not meet these
/// requirements, the implementation must fail the memory import operation with the error code
/// [`VkResult::VkErrorInvalidExternalHandle`]. If the parameters define an export operation and
/// the external handle type is [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`],
/// implementations should not strictly follow `memory_type_index`. Instead, they should modify the
/// allocation internally to use the required memory type for the application’s given usage. This
/// is because for an export operation, there is currently no way for the application to know the
/// memory type index before allocating.
///
/// # Valid Usage
///  - The parameters must not define more than one import operation
///  - If the parameters define an import operation and the external handle specified was created
///    by the Vulkan API, the device mask specified by [`VkMemoryAllocateFlagsInfo`] must match the
///    mask specified when the payload being imported was allocated
///  - If the parameters define an import operation and the external handle specified was created
///    by the Vulkan API, the list of physical devices that comprise the logical device passed to
///    [`VkAllocateMemory`] must match the list of physical devices that comprise the logical
///    device on which the payload was originally allocated
///  - If the parameters define an import operation and the external handle type is
///    [`VkExternalMemoryHandleTypeFlag::ZirconVmoFuchsia`], the value of `memory_type_index` must
///    be an index identifying a memory type from the `memory_type_bits` field of the
///    [`VkMemoryZirconHandlePropertiesFuchsia`] structure populated by a call to
///    [`VkGetMemoryZirconHandlePropertiesFuchsia`]
///
/// Provided by [`VK_VERSION_1_0`]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VkMemoryAllocateInfo {
    /// `r#type` is a [`VkStructureType`] value identifying this structure.
    ///
    /// # Valid Usage (Implicit)
    ///  - `r#type` must be [`VkStructureType::MemoryAllocateInfo`]
    pub r#type: VkStructureType,

    /// `next` is [`null`] or a pointer to a structure extending this structure.
    ///
    /// # Valid Usage
    ///  - If the parameters define an import operation from an [`VkBufferCollectionFuchsia`], and
    ///    [`VkMemoryDedicatedAllocateInfo::buffer`] is present and non-[`null_mut`],
    ///    [`VkImportMemoryBufferCollectionFuchsia::collection`] and
    ///    [`VkImportMemoryBufferCollectionFuchsia::index`] must match
    ///    [`VkBufferCollectionBufferCreateInfoFuchsia::collection`] and
    ///    [`VkBufferCollectionBufferCreateInfoFuchsia::index`], respectively, of the
    ///    [`VkBufferCollectionBufferCreateInfoFuchsia`] structure used to create the
    ///    [`VkMemoryDedicatedAllocateInfo::buffer`]
    ///  - If the parameters define an import operation from an [`VkBufferCollectionFuchsia`], and
    ///    [`VkMemoryDedicatedAllocateInfo::image`] is present and non-[`null_mut`],
    ///    [`VkImportMemoryBufferCollectionFuchsia::collection`] and
    ///    [`VkImportMemoryBufferCollectionFuchsia::index`] must match
    ///    [`VkBufferCollectionImageCreateInfoFuchsia::collection`] and
    ///    [`VkBufferCollectionImageCreateInfoFuchsia::index`], respectively, of the
    ///    [`VkBufferCollectionImageCreateInfoFuchsia`]  structure used to create the
    ///    [`VkMemoryDedicatedAllocateInfo::image`]
    ///  - If the parameters define an import operation from an [`VkBufferCollectionFuchsia`], the
    ///    `next` chain must include a [`VkMemoryDedicatedAllocateInfo`] structure with either its
    ///    `image` or `buffer` field set to a value other than [`VK_NULL_HANDLE`]
    ///  - If the parameters define an import operation from an [`VkBufferCollectionFuchsia`] and
    ///    [`VkMemoryDedicatedAllocateInfo::image`] is not [`VK_NULL_HANDLE`], the `image` must be
    ///    created with a [`VkBufferCollectionImageCreateInfoFuchsia`] structure chained to its
    ///    [`VkImageCreateInfo::next`] pointer
    ///  - If the parameters define an import operation from an [`VkBufferCollectionFuchsia`] and
    ///    [`VkMemoryDedicatedAllocateInfo::buffer`] is not [`VK_NULL_HANDLE`], the `buffer` must
    ///    be created with a [`VkBufferCollectionBufferCreateInfoFuchsia`] structure chained to its
    ///    [`VkBufferCreateInfo::next`] pointer
    ///  - If the `next` chain includes a [`VkExportMemoryAllocateInfo`] structure, and any of the
    ///    handle types specified in [`VkExportMemoryAllocateInfo::handle_types`] require a
    ///    dedicated allocation, as reported by [`VkGetPhysicalDeviceImageFormatProperties2`] in
    ///    `VkExternalImageFormatProperties::external_memory_properties.external_memory_features`,
    ///    or by [`VkGetPhysicalDeviceExternalBufferProperties`] in
    ///    `VkExternalBufferProperties::external_memory_properties.external_memory_features`, the
    ///    `next` chain must include a [`VkMemoryDedicatedAllocateInfo`] or
    ///    [`VkDedicatedAllocationMemoryAllocateInfoNv`] structure with either its `image` or
    ///    `buffer` member set to a value other than [`VK_NULL_HANDLE`]
    ///  - If the `next` chain includes a [`VkExportMemoryAllocateInfo`] structure, and any of the
    ///    handle types specified in [`VkExportMemoryAllocateInfo::handle_types`] require a
    ///    dedicated allocation, as reported by [`VkGetPhysicalDeviceExternalTensorPropertiesArm`]
    ///    in `VkExternalTensorPropertiesArm::external_memory_properties.external_memory_features`,
    ///    the `next` chain must include a [`VkMemoryDedicatedAllocateInfoTensorArm`] structure
    ///    with its `tensor` member set to a value other than [`VK_NULL_HANDLE`]
    ///  - If the `next` chain includes a [`VkExportMemoryAllocateInfo`] structure, it must not
    ///    include a [`VkExportMemoryAllocateInfoNv`] or [`VkExportMemoryWin32HandleInfoNv`]
    ///    structure
    ///  - If the `next` chain includes a [`VkImportMemoryWin32HandleInfoKhr`] structure, it must
    ///    not include a [`VkImportMemoryWin32HandleInfoNv`] structure
    ///  - If the parameters define an import operation and the external handle is a host pointer,
    ///    the `next` chain must not include a [`VkDedicatedAllocationMemoryAllocateInfoNv`]
    ///    structure with either its `image` or `buffer` field set to a value other than
    ///    [`VK_NULL_HANDLE`]
    ///  - If the parameters define an import operation and the external handle is a host pointer,
    ///    the `next` chain must not include a [`VkMemoryDedicatedAllocateInfo`] structure with
    ///    either its `image` or `buffer` field set to a value other than [`VK_NULL_HANDLE`]
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`], and the `next` chain
    ///    does not include a [`VkMemoryDedicatedAllocateInfo`] structure or
    ///    [`VkMemoryDedicatedAllocateInfo::image`] is [`VK_NULL_HANDLE`], the Android hardware
    ///    buffer must have a `AHardwareBuffer_Desc::format` of `AHARDWAREBUFFER_FORMAT_BLOB` and a
    ///    `AHardwareBuffer_Desc::usage` that includes `AHARDWAREBUFFER_USAGE_GPU_DATA_BUFFER`
    ///  - If the parameters do not define an import operation, and the `next` chain includes a
    ///    [`VkExportMemoryAllocateInfo`] structure with
    ///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`] included in its
    ///    `handle_types` member, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`]
    ///    structure with `image` not equal to [`VK_NULL_HANDLE`], then `allocation_size` must be 0
    ///  - If the parameters define an export operation, the handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`], and the `next` does
    ///    not include a [`VkMemoryDedicatedAllocateInfo`] structure, `allocation_size` must be
    ///    greater than 0
    ///  - If the parameters define an export operation, the handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`], and the `next` chain
    ///    includes a [`VkMemoryDedicatedAllocateInfo`] structure with `buffer` set to a valid
    ///    [`VkBuffer`] object, `allocation_size` must be greater than 0
    ///  - If the parameters define an import operation, the external handle is an Android hardware
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] with `image`
    ///    that is not [`VK_NULL_HANDLE`], the Android hardware buffer’s `AHardwareBuffer::usage`
    ///    must include at least one of `AHARDWAREBUFFER_USAGE_GPU_FRAMEBUFFER`,
    ///    `AHARDWAREBUFFER_USAGE_GPU_SAMPLED_IMAGE` or `AHARDWAREBUFFER_USAGE_GPU_DATA_BUFFER`
    ///  - If the parameters define an import operation, the external handle is an Android hardware
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] with `image`
    ///    that is not [`VK_NULL_HANDLE`], the format of image must be [`VkFormat::Undefined`] or
    ///    the format returned by [`VkGetAndroidHardwareBufferPropertiesAndroid`] in
    ///    [`VkAndroidHardwareBufferFormatPropertiesAndroid::format`] for the Android hardware
    ///    buffer
    ///  - If the parameters define an import operation, the external handle is an Android hardware
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] structure with
    ///    `image` that is not [`VK_NULL_HANDLE`], the width, height, and array layer dimensions of
    ///    image and the Android hardware buffer’s `AHardwareBuffer_Desc` must be identical
    ///  - If the parameters define an import operation, the external handle is an Android hardware
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] structure with
    ///    `image` that is not [`VK_NULL_HANDLE`], and the Android hardware buffer’s
    ///    `AHardwareBuffer::usage` includes `AHARDWAREBUFFER_USAGE_GPU_MIPMAP_COMPLETE`, the image
    ///    must have a complete mipmap chain
    ///  - If the parameters define an import operation, the external handle is an Android hardware
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] structure with
    ///    `image` that is not [`VK_NULL_HANDLE`], and the Android hardware buffer’s
    ///    `AHardwareBuffer::usage` does not include `AHARDWAREBUFFER_USAGE_GPU_MIPMAP_COMPLETE`,
    ///    the image must have exactly one mipmap level
    ///  - If the parameters define an import operation, the external handle is an Android hardware
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] structure with
    ///    `image` that is not [`VK_NULL_HANDLE`], each bit set in the usage of image must be
    ///    listed in AHardwareBuffer Usage Equivalence, and if there is a corresponding
    ///    `AHARDWAREBUFFER_USAGE` bit listed that bit must be included in the Android hardware
    ///    buffer’s `AHardwareBuffer_Desc::usage`
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::ScreenBufferQnx`],
    ///    [`VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQnx::screen_buffer_import`] must be
    ///    enabled
    ///  - If the parameters define an import operation, the external handle is a QNX Screen
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] with `image`
    ///    that is not [`VK_NULL_HANDLE`], the QNX Screen’s buffer must be a valid QNX Screen
    ///    buffer
    ///  - If the parameters define an import operation, the external handle is an QNX Screen
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] with `image`
    ///    that is not [`VK_NULL_HANDLE`], the format of `image` must be [`VkFormat::Undefined`] or
    ///    the format returned by [`VkGetScreenBufferPropertiesQnx`] in
    ///    [`VkScreenBufferFormatPropertiesQnx::format`] for the QNX Screen buffer
    ///  - If the parameters define an import operation, the external handle is a QNX Screen
    ///    buffer, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`] structure with
    ///    `image` that is not [`VK_NULL_HANDLE`], the width, height, and array layer dimensions of
    ///    image and the QNX Screen buffer’s `_screen_buffer` must be identical
    ///  - If the parameters define an import operation and the external handle is a
    ///    [`VkExternalMemoryHandleTypeFlag::MtlTextureExt`], then `next` must include a
    ///    [`VkMemoryDedicatedAllocateInfo`] with `image` that is not [`VK_NULL_HANDLE`]
    ///  - If the parameters define an import operation, the external handle is a Metal
    ///    `MTLTexture`, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`]
    ///    structure with `image` that is not [`VK_NULL_HANDLE`], the width, height, array layer
    ///    dimensions, and mipmap levels of image and the Metal `MTLTexture`’s must be identical
    ///  - If the parameters define an import operation, the external handle is a Metal
    ///    `MTLTexture`, and the `next` chain includes a [`VkMemoryDedicatedAllocateInfo`]
    ///    structure with `image` that is not [`VK_NULL_HANDLE`], `allocation_size` must be 0
    ///  - If [`VkMemoryOpaqueCaptureAddressAllocateInfo::opaque_capture_address`] is not zero,
    ///    [`VkMemoryAllocateFlagsInfo::flags`] must include
    ///    [`VkMemoryAllocateFlag::DeviceAddressCaptureReplay`]
    ///  - If [`VkMemoryAllocateFlagsInfo::flags`] includes
    ///    [`VkMemoryAllocateFlag::DeviceAddressCaptureReplay`], the
    ///    `buffer_device_address_capture_replay` feature must be enabled
    ///  - If [`VkMemoryAllocateFlagsInfo::flags`] includes
    ///    [`VkMemoryAllocateFlag::DeviceAddress`], the `buffer_device_address` feature must be
    ///    enabled
    ///  - If the `next` chain includes a [`VkImportMemoryHostPointerInfoExt`] structure,
    ///    [`VkMemoryOpaqueCaptureAddressAllocateInfo::opaque_capture_address`] must be zero
    ///  - If the parameters define an import operation,
    ///    [`VkMemoryOpaqueCaptureAddressAllocateInfo::opaque_capture_address`] must be zero
    ///  - If the `next` chain includes a [`VkExportMetalObjectCreateInfoExt`] structure, its
    ///    `export_object_type` member must be [`VkExportMetalObjectTypeFlag::MetalBufferExt`]
    ///
    /// # Valid Usage (Implicit)
    ///  - Each `next` member of any structure (including this one) in the `next` chain must be
    ///    either [`null`] or a pointer to a valid instance of
    ///    [`VkDedicatedAllocationMemoryAllocateInfoNv`], [`VkExportMemoryAllocateInfo`],
    ///    [`VkExportMemoryAllocateInfoNv`], [`VkExportMemoryWin32HandleInfoKhr`],
    ///    [`VkExportMemoryWin32HandleInfoNv`], [`VkExportMetalObjectCreateInfoExt`],
    ///    [`VkImportAndroidHardwareBufferInfoAndroid`], [`VkImportMemoryBufferCollectionFuchsia`],
    ///    [`VkImportMemoryFdInfoKhr`], [`VkImportMemoryHostPointerInfoExt`],
    ///    [`VkImportMemoryMetalHandleInfoExt`], [`VkImportMemoryWin32HandleInfoKhr`],
    ///    [`VkImportMemoryWin32HandleInfoNv`], [`VkImportMemoryZirconHandleInfoFuchsia`],
    ///    [`VkImportMetalBufferInfoExt`], [`VkImportNativeBufferInfoOhos`],
    ///    [`VkImportScreenBufferInfoQnx`], [`VkMemoryAllocateFlagsInfo`],
    ///    [`VkMemoryDedicatedAllocateInfo`], [`VkMemoryDedicatedAllocateInfoTensorArm`],
    ///    [`VkMemoryOpaqueCaptureAddressAllocateInfo`], or [`VkMemoryPriorityAllocateInfoExt`]
    ///  - The `r#type` value of each structure in the `next` chain must be unique, with the
    ///    exception of structures of type [`VkExportMetalObjectCreateInfoExt`]
    pub next: *const c_void,

    /// `allocation_size` is the size of the allocation in bytes.
    ///
    /// # Valid Usage
    ///  - If the parameters do not define an import or export operation, `allocation_size` must be
    ///    greater than 0
    ///  - If the parameters define an export operation and the handle type is not
    ///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`], `allocation_size` must
    ///    be greater than 0
    ///  - If the parameters define an import operation from an [`VkBufferCollectionFuchsia`],
    ///    `allocation_size` must match [`VkMemoryRequirements::size`] value retrieved by
    ///    [`VkGetImageMemoryRequirements`] or [`VkGetBufferMemoryRequirements`] for image-based or
    ///    buffer-based collections respectively
    ///  - If the parameters define an import operation, the external handle specified was created
    ///    by the Vulkan API, and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::OpaqueFd`], then the values of `allocation_size` and
    ///    `memory_type_index` must match those specified when the payload being imported was
    ///    created
    ///  - If the parameters define an import operation, the external handle was created by the
    ///    Vulkan API, and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::OpaqueWin32`] or
    ///    [`VkExternalMemoryHandleTypeFlag::OpaqueWin32Kmt`], then the values of `allocation_size`
    ///    and `memory_type_index` must match those specified when the payload being imported was
    ///    created
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::D3D12Heap`], `allocation_size` must match the size
    ///    specified when creating the Direct3D 12 heap from which the payload was extracted
    ///  - If the parameters define an import operation and the external handle is a host pointer,
    ///    `allocation_size` must be an integer multiple of
    ///    [`VkPhysicalDeviceExternalMemoryHostPropertiesExt::min_imported_host_pointer_alignment`]
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`], `allocation_size` must
    ///    be the size returned by [`VkGetAndroidHardwareBufferPropertiesAndroid`] for the Android
    ///    hardware buffer
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::ScreenBufferQnx`], `allocation_size` must be the size
    ///    returned by [`VkGetScreenBufferPropertiesQnx`] for the QNX Screen buffer
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::ZirconVmoFuchsia`], the value of `allocation_size`
    ///    must be greater than 0
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::ZirconVmoFuchsia`], the value of `allocation_size`
    ///    must be less than or equal to the size of the VMO as determined by
    ///    `zx_vmo_get_size(handle)` where handle is the VMO handle to the imported external memory
    pub allocation_size: VkDeviceSize,

    /// `memory_type_index` is an index identifying a memory type from the `memory_types` array of
    /// the [`VkPhysicalDeviceMemoryProperties`] structure.
    ///
    /// # Valid Usage
    ///  - If the parameters define an import operation from an [`VkBufferCollectionFuchsia`],
    ///    `memory_type_index` must be from [`VkBufferCollectionPropertiesFuchsia`] as retrieved by
    ///    [`VkGetBufferCollectionPropertiesFuchsia`]
    ///  - If the parameters define an import operation and the external handle is an NT handle or
    ///    a global share handle created outside of the Vulkan API, the value of
    ///    `memory_type_index` must be one of those returned by
    ///    [`VkGetMemoryWin32HandlePropertiesKhr`]
    ///  - If the parameters define an import operation and the external handle is a POSIX file
    ///    descriptor created outside of the Vulkan API, the value of `memory_type_index` must be
    ///    one of those returned by [`VkGetMemoryFdPropertiesKhr`]
    ///  - If the `protected_memory` feature is not enabled, the
    ///    [`VkMemoryAllocateInfo::memory_type_index`] must not indicate a memory type that
    ///    reports [`VkMemoryPropertyFlag::Protected`]
    ///  - If the parameters define an import operation and the external handle is a host pointer,
    ///    the value of `memory_type_index` must be one of those returned by
    ///    [`VkGetMemoryHostPointerPropertiesExt`]
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::AndroidHardwareBufferAndroid`], `memory_type_index`
    ///    must be one of those returned by [`VkGetAndroidHardwareBufferPropertiesAndroid`] for the
    ///    Android hardware buffer
    ///  - If the parameters define an import operation and the external handle type is
    ///    [`VkExternalMemoryHandleTypeFlag::ScreenBufferQnx`], `memory_type_index` must be one of
    ///    those returned by [`VkGetScreenBufferPropertiesQnx`] for the QNX Screen buffer
    pub memory_type_index: u32,
}

impl const Default for VkMemoryAllocateInfo {
    fn default() -> Self {
        VkMemoryAllocateInfo {
            r#type: VkStructureType::MemoryAllocateInfo,
            next: null(),
            allocation_size: 0,
            memory_type_index: 0,
        }
    }
}

impl NextChain for VkMemoryAllocateInfo {
    fn structure_type(&self) -> VkStructureType {
        self.r#type
    }

    fn next(&self) -> *const c_void {
        self.next
    }

    fn as_ptr(&self) -> *const c_void {
        (self as *const Self).cast()
    }

    fn set_next(&mut self, next: Option<&dyn NextChain>) {
        self.next = next.map_or(null(), |n| n.as_ptr());
    }
}
