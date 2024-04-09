#[derive(Clone, Copy)]
pub struct GameMap {
    map: [u8; 49],
    pub width: u8,
    pub height: u8,
}

impl Default for GameMap {
    fn default() -> Self {
        Self {
            map: [
                1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 1,
                1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1,
            ],
            width: 7,
            height: 7,
        }
    }
}

impl GameMap {
    pub fn get_xy(self, x: u8, y: u8) -> Option<u8> {
        let index: usize = ((y * self.height) + x).into();

        self.map.get(index).copied()
    }

    pub fn get(self, x: u8, y: u8) -> MapElement {
        let xy = self.get_xy(x, y);

        MapElement::from(xy)
    }

    pub fn out_of_bounds(self, x: u8, y: u8) -> bool {
        x >= self.map.len() as u8 || y >= self.map.len() as u8
    }
}

pub enum MapElement {
    Void,
    Wall,
}

impl From<Option<u8>> for MapElement {
    fn from(value: Option<u8>) -> Self {
        match value {
            Some(n) => match n {
                1 => Self::Wall,
                0 => Self::Void,
                _ => Self::Wall,
            },
            None => Self::Void,
        }
    }
}

impl Into<u8> for MapElement {
    fn into(self) -> u8 {
        match self {
            MapElement::Void => 0,
            MapElement::Wall => 1,
        }
    }
}
