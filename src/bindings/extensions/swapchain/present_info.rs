use super::VkSwapchainKHR;
use crate::{
    bindings::{VkSemaphore, VkStructureType},
    VkResult,
};
use std::{alloc::Layout, ffi::c_void, marker::PhantomData, ptr::NonNull};

#[repr(C)]
pub struct VkPresentInfoKHR<'a> {
    s_type: VkStructureType,
    p_next: Option<NonNull<c_void>>,
    wait_semaphore_count: u32,
    p_wait_semaphores: Option<NonNull<VkSemaphore>>,
    swapchain_count: u32,
    p_swapchains: NonNull<VkSwapchainKHR>,
    p_image_indices: NonNull<u32>,
    p_results: NonNull<VkResult>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> VkPresentInfoKHR<'a> {
    pub fn new(
        semaphores: &'a [&mut crate::VkSemaphore],
        swapchains: &'a [&mut VkSwapchainKHR],
        image_indices: &'a [u32],
    ) -> Self {
        assert!(swapchains.len() > 0);
        assert_eq!(swapchains.len(), image_indices.len());

        VkPresentInfoKHR {
            s_type: VkStructureType::SwapchainPresentInfo,
            p_next: None,
            wait_semaphore_count: semaphores.len() as u32,
            p_wait_semaphores: if semaphores.len() == 0 {
                None
            } else {
                let layout = Layout::from_size_align(
                    std::mem::size_of::<VkSemaphore>() * semaphores.len(),
                    std::mem::align_of::<VkSemaphore>(),
                )
                .unwrap();

                Some(NonNull::new(unsafe { std::alloc::alloc(layout) } as _).unwrap())
            },
            swapchain_count: swapchains.len() as u32,
            p_swapchains: {
                let layout = Layout::from_size_align(
                    std::mem::size_of::<VkSwapchainKHR>() * swapchains.len(),
                    std::mem::align_of::<VkSwapchainKHR>(),
                )
                .unwrap();

                NonNull::new(unsafe { std::alloc::alloc(layout) } as _).unwrap()
            },
            p_image_indices: unsafe { NonNull::new_unchecked(image_indices.as_ptr() as _) },
            p_results: {
                let layout = Layout::from_size_align(
                    std::mem::size_of::<VkResult>() * swapchains.len(),
                    std::mem::align_of::<VkResult>(),
                )
                .unwrap();

                NonNull::new(unsafe { std::alloc::alloc(layout) } as _).unwrap()
            },
            phantom: PhantomData,
        }
    }
}

impl<'a> Drop for VkPresentInfoKHR<'a> {
    fn drop(&mut self) {
        self.p_wait_semaphores
            .take()
            .map(|p_wait_semaphores| unsafe {
                std::alloc::dealloc(
                    p_wait_semaphores.as_ptr() as *mut _,
                    Layout::from_size_align(
                        std::mem::size_of::<VkSemaphore>() * self.wait_semaphore_count as usize,
                        std::mem::align_of::<VkSemaphore>(),
                    )
                    .unwrap(),
                )
            });

        unsafe {
            std::alloc::dealloc(
                self.p_swapchains.as_ptr() as _,
                Layout::from_size_align(
                    std::mem::size_of::<VkSwapchainKHR>() * self.swapchain_count as usize,
                    std::mem::align_of::<VkSwapchainKHR>(),
                )
                .unwrap(),
            )
        }

        unsafe {
            std::alloc::dealloc(
                self.p_results.as_ptr() as _,
                Layout::from_size_align(
                    std::mem::size_of::<VkResult>() * self.swapchain_count as usize,
                    std::mem::align_of::<VkResult>(),
                )
                .unwrap(),
            )
        }
    }
}
