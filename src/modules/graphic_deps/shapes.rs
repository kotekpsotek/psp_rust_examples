use crate::examples::types_def::Vertex;
use psp::sys::rgba;
use psp::Align16;
use core::ffi::c_short;

/// Define point of rendering for triangle
pub static TRIANGLE: Align16<[Vertex; 3]> = Align16([ // Each 'color' of course can be different but 'z' point for all 'indices' must be the same value
    Vertex { color: rgba(210, 0, 238, 0), x: 0.35, y: 0.0, z: -10f32 },
    Vertex { color: rgba(210, 10, 238, 0), x: -0.35, y: 0.0, z: -10f32 },
    Vertex { color: rgba(210, 0, 238, 0), x: 0.0, y: 0.5, z: -10f32 }
]);

pub static TRIANGLE_2: Align16<[Vertex; 3]> = Align16([ // right triangle version
    Vertex { color: rgba(247, 190, 3, 0), x: 0.0, y: 0.0, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: -0.5, y: 0.0, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.0, y: 0.5, z: -10f32 }
]);

/// Define points of rendering for square
///    2/3------1
///    |       |
///    4------0/5
pub static SQUARE: Align16<[Vertex; 6]> = Align16([
    Vertex { color: rgba(14, 212, 106, 0), x: -0.15, y: -0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: -0.15, y: 0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: 0.15, y: 0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: 0.15, y: 0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: 0.15, y: -0.15, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: -0.15, y: -0.15, z: -10f32 }
]);

/// Same schema as SQUARE
pub static RECTANGLE: Align16<[Vertex; 6]> = Align16([
    Vertex { color: rgba(247, 190, 3, 0), x: -0.15, y: -0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: -0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.15, y: -0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: -0.15, y: -0.3, z: -10f32 },
]);
    
    
/// Indexed version of rectangle
pub static RECTANGLE_INDX: Align16<[Vertex; 4]> = Align16([ // without double '2' and '0' indexes from normal 'RECTANGLE' list
    Vertex { color: rgba(247, 190, 3, 0), x: -0.15, y: -0.3, z: -10f32 },
    Vertex { color: rgba(14, 212, 106, 0), x: -0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(247, 190, 3, 0), x: 0.15, y: 0.3, z: -10f32 },
    Vertex { color: rgba(210, 0, 238, 0), x: 0.15, y: -0.3, z: -10f32 }
]);

/// Indexes for 'RECTANGLE_INDX'
pub static INDEXES_RECTANGLE: Align16<[c_short; 6]> = Align16([
    0, 1, 2, 2, 3, 0
]);
