use core::ffi::c_void;

use psp::Align16;

/// It is using to represent one point of rendering in graphic
/// It is also using pretty much as a default drawning mode for everything because it gives you necessary control over what you do
#[repr(C)]
pub struct Vertex {
    pub u: f32,
    pub v: f32,
    pub color: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[repr(C)]
pub struct Texture {
    pub bytes: *const c_void,
    pub width: i32,
    pub height: i32,
    pub tbw: i32
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            bytes: &[] as &[u8; 0] as *const _ as *const c_void,
            width: 0,           
            height: 0,
            tbw: i32::default()
        }
    }
}

/// Store image fundamental informations
#[derive(Debug)]
pub struct Dimension {
    /// Width of image
    pub w: i32,
    /// Height of image
    pub h: i32,
}

impl Texture {
    /// Load texture widt it retured parameters
    pub fn tex_load(bytes: &[u8], d: Dimension) -> Self {
        // Get image dimensions
        let Dimension { w, h } = d;

        // Return ready texture object
        Self {
            bytes: &Align16(bytes) as *const _ as *const c_void,
            width: w,
            height: h,
            tbw: w.pow(2)
        }
    }
}
