//! entrées de répertoire
//! 
//! ce module contient la structure DirEntry qui représente une entrée
//! dans un répertoire FAT32 (fichier ou sous-répertoire).

/// entrée de répertoire (32 octets)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct DirEntry {
    pub name: [u8; 11],
    pub attributes: u8,
    pub nt_reserved: u8,
    pub creation_time_tenth: u8,
    pub creation_time: u16,
    pub creation_date: u16,
    pub last_access_date: u16,
    pub first_cluster_high: u16,
    pub last_write_time: u16,
    pub last_write_date: u16,
    pub first_cluster_low: u16,
    pub file_size: u32,
}

// attributs de fichier
pub const ATTR_READ_ONLY: u8 = 0x01;
pub const ATTR_HIDDEN: u8 = 0x02;
pub const ATTR_SYSTEM: u8 = 0x04;
pub const ATTR_VOLUME_ID: u8 = 0x08;
pub const ATTR_DIRECTORY: u8 = 0x10;
pub const ATTR_ARCHIVE: u8 = 0x20;
pub const ATTR_LONG_NAME: u8 = 0x0F;

impl DirEntry {
    pub fn first_cluster(&self) -> u32 {
        ((self.first_cluster_high as u32) << 16) | (self.first_cluster_low as u32)
    }
    
    pub fn is_directory(&self) -> bool {
        self.attributes & ATTR_DIRECTORY != 0
    }
    
    pub fn is_file(&self) -> bool {
        !self.is_directory() && self.attributes & ATTR_VOLUME_ID == 0
    }
    
    pub fn is_empty(&self) -> bool {
        self.name[0] == crate::constants::ENTRY_EMPTY 
            || self.name[0] == crate::constants::ENTRY_DELETED
    }
    
    /// retourne le nom au format 8.3
    pub fn get_name(&self) -> [u8; 11] {
        self.name
    }
    
    /// vérifie si le fichier est en lecture seule
    pub fn is_read_only(&self) -> bool {
        self.attributes & ATTR_READ_ONLY != 0
    }
    
    /// vérifie si le fichier est caché
    pub fn is_hidden(&self) -> bool {
        self.attributes & ATTR_HIDDEN != 0
    }
    
    /// vérifie si c'est un fichier système
    pub fn is_system(&self) -> bool {
        self.attributes & ATTR_SYSTEM != 0
    }
    
    /// vérifie si c'est une entrée de nom long
    pub fn is_long_name(&self) -> bool {
        self.attributes == ATTR_LONG_NAME
    }
}

