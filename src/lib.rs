use slotmap::{SlotMap, new_key_type};
new_key_type! {
    pub struct VertexID;
    pub struct LineID;
    pub struct SectorID;
}

pub struct Vertex {
    pub id:VertexID,
    pub x:i32,
    pub y:i32
}

pub struct Line {
    pub id:LineID,
    pub p1:VertexID,
    pub p2:VertexID
}

pub struct Sector {
    pub id:SectorID,
    pub lines:Vec<LineID>
}

pub struct World {
    vertices:SlotMap<VertexID, Vertex>,
    lines:SlotMap<LineID, Line>,
    sectors:SlotMap<SectorID, Sector>
}

impl Default for World {
    fn default() -> Self {
        Self { vertices: Default::default(), lines: Default::default(), sectors: Default::default() }
    }
}

impl World {
    
}