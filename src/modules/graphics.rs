use core::ffi::{c_void, c_short};
use psp::sys::*;
use psp::{vram_alloc::get_vram_allocator, Align16};
use embedded_graphics::{prelude::*, primitives::*, pixelcolor::Rgb888};
use psp::embedded_graphics::Framebuffer;
use crate::shapes::*;
use crate::examples::types_def::Texture;

use super::types_def::{Dimension};

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

    // Loading texture and returning it as 'struct **Texture**' data type 'structure'
    /* unsafe fn load_texture(image_by: &[u8]) -> Texture {
        // use 'stb' helping library
        use stb::image::*;

        // S.c: Helper functions
        let pow2 = |num: i32| {
            // Define variable which will be expanded by powering operation
            let mut pw2 = 1;

            // Power to moment when 'pw2' variable is greater then 'num' parameter
            while pw2 < num {
                pw2 <<= 1; // multiply pw2 by 2 in each loop iteration
            };

            // return result
            pw2
        };

        // Storage for texture datas
        let mut texture = Texture::default();
        
        // Flip loaded image
        stbi_set_flip_vertically_on_load(true);

        // Load image // TODO: Loading texture file defined under 'src' attribute
        let image_data: &[u8] = &[]; 
        let ld = stbi_load_from_memory(image_data, Channels::RgbAlpha)
            .unwrap(); // FIXME: .unwrap() combinator isn't handled by 'psp' by default so this can cause unexpected behaviour
            // Assign image properties to texture struct
        texture.width = ld.0.width;
        texture.height = ld.0.height;
        texture.p_w = pow2(ld.0.width);
        texture.p_h = pow2(ld.0.height);

        // Define size of texture as bytes per pixel for calculate buffer size for texture
        let size = texture.p_w * texture.p_h * 4; // times 4 because per 1 pixel are assigned 4 bytes

        // TODO: ... Create data buffer and rest of loading texture with above 'todo' one

        texture
    }

    /// Bind texture to GPU (texture application for next drawning element will be added to commands execution list)
    unsafe fn bind_texture(texture: &Texture) {
        sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 1);
        sceGuTexFunc(TextureEffect::Modulate, TextureColorComponent::Rgba); // setup texture color
        sceGuTexFilter(TextureFilter::Nearest, TextureFilter::Nearest); // how textures are filtering
        sceGuTexWrap(GuTexWrapMode::Repeat, GuTexWrapMode::Repeat); // Setup multiplying or cliping texture
        sceGuTexImage(MipmapLevel::None, texture.p_w, texture.p_h, texture.p_w, texture.data); // Set texture to GPU with specified for function configuration
    } */
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

    // Load texture
    let tex_bytes = include_bytes!("../../files/texture.jpg");
    let Texture { bytes, width, height, tbw } = Texture::tex_load(tex_bytes, Dimension { w: 1299, h: 1300 });

    while draw {
        GMng::start_new_frame();

        // Disable some unnsessary things // Must be enabled for texture loading
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
        
        // Draw square with assigned texture
        sceGuEnable(GuState::Texture2D);
        change_translate(0.0, -0.45, 0f32);
        sceGuTexMode(TexturePixelFormat::Psm8888, 0, 0, 0);
        sceGuTexImage(MipmapLevel::None, width, height, tbw, bytes);
        sceGuTexFunc(TextureEffect::Replace, TextureColorComponent::Rgba);
        sceGuTexFilter(TextureFilter::Linear, TextureFilter::Linear);

        sceGumDrawArray(GuPrimitive::Triangles, VertexType::COLOR_8888 | VertexType::VERTEX_32BITF | VertexType::TEXTURE_32BITF | VertexType::TRANSFORM_3D, 6, core::ptr::null(), &SQUARE as *const _ as *const c_void);


        GMng::end_existing_frame();
    }

    GMng::terminate_graphics();
}
