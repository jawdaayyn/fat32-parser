//! trait pour les dispositifs de stockage
//! 
//! ce module définit le trait BlockDevice qui doit être implémenté
//! par tout dispositif de stockage (disque, image, etc.) pour être
//! utilisé avec le parser FAT32.

use crate::utils::error::Fat32Error;

/// trait pour un dispositif bloc
/// 
/// # Exemples
/// 
/// ```no_run
/// use fat32_parser::block_device::BlockDevice;
/// use fat32_parser::error::Fat32Error;
/// 
/// struct MonDevice;
/// 
/// impl BlockDevice for MonDevice {
///     fn read_sector(&self, sector: u32, buffer: &mut [u8]) -> Result<(), Fat32Error> {
///         // lecture du secteur
///         Ok(())
///     }
///     
///     fn write_sector(&mut self, sector: u32, buffer: &[u8]) -> Result<(), Fat32Error> {
///         // écriture du secteur
///         Ok(())
///     }
/// }
/// ```
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

