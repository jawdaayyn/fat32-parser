//! structures pour le boot sector FAT32

/// boot sector FAT32 (512 octets)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BootSector {
    // partie commune à toutes les FAT
    pub jmp_boot: [u8; 3],              // code de saut
    pub oem_name: [u8; 8],              // nom OEM
    pub bytes_per_sector: u16,          // octets par secteur (512)
    pub sectors_per_cluster: u8,        // secteurs par cluster
    pub reserved_sector_count: u16,     // secteurs réservés
    pub num_fats: u8,                   // nombre de FAT (2)
    pub root_entry_count: u16,          // entrées racine (0 pour FAT32)
    pub total_sectors_16: u16,          // total secteurs 16-bit (0 pour FAT32)
    pub media_type: u8,                 // type de média
    pub fat_size_16: u16,               // taille FAT 16-bit (0 pour FAT32)
    pub sectors_per_track: u16,         // secteurs par piste
    pub num_heads: u16,                 // nombre de têtes
    pub hidden_sectors: u32,            // secteurs cachés
    pub total_sectors_32: u32,          // total secteurs 32-bit
    
    // partie spécifique FAT32
    pub fat_size_32: u32,               // taille FAT en secteurs
    pub ext_flags: u16,                 // flags d'extension
    pub fs_version: u16,                // version FS
    pub root_cluster: u32,              // cluster racine
    pub fs_info_sector: u16,            // secteur FSInfo
    pub backup_boot_sector: u16,        // secteur backup
    pub reserved: [u8; 12],             // réservé
    pub drive_number: u8,               // numéro de drive
    pub reserved1: u8,                  // réservé
    pub boot_signature: u8,             // signature boot (0x29)
    pub volume_id: u32,                 // ID du volume
    pub volume_label: [u8; 11],         // label du volume
    pub fs_type: [u8; 8],               // type FS "FAT32   "
    pub boot_code: [u8; 420],           // code de boot
    pub signature: u16,                 // signature (0xAA55)
}

