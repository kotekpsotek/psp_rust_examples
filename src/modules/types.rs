/// It is using to represent one point of rendering in graphic
/// It is also using pretty much as a default drawning mode for everything because it gives you necessary control over what you do
#[repr(C)]
pub struct Vertex {
    pub color: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32
}