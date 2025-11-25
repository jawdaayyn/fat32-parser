//! boot sector
//! 
//! contient la structure BootSector qui représente le premier secteur
//! d'un volume FAT32. ce secteur contient toutes les informations
//! nécessaires pour accéder au système de fichiers.

/// boot sector (512 octets)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BootSector {
    pub jmp_boot: [u8; 3],
    pub oem_name: [u8; 8],
    pub bytes_per_sector: u16,
    pub sectors_per_cluster: u8,
    pub reserved_sector_count: u16,
    pub num_fats: u8,
    pub root_entry_count: u16,
    pub total_sectors_16: u16,
    pub media_type: u8,
    pub fat_size_16: u16,
    pub sectors_per_track: u16,
    pub num_heads: u16,
    pub hidden_sectors: u32,
    pub total_sectors_32: u32,
    
    pub fat_size_32: u32,
    pub ext_flags: u16,
    pub fs_version: u16,
    pub root_cluster: u32,
    pub fs_info_sector: u16,
    pub backup_boot_sector: u16,
    pub reserved: [u8; 12],
    pub drive_number: u8,
    pub reserved1: u8,
    pub boot_signature: u8,
    pub volume_id: u32,
    pub volume_label: [u8; 11],
    pub fs_type: [u8; 8],
    pub boot_code: [u8; 420],
    pub signature: u16,
}

impl BootSector {
    /// # Safety
    /// buffer doit être 512 octets valides
    pub unsafe fn from_bytes(data: &[u8; 512]) -> Self {
        core::ptr::read_unaligned(data.as_ptr() as *const BootSector)
    }
    
    pub fn is_valid(&self) -> bool {
        self.signature == crate::constants::BOOT_SIGNATURE
    }
    
    /// retourne la taille d'un cluster en octets
    pub fn cluster_size(&self) -> u32 {
        self.bytes_per_sector as u32 * self.sectors_per_cluster as u32
    }
    
    /// retourne le secteur de début de la première FAT
    pub fn fat_start_sector(&self) -> u32 {
        self.reserved_sector_count as u32
    }
    
    /// retourne le secteur de début de la zone de données
    pub fn data_start_sector(&self) -> u32 {
        self.reserved_sector_count as u32 + (self.num_fats as u32 * self.fat_size_32)
    }
    
    /// convertit un numéro de cluster en secteur
    pub fn cluster_to_sector(&self, cluster: u32) -> u32 {
        self.data_start_sector() + ((cluster - 2) * self.sectors_per_cluster as u32)
    }
    
    /// retourne le nombre total de secteurs
    pub fn total_sectors(&self) -> u32 {
        if self.total_sectors_32 != 0 {
            self.total_sectors_32
        } else {
            self.total_sectors_16 as u32
        }
    }
    
    /// retourne la taille de la FAT en secteurs
    pub fn fat_size(&self) -> u32 {
        if self.fat_size_32 != 0 {
            self.fat_size_32
        } else {
            self.fat_size_16 as u32
        }
    }
}

