//! trait pour les dispositifs de stockage

use crate::error::Fat32Error;

/// trait pour un dispositif bloc
pub trait BlockDevice {
    /// lit un secteur
    fn read_sector(&self, sector: u32, buffer: &mut [u8]) -> Result<(), Fat32Error>;
    
    /// écrit un secteur
    fn write_sector(&mut self, sector: u32, buffer: &[u8]) -> Result<(), Fat32Error>;
    
    /// retourne la taille d'un secteur
    fn sector_size(&self) -> u32 {
        512
    }
}

