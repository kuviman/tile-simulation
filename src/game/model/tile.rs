use super::*;

#[derive(Debug, Clone)]
pub struct Tile {
    pub updated: bool,
    pub needs_update: bool,
    pub position: IVec2,
    pub content: TileContent,
}

impl Tile {
    pub fn move_directions(&self) -> Vec<IVec2> {
        match &self.content {
            TileContent::Solid { tile_solid } => match tile_solid {
                TileSolid::Barrier => vec![],
                _ => vec![ivec2(0, -1), ivec2(-1, -1), ivec2(1, -1)],
            },
            TileContent::Liquid { .. } => vec![
                ivec2(0, -1),
                ivec2(-1, -1),
                ivec2(1, -1),
                ivec2(-1, 0),
                ivec2(1, 0),
            ],
            TileContent::Gas { .. } => vec![
                ivec2(0, 1),
                ivec2(-1, 1),
                ivec2(1, 1),
                ivec2(-1, 0),
                ivec2(1, 0),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub enum TileContent {
    Solid { tile_solid: TileSolid },
    Liquid { tile_liquid: TileLiquid },
    Gas { tile_gas: TileGas },
}

#[derive(Debug, Clone, Copy)]
pub enum TileSolid {
    Barrier,
    Sand,
}

#[derive(Debug, Clone, Copy)]
pub enum TileLiquid {
    Water,
}

#[derive(Debug, Clone, Copy)]
pub enum TileGas {
    Smoke,
}
