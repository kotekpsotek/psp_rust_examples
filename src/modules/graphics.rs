use core::ffi::c_void;

use psp::sys::*;

const PSP_BUF_WIDTH: u16 = 512;
const PSP_SCR_WIDTH: u16 = 490;
const PSP_SCR_HEIGHT: u16 = 272;

// List which stores graphic command which next will be send to GPU to be executed in determined by itself direction
static mut GRP_LIST: &[u8] = &[];

/// Calculated required memory size (MB = Megabytes) for VRAM (Video Ram = Graphic card Ram) Buffer size
fn get_memory_size(width: u16, height: u16, psm: TexturePixelFormat) -> u16 {
    use TexturePixelFormat::*;

    // Basic size calculating using known width and height within strict mode
    let mut size = width * height;

    // Convert calculated above "size" using definition of 'TexturePixelFormat'
    match psm {
        // RGBA handle: 4 color values rgba (red, green, blue, alpha) = 4 bits per one color value (e.g: 4 for "r" etc...) = 2 bytes per pixel at all
        PsmT4 => size / 2,
        // GrayScale handle: 2 bits per one color = per each bit is assigned one pixel which has got 4 shaded for every color
        PsmT8 => size,
        // All psmX where X > 1000 has got specified amount of bits per each color channel = 2 bytes per one pixel
        Psm5650 | Psm5551 | Psm4444 | PsmT16 => size * 2,
        // 1 byte per each color channel assigned for pixel which has got 4 shades (rgba = red, green, blue, alpha = transparency)
        Psm8888 | PsmT32 => size * 4,
        // Only for safety reasons
        _ => 0
    }
}

/// Creates a VRAM buffer
fn create_vram_buffer(width: u16, height: u16, psm: TexturePixelFormat) -> *mut u16 {
    let mut offset: u16 = 0; // offset which will be increasing each time when user take new texture to VRAM Buffer
    let result = &mut offset as *mut u16;
    let mem_size: u16 = self::get_memory_size(width, height, psm);

    offset += mem_size;

    result
}

// Assigning our created VRAM Buffer to the bengining of where VRAM is allocated (operation will be performing by CPU)
unsafe fn get_vram_texture(width: u16, height: u16, psm: TexturePixelFormat) -> u16 {
    let result = create_vram_buffer(width, height, psm);

    *result + *sceGeEdramGetAddr() as u16
}

unsafe fn init_graphic() {
    // This is draw buffer (to draw something to the end before it will be displaying to user PSP screen)
    let buf0 = create_vram_buffer(PSP_BUF_WIDTH, PSP_SCR_HEIGHT, TexturePixelFormat::PsmT4);
    // This is displaying buffer (to display result of drawing to the screen of PSP). Displaying is performing by swaping 'buf1' content with contented graphic stored actualy in 'buf0'
    let buf1 = create_vram_buffer(PSP_BUF_WIDTH, PSP_SCR_HEIGHT, TexturePixelFormat::PsmT4);
    // This is to obtain static VRAM Buffer
    let zbuf = create_vram_buffer(PSP_BUF_WIDTH, PSP_SCR_HEIGHT, TexturePixelFormat::Psm4444); // here is different texture pixel format equal to gray scale because zbuffer doesn't need to record and doesn't at all recording rdba colors set

    // Init graphic as first step of graphic creation and displaying it
    sceGuInit();

    // Start filling list of commands to affort them to be recorded and next sent to GPU engine which initializes and setup context to displaying result of commands
    sceGuStart(GuContextType::Direct, GRP_LIST.as_ptr() as *mut c_void);

    // Setup Draw buffer
    sceGuDrawBuffer(DisplayPixelFormat::Psm8888, buf0 as *mut c_void, PSP_BUF_WIDTH.into());

    // Setup Display buffer
    sceGuDispBuffer(PSP_SCR_WIDTH.into(), PSP_SCR_HEIGHT.into(), buf1 as *mut c_void, PSP_BUF_WIDTH as i32);

    // Setup Depth buffer
    sceGuDepthBuffer(zbuf as *mut c_void, PSP_BUF_WIDTH.into());
}

pub unsafe fn background() -> () {
    let mut should_run = true;
    
    while should_run {
        
    }

    // Exit game at the end
    sceKernelExitGame();
}
