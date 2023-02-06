#[derive(Copy, Clone)]
pub enum ParticleType {
    Particle(Particle),
    SandParticle(SandParticle),
    IronParticle(IronParticle),
    AcidParticle(AcidParticle),
    WaterParticle(WaterParticle),
    AgitatedParticle(AgitatedParticle),
}

#[derive(Copy, Clone)]
pub enum ParticleNum {
    Base,
    Sand,
    Iron,
    Acid,
    Water,
    Agitated,
}

#[derive(Copy, Clone)]
pub struct Particle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

#[derive(Copy, Clone)]
pub struct SandParticle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

#[derive(Copy, Clone)]
pub struct IronParticle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

#[derive(Copy,Clone)]
pub struct AcidParticle{
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

#[derive(Copy,Clone)]
pub struct WaterParticle{
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

#[derive(Copy,Clone)]
pub struct AgitatedParticle{
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

pub trait BaseParticle {
    fn move_particle(&mut self, frame: &mut [u8]);

    fn colision(&self, frame: &mut [u8]) -> bool;

}
