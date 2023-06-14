use core::ffi::{c_void, c_short};
use psp::sys::*;
use psp::{vram_alloc::get_vram_allocator, Align16};
use embedded_graphics::{prelude::*, primitives::*, pixelcolor::Rgb888};
use psp::embedded_graphics::Framebuffer;
use crate::examples::types_def::Vertex;

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

/// Make configuration setup for graphic```
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

/// Drawing shapes in PSP screen
/// It's performing using 'embedded-graphic' crate which is doing this is in simplier way then native 'sceGu' = 'sceGum' library
pub unsafe fn draw_shapes() -> () {
    // Define buffer for frames
    let mut display = Framebuffer::new();

    // Add background color to drawing shape area
    let _bckg_style = PrimitiveStyleBuilder::new()
        .fill_color(Rgb888::new(27, 152, 87))
        .build();
    let _bckg = Rectangle::new(
        Point::new(0, 0), 
        Size::new(PSP_SCR_WIDTH.into(), PSP_SCR_HEIGHT.into())
    );
    display.fill_solid(&_bckg, Rgb888::new(23, 165, 85)).unwrap();

    // Draw shapes
        // Triangle drawning
        //        2_Point = apex
        //            /  \
        //          /     \
        //        /        \
        //      /           \
        //  [0_Point ===> 1_Point] = basis points
    let rectangle_basis_width = 100;
    let start_point_x = PSP_SCR_WIDTH / 2 - rectangle_basis_width / 2;
    let middle_point_x = PSP_SCR_WIDTH / 2 - rectangle_basis_width / 2 + rectangle_basis_width / 2;
    let end_point_x = PSP_SCR_WIDTH / 2 - rectangle_basis_width / 2 + rectangle_basis_width;
    let _triangle = Triangle::new(
        Point::new(start_point_x.into(), 100 + 30),
        Point::new(end_point_x.into(), 100 + 30),
        Point::new(middle_point_x.into(), 100),
    )
        .into_styled(
        PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::RED)
                .fill_color(Rgb888::RED)
                .stroke_width(1)
                .build()
        )
        .draw(&mut display)
        .unwrap();
}

/// Define point of rendering for triangle
static TRIANGLE: Align16<[Vertex; 3]> = Align16([ // Each 'color' of course can be different but 'z' point for all 'indices' must be the same value
    Vertex { color: rgba(210, 0, 238, 0), x: 0.35, y: 0.0, z: -10f32 },
    Vertex { color: rgba(210, 10, 238, 0), x: -0.35, y: 0.0, z: -10f32 },
    Vertex { color: rgba(210, 0, 238, 0), x: 0.0, y: 0.5, z: -10f32 }
]);

static TRIANGLE_2: Align16<[Vertex; 3]> = Align16([ // right triangle version
    Vertex { color: rgba(247, 190, 3, 0), x: 0.0, y: 0.0, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: -0.5, y: 0.0, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.0, y: 0.5, z: -10f32 }
]);

/// Define points of rendering for square
///    2/3------1
///    |       |
///    4------0/5
static SQUARE: Align16<[Vertex; 6]> = Align16([
    Vertex { color: rgba(14, 212, 106, 0), x: -0.15, y: -0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: -0.15, y: 0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: 0.15, y: 0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: 0.15, y: 0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: 0.15, y: -0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: -0.15, y: -0.15, z: -10f32 }
]);

/// Same schema as SQUARE
static RECTANGLE: Align16<[Vertex; 6]> = Align16([
    Vertex { color: rgba(247, 190, 3, 0), x: -0.15, y: -0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: -0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.15, y: -0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: -0.15, y: -0.3, z: -10f32 },
]);
    
    
/// Indexed version of rectangle
static RECTANGLE_INDX: Align16<[Vertex; 4]> = Align16([ // without double '2' and '0' indexes from normal 'RECTANGLE' list
    Vertex { color: rgba(247, 190, 3, 0), x: -0.15, y: -0.3, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: -0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(210, 0, 238, 0), x: 0.15, y: -0.3, z: -10f32 }
]);

/// Indexes for 'RECTANGLE_INDX'
static INDEXES_RECTANGLE: Align16<[c_short; 6]> = Align16([
    0, 1, 2, 2, 3, 0
]);

/// Draw shapes in Graphic context using raw 'sceGu' library for this
#[allow(unused_mut)]
pub unsafe fn draw_shapes_native() {
    init_graphic();
    let mut draw = true;
    let change_translate = |x: f32, y: f32, z: f32| {
        // Reset 'Model' matrice
        sceGumMatrixMode(MatrixMode::Model);
        sceGumLoadIdentity();

        // Change translate position
        let position = ScePspFVector3 { x, y, z }; // defining position to change
        sceGumTranslate(&position); // setup start position for drawning
    };

    // Configure stuff for shapes drawning
    sceGumMatrixMode(MatrixMode::Projection); // whether user is in 3D (perspective matrix) or 2D (same view as creating)
    sceGumLoadIdentity(); // load default values to matrices (after multiplying values from matrices there isn't any result)
    sceGumOrtho(-16.0 / 9.0, 16.0 / 9.0, -1.0, 1.0, -10.0, 10.0); // to apply 'ortho' projection matrix
    // sceGumPerspective(50.0, 16f32 / 9f32, 0.5, 1000f32); // to apply perspective

    sceGumMatrixMode(MatrixMode::View); // for camera perspective
    sceGumLoadIdentity(); // same as above purpose

    sceGumMatrixMode(MatrixMode::Model); // assest position of current rendering model
    sceGumLoadIdentity(); // same as above purpose

    while draw {
        GMng::start_new_frame();

        // Disable some unnsessary things
        sceGuDisable(GuState::DepthTest);
        sceGuDisable(GuState::Texture2D); // This must be disabled to show colors for 3D objects rendered on screen

        // Apply color
        sceGuClearColor(rgba(9, 15, 129, 0));
        sceGuClear(ClearBuffer::COLOR_BUFFER_BIT);

        // Draw shape of triangle
        change_translate(-0.75, 0.15, 0f32);
        sceGumDrawArray(GuPrimitive::Triangles, VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D, 3, core::ptr::null(), &TRIANGLE as *const _ as *const c_void); // 2. attribure specifies what is using for rendering the whole graphic shape (drawning points with Vertex type)
        
        // Draw shape of straight triangle
        change_translate(-0.75 + 0.35, -0.5, 0.0);
        sceGumDrawArray(GuPrimitive::Triangles, VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D, 3, core::ptr::null(), &TRIANGLE_2 as *const _ as *const c_void);

        // Draw shape of square
        change_translate(0.0, 0.3, 0f32);
        sceGumDrawArray(GuPrimitive::Triangles, VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D, 6, core::ptr::null(), &SQUARE as *const _ as *const c_void);

        // Draw shape of rectangle
        change_translate(0.55, 0.45, 0f32);
        sceGumDrawArray(GuPrimitive::Triangles, VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D, 6, core::ptr::null(), &RECTANGLE as *const _ as *const c_void);
        
        // Draw shape of indexed rectangle
        change_translate(0.55, -0.5 + 0.3 / 2f32, 0f32);
        sceGumDrawArray(GuPrimitive::Triangles, VertexType::COLOR_8888 | VertexType::INDEX_16BIT | VertexType::VERTEX_32BITF | VertexType::TRANSFORM_3D, 6, &INDEXES_RECTANGLE as *const _ as *const c_void, &RECTANGLE_INDX as *const _ as *const c_void);
        
        GMng::end_existing_frame();
    }

    GMng::terminate_graphics();
}
