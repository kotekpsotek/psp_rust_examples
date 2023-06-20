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
    pub fn tex_load(bytes: &[u8]) -> Self {
        // Get image dimensions
        let [w, h] = match Self::tex_dimensions(bytes) {
            Ok(Dimension { w, h }) => [w, h],
            Err(_) => [0, 0]
        };

        // Return ready texture object
        Self {
            bytes: bytes as *const _ as *const c_void,
            width: w,
            height: h,
            tbw: {
                0
            }
        }
    }

    /// Get image dimension
    pub fn tex_dimensions(bytes: &[u8]) -> Result<Dimension, ()> {
        // Works now only for JPG image types
        let mut off = 0;
        while off < bytes.len() {
            while bytes[off]==0xff {
                off+=1;
            }

            let mrkr = bytes[off];
            off += 1;
            
            if mrkr==0xd8 { // SOI
                continue;
            }
            
            if mrkr==0xd9 { // EOI
                break;
            }

            if 0xd0<=mrkr && mrkr<=0xd7 {
                continue;
            }

            if mrkr==0x01 { // TEM
                continue;
            }

            let len = (bytes[off].wrapping_shl(8)) | bytes[off+1];  
            off+=2;  
            
            if mrkr == 0xc0 {
                let w = ((bytes[off+3].wrapping_shl(8)) | bytes[off+4]) as i32;
                let h = ((bytes[off+1].wrapping_shl(8)) | bytes[off+2]) as i32;
                
                return Ok(Dimension {
                    w,
                    h
                })
            }

            off += len.wrapping_sub(2) as usize;
        };

        return Err(());
    }
}
