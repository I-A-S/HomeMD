use super::math::{Vec3};

pub struct Atom {
    pub name: String,
    pub id: u64,
    pub mass: f32,
    pub charge: f32,

    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3
}

pub enum BondTopology {
    Linear([u64; 2]),
    Angle([u64; 3]),
    Dihedral([u64; 4])
}

pub struct Bond {
    pub id: u64,
    pub enabled: bool,
    pub stiffness: f32,
    pub eqb_value: f32,
    pub topology: BondTopology
}

pub struct Molecule {
    pub id: u64,
    pub name: String,
    pub bonds: Vec<Bond>
}

