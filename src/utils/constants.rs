//! constantes pour FAT32

/// taille standard d'un secteur
pub const SECTOR_SIZE: usize = 512;

/// signature boot sector
pub const BOOT_SIGNATURE: u16 = 0xAA55;

/// signature boot étendue
pub const EXTENDED_BOOT_SIGNATURE: u8 = 0x29;

/// signature FSInfo lead
pub const FSINFO_LEAD_SIG: u32 = 0x41615252;

/// signature FSInfo struct
pub const FSINFO_STRUCT_SIG: u32 = 0x61417272;

/// signature FSInfo trail
pub const FSINFO_TRAIL_SIG: u32 = 0xAA550000;

/// premier cluster valide
pub const FIRST_VALID_CLUSTER: u32 = 2;

/// marqueur d'entrée vide
pub const ENTRY_EMPTY: u8 = 0x00;

/// marqueur d'entrée supprimée
pub const ENTRY_DELETED: u8 = 0xE5;

