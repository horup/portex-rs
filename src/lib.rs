use std::hash::Hash;

use slotmap::{SlotMap, new_key_type};
use std::collections::HashMap;
new_key_type! {
    pub struct VertexID;
    pub struct LineID;
    pub struct SectorID;
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub x:i32,
    pub y:i32
}

#[derive(Clone, Copy, Debug)]
pub struct Line {
    pub p1:VertexID,
    pub p2:VertexID
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        (self.p1 == other.p1 && self.p2 == other.p2)
        ||
        (self.p1 == other.p2 && self.p2 == other.p1)
    }
}
impl Eq for Line {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Sector {
    pub lines:Vec<LineID>
}

impl Sector {
    pub fn is_closed(&self, lines: &SlotMap<LineID, Line>) -> bool {
        if self.lines.is_empty() {
            return false;
        }

        // Check if all lines form a closed loop
        let mut vertex_count: HashMap<VertexID, usize> = HashMap::new();
        
        // Count how many times each vertex appears
        for &line_id in &self.lines {
            if let Some(line) = lines.get(line_id) {
                *vertex_count.entry(line.p1).or_insert(0) += 1;
                *vertex_count.entry(line.p2).or_insert(0) += 1;
            } else {
                return false; // Invalid line reference
            }
        }
        
        // In a closed loop, each vertex should appear exactly twice
        vertex_count.values().all(|&count| count == 2)
    }
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

impl From<(i32, i32)> for Vertex {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl World {
    pub fn new_vertex(&mut self, vertex: Vertex) -> VertexID {
        self.vertices.insert(vertex)
    }

    pub fn new_line(&mut self, p1: VertexID, p2: VertexID) -> LineID {
        self.lines.insert(Line { p1, p2 })
    }

    pub fn new_sector(&mut self, lines: Vec<LineID>) -> SectorID {
        self.sectors.insert(Sector { lines })
    }

    pub fn split_line(&mut self, line:LineID, new_vertex:Vertex) {
        if let Some(original_line) = self.lines.remove(line) {
            let new_vertex_id = self.new_vertex(new_vertex);
            
            // Create two new lines: p1 -> new_vertex and new_vertex -> p2
            let line1_id = self.new_line(original_line.p1, new_vertex_id);
            let line2_id = self.new_line(new_vertex_id, original_line.p2);
            
            // Update all sectors that reference the original line
            for sector in self.sectors.values_mut() {
                if let Some(pos) = sector.lines.iter().position(|&id| id == line) {
                    // Replace the original line with the two new lines
                    sector.lines[pos] = line1_id;
                    sector.lines.insert(pos + 1, line2_id);
                }
            }
        }
    }

    pub fn vertex(&self, id: VertexID) -> Option<&Vertex> {
        self.vertices.get(id)
    }

    pub fn vertex_mut(&mut self, id: VertexID) -> Option<&mut Vertex> {
        self.vertices.get_mut(id)
    }

    pub fn merge_vertices(&mut self, vertices:impl Iterator<Item = VertexID>) {
        let vertices_vec: Vec<VertexID> = vertices.collect();
        if vertices_vec.is_empty() {
            return;
        }

        // Calculate the center point
        let mut sum_x = 0i64;
        let mut sum_y = 0i64;
        let mut valid_count = 0;

        for &vertex_id in &vertices_vec {
            if let Some(vertex) = self.vertices.get(vertex_id) {
                sum_x += vertex.x as i64;
                sum_y += vertex.y as i64;
                valid_count += 1;
            }
        }

        if valid_count == 0 {
            return;
        }

        let center = Vertex {
            x: (sum_x / valid_count as i64) as i32,
            y: (sum_y / valid_count as i64) as i32,
        };

        // Keep the first vertex and update its position to the center
        let first_vertex_id = vertices_vec[0];
        if let Some(first_vertex) = self.vertices.get_mut(first_vertex_id) {
            *first_vertex = center;
        }

        // Update all lines that reference the other vertices to use the first vertex instead
        for &vertex_id in &vertices_vec[1..] {
            for line in self.lines.values_mut() {
                if line.p1 == vertex_id {
                    line.p1 = first_vertex_id;
                }
                if line.p2 == vertex_id {
                    line.p2 = first_vertex_id;
                }
            }
            // Remove the merged vertex
            self.vertices.remove(vertex_id);
        }
    }
}

#[derive(Default)]
pub struct SectorBuilder {
    vertices:Vec<Vertex>
}

impl SectorBuilder {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new()
        }
    }

    pub fn add_vertex(&mut self, x: i32, y: i32) -> &mut Self {
        self.vertices.push(Vertex { x, y });
        self
    }

    pub fn build(self, world: &mut World) -> SectorID {
        // Add vertices to world
        let vertex_ids: Vec<VertexID> = self.vertices
            .into_iter()
            .map(|v| world.new_vertex(v))
            .collect();

        // Create lines connecting consecutive vertices
        let mut line_ids = Vec::new();
        for i in 0..vertex_ids.len() {
            let next_i = (i + 1) % vertex_ids.len();
            let line_id = world.new_line(vertex_ids[i], vertex_ids[next_i]);
            line_ids.push(line_id);
        }

        // Create and return sector
        world.new_sector(line_ids)
    }
}

impl Default for SectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}