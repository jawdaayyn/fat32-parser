//! fonctions de validation

use crate::boot_sector::BootSector;
use crate::constants::*;

/// valide un boot sector FAT32
pub fn validate_boot_sector(bs: &BootSector) -> bool {
    // vérifier signature
    if !bs.is_valid() {
        return false;
    }
    
    // vérifier bytes par secteur (doit être 512)
    if bs.bytes_per_sector != SECTOR_SIZE as u16 {
        return false;
    }
    
    // vérifier secteurs par cluster (doit être puissance de 2)
    let spc = bs.sectors_per_cluster;
    if spc == 0 || (spc & (spc - 1)) != 0 {
        return false;
    }
    
    // vérifier nombre de FAT (généralement 2)
    if bs.num_fats == 0 || bs.num_fats > 2 {
        return false;
    }
    
    // pour FAT32, root_entry_count doit être 0
    if bs.root_entry_count != 0 {
        return false;
    }
    
    // pour FAT32, total_sectors_16 doit être 0
    if bs.total_sectors_16 != 0 {
        return false;
    }
    
    // pour FAT32, fat_size_16 doit être 0
    if bs.fat_size_16 != 0 {
        return false;
    }
    
    // vérifier que fat_size_32 est non nul
    if bs.fat_size_32 == 0 {
        return false;
    }
    
    // cluster racine doit être >= 2
    if bs.root_cluster < FIRST_VALID_CLUSTER {
        return false;
    }
    
    true
}

/// vérifie si un cluster est valide
pub fn is_valid_cluster(cluster: u32) -> bool {
    cluster >= FIRST_VALID_CLUSTER && cluster < 0x0FFFFFF8
}

