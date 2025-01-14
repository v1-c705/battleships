use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnArray{
    pub array: [[i32; 10]; 10],
}

#[derive(Clone, Copy, PartialEq)]
pub enum Size{
    NoTiles = 0,
    TwoTiles = 2,
    ThreeTiles = 3,
    FourTiles = 4,
    FiveTiles = 5
}

#[derive(Clone, Copy)]
pub enum Orientation{
    HorizontalRight,
    HorizontalLeft,
    VerticalUp,
    VerticalDown
}