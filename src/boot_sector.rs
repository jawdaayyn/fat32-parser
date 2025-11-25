//! Structures pour le Boot Sector FAT32

/// Boot Sector FAT32 (512 octets)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BootSector {
    // Partie commune à toutes les FAT
    pub jmp_boot: [u8; 3],              // Code de saut
    pub oem_name: [u8; 8],              // Nom OEM
    pub bytes_per_sector: u16,          // Octets par secteur (512)
    pub sectors_per_cluster: u8,        // Secteurs par cluster
    pub reserved_sector_count: u16,     // Secteurs réservés
    pub num_fats: u8,                   // Nombre de FAT (2)
    pub root_entry_count: u16,          // Entrées racine (0 pour FAT32)
    pub total_sectors_16: u16,          // Total secteurs 16-bit (0 pour FAT32)
    pub media_type: u8,                 // Type de média
    pub fat_size_16: u16,               // Taille FAT 16-bit (0 pour FAT32)
    pub sectors_per_track: u16,         // Secteurs par piste
    pub num_heads: u16,                 // Nombre de têtes
    pub hidden_sectors: u32,            // Secteurs cachés
    pub total_sectors_32: u32,          // Total secteurs 32-bit
    
    // Partie spécifique FAT32
    pub fat_size_32: u32,               // Taille FAT en secteurs
    pub ext_flags: u16,                 // Flags d'extension
    pub fs_version: u16,                // Version FS
    pub root_cluster: u32,              // Cluster racine
    pub fs_info_sector: u16,            // Secteur FSInfo
    pub backup_boot_sector: u16,        // Secteur backup
    pub reserved: [u8; 12],             // Réservé
    pub drive_number: u8,               // Numéro de drive
    pub reserved1: u8,                  // Réservé
    pub boot_signature: u8,             // Signature boot (0x29)
    pub volume_id: u32,                 // ID du volume
    pub volume_label: [u8; 11],         // Label du volume
    pub fs_type: [u8; 8],               // Type FS "FAT32   "
    pub boot_code: [u8; 420],           // Code de boot
    pub signature: u16,                 // Signature (0xAA55)
}

