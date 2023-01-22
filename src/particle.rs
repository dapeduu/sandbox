#[derive(Copy,Clone)]
pub enum ParticleType{
    Particle(Particle),
    SandParticle(SandParticle)
}

#[derive(Copy,Clone)]
pub enum ParticleNum{
    Base,
    Sand,
}

#[derive(Copy,Clone)]
pub struct Particle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

#[derive(Copy,Clone)]
pub struct SandParticle{
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

pub trait BaseParticle {
    fn move_particle(&mut self, frame: &mut [u8]);

    fn colision(&self, frame: &mut [u8]) -> bool;
}