use win32::{HInstance, HWnd};

pub struct Win32SurfaceCreateInfo {
    pub h_instance: HInstance,
    pub h_wnd: HWnd,
}

impl From<HWnd> for Win32SurfaceCreateInfo {
    fn from(h_wnd: HWnd) -> Self {
        Win32SurfaceCreateInfo {
            h_instance: win32::get_module_handle(None).unwrap(),
            h_wnd,
        }
    }
}
