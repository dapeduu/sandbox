//! Definição das Structs para cada Partícula, declaração das traits básicas de movimento e colisão, definição de Enums para os tipos das partículas.
//!
//! Toda Partícula possui a mesma estrutura base, contendo posições x,y e uma cor
//! ```
//! pub struct Particle {
//!     pub x: u32,
//!     pub y: u32,
//!     pub rgba: [u8; 4],
//! }
//! ```

///Enumeration Tipada, constructo específico do rust que se comporta como uma Union de C, utilizada para construir um vetor heterogêneo de partículas
#[derive(Copy, Clone)]
pub enum ParticleType {
    Particle(Particle),
    SandParticle(SandParticle),
    IronParticle(IronParticle),
    WaterParticle(WaterParticle),
    AgitatedParticle(AgitatedParticle),
    ElectricityParticle(ElectricityParticle),
}

///Enumeration Tradicional, utilizada para associar tipos às teclas e para realizar o switch
#[derive(Copy, Clone)]
pub enum ParticleNum {
    Base,
    Sand,
    Iron,
    Water,
    Agitated,
    Electricity,
}

///Base, apenas cai
#[derive(Copy, Clone)]
pub struct Particle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

///Areia, cai e forma pirâmide de apoio
#[derive(Copy, Clone)]
pub struct SandParticle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

///Ferro, não se move
#[derive(Copy, Clone)]
pub struct IronParticle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

///Cai e escorrega para os lados tentando cair
#[derive(Copy, Clone)]
pub struct WaterParticle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

///Se move aleatoriamente
#[derive(Copy, Clone)]
pub struct AgitatedParticle {
    pub x: u32,
    pub y: u32,
    pub rgba: [u8; 4],
}

///Parada, quando entra em contato com água se move dentro dela
#[derive(Copy, Clone)]
pub struct ElectricityParticle {
    pub x: u32,
    pub y: u32,
    pub life_time: u8,
    pub rgba: [u8; 4],
}

///Trait base para todas as partículas
pub trait BaseParticle {
    /// Função de movimento da partícula
    fn move_particle(&mut self, frame: &mut [u8]);
    /// Função de colisão da partícula
    fn colision(&self, frame: &mut [u8]) -> bool;
}
