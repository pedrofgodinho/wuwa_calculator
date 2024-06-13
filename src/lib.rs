
pub mod calculator;
pub mod echo;

#[derive(Clone, Copy)]
pub enum Element {
    Glacio,
    Fusion,
    Electro,
    Aero,
    Spectro,
    Havoc,
}

#[derive(Clone, Copy)]
pub enum SkillType {
    Skill = 0,
    Basic,
    Heavy,
    Liberation,
}