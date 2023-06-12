use core::ffi::c_void;
use psp::sys::*;
use psp::{vram_alloc::get_vram_allocator, Align16};

/// Width for PSP Buffer (must accumulate the nearest 'double' amount greater then 'PSP_SCR_WIDTH' to swear all guarantes)
const PSP_BUF_WIDTH: u16 = 512;
/// PSP Screen width in pixels
const PSP_SCR_WIDTH: u16 = 490;
/// PSP Screen Height in pixels
const PSP_SCR_HEIGHT: u16 = 272;

// List which stores graphic command which next will be send to GPU to be executed in determined by itself direction
static mut GRP_LIST: Align16<[u8; 0x40000]> = Align16([0; 0x40000]);

/// Only as example: Calculated required memory size (MB = Megabytes) for VRAM (Video Ram = Graphic card Ram) Buffer size
#[allow(dead_code)]
fn get_memory_size(width: u16, height: u16, psm: TexturePixelFormat) -> u32 {
    use TexturePixelFormat::*;

    // Basic size calculating using known width and height within strict mode
    let size = width as u32 * height as u32;

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

/// Only as example: Creates a VRAM buffer
#[allow(dead_code)]
fn create_vram_buffer(width: u16, height: u16, psm: TexturePixelFormat) -> *mut u32 {
    let mut offset: u32 = 0; // offset which will be increasing each time when user take new texture to VRAM Buffer
    let result = &mut offset as *mut u32;
    let mem_size = self::get_memory_size(width, height, psm);

    offset += mem_size;

    result
}

/// Only as example: Assigning our created VRAM Buffer to the bengining of where VRAM is allocated (operation will be performing by CPU)
#[allow(dead_code)]
unsafe fn get_vram_texture(width: u16, height: u16, psm: TexturePixelFormat) -> u32 {
    let result = create_vram_buffer(width, height, psm);

    *result + *sceGeEdramGetAddr() as u32
}

/// Make configuration setup for graphic
unsafe fn init_graphic() {
    // Simplifies video memory allocation for buffers
    let allocator = get_vram_allocator().unwrap();

    // This is draw buffer (to draw something to the end before it will be displaying to user PSP screen)
    let buf0 = allocator.alloc_texture_pixels(PSP_BUF_WIDTH as u32, PSP_SCR_HEIGHT as u32, TexturePixelFormat::PsmT4).as_mut_ptr_from_zero();
    // This is displaying buffer (to display result of drawing to the screen of PSP). Displaying is performing by swaping 'buf1' content with contented graphic stored actualy in 'buf0'
    let buf1 = allocator.alloc_texture_pixels(PSP_BUF_WIDTH as u32, PSP_SCR_HEIGHT as u32, TexturePixelFormat::PsmT4).as_mut_ptr_from_zero();
    // This is to obtain static VRAM Buffer
    let zbuf = allocator.alloc_texture_pixels(PSP_BUF_WIDTH as u32, PSP_SCR_HEIGHT as u32, TexturePixelFormat::Psm4444).as_mut_ptr_from_zero();

    // Init graphic as first step of graphic creation and displaying it
    sceGuInit();

    // Start filling list of commands to affort them to be recorded and next sent to GPU engine which initializes and setup context to displaying result of commands
    sceGuStart(GuContextType::Direct, &mut GRP_LIST as *mut _ as *mut c_void);

    // Setup Draw buffer
    sceGuDrawBuffer(DisplayPixelFormat::Psm8888, buf0 as *mut c_void, PSP_BUF_WIDTH.into());

    // Setup Display buffer
    sceGuDispBuffer(PSP_SCR_WIDTH.into(), PSP_SCR_HEIGHT.into(), buf1 as *mut c_void, PSP_BUF_WIDTH as i32);

    // Setup Depth buffer
    sceGuDepthBuffer(zbuf as *mut c_void, PSP_BUF_WIDTH.into());

    // Set virtual coordinate offset for PSP. To determine where rendering should happen
    sceGuOffset((2048 - (PSP_SCR_WIDTH / 2)) as u32, (2048  - (PSP_SCR_HEIGHT / 2)) as u32);

    // Set a viewport with specific size in specific screen point
    sceGuViewport(2048, 2048, PSP_SCR_WIDTH as i32, PSP_SCR_HEIGHT as i32);

    // Set range for calculations in buffer depth in inversed depth span (PSP uses inversed order of Depth Buffer)
    sceGuDepthRange(u16::MAX as i32, 0);

    // Cut everything what is outside screen width and height viewport from graphic rendering
    sceGuEnable(GuState::ScissorTest);
    sceGuScissor(0, 0, PSP_SCR_WIDTH.into(), PSP_SCR_HEIGHT.into());

    // Enable and Select depth-test function
    sceGuEnable(GuState::DepthTest);
    sceGuDepthFunc(DepthFunc::Equal);

    // Setup current front-face order
    sceGuEnable(GuState::CullFace);
    sceGuFrontFace(FrontFaceDirection::Clockwise);

    // Setup shading model -> Smooth is per vertext thus not per pixel shading mode
    sceGuShadeModel(ShadingModel::Smooth); 

    // Setup last things for sceGu library
    sceGuEnable(GuState::Texture2D); // textures modes 
    sceGuEnable(GuState::ClipPlanes); // clip everything what is outside the rendering scene from rendering demand

    // Finish sceGu configuration setup located in whole this function context
    sceGuFinish(); // finish list and send it to execution (whole commands list setted up above)
    sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait); // waits until GPU executes all commands list to send another list of commands
    sceDisplayWaitVblankStart(); // wait until next avaiable time to get next Vsync frame
    sceGuDisplay(true); // enable PSP display to show graphic rendering status
}

/// To manage over **graphic rendering**
struct GMng;
impl GMng {
    /// Stop rendering graphics
    unsafe fn terminate_graphics() {
        sceGuTerm();
    }

    /// Start new frame
    unsafe fn start_new_frame() {
        sceGuStart(GuContextType::Direct, &mut GRP_LIST as *mut _ as *mut c_void);
    }

    /// End existing frame by displaying it on PSP screen
    unsafe fn end_existing_frame() {
        sceGuFinish(); // finish current display list
        sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait); // wait until GPU executes all commands list before send to execute new list with commands
        sceDisplayWaitVblank(); // wait until next avaiable screen Vsync frame
        sceGuSwapBuffers(); // swap draw buffer with display buffer to show graphic rendering result on PSP screen
    }
}

/// Change color of PSP background screen
pub unsafe fn background() -> () {
    let should_run = true;

    // Initialise graphic
    init_graphic();
    
    // Loop to graphic rendering
    while should_run {
        GMng::start_new_frame();
        
        // Setup color in reverse RGBA order
        sceGuClearColor(rgba(23, 165, 85, 0));

        // Clear draw buffer of colors to show user color
        sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::DEPTH_BUFFER_BIT);

        GMng::end_existing_frame();
    }

    // Stop graphic rendering
    GMng::terminate_graphics();

    // Exit game at the end
    sceKernelExitGame();
}
