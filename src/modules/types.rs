use core::ffi::c_void;

/// It is using to represent one point of rendering in graphic
/// It is also using pretty much as a default drawning mode for everything because it gives you necessary control over what you do
#[repr(C)]
pub struct Vertex {
    pub color: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[repr(C)]
pub struct Texture {
    pub data: *const c_void,
    pub p_w: i32,
    pub p_h: i32,
    pub width: i32,
    pub height: i32
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            data: &[] as &[u8; 0] as *const _ as *const c_void,
            p_w: 0,           
            p_h: 0,           
            width: 0,           
            height: 0  
        }
    }
}