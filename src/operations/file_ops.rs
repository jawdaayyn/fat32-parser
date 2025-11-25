//! opérations sur les fichiers
//! 
//! fonctions utilitaires pour créer et manipuler des fichiers
//! et des répertoires.

use crate::structures::dir_entry::{DirEntry, ATTR_DIRECTORY, ATTR_ARCHIVE};

/// crée une nouvelle entrée de fichier
/// 
/// # Arguments
/// 
/// * `name` - nom au format 8.3
/// * `cluster` - premier cluster du fichier
/// * `size` - taille du fichier en octets
pub fn create_file_entry(name: [u8; 11], cluster: u32, size: u32) -> DirEntry {
    DirEntry {
        name,
        attributes: ATTR_ARCHIVE,
        nt_reserved: 0,
        creation_time_tenth: 0,
        creation_time: 0,
        creation_date: 0,
        last_access_date: 0,
        first_cluster_high: (cluster >> 16) as u16,
        last_write_time: 0,
        last_write_date: 0,
        first_cluster_low: (cluster & 0xFFFF) as u16,
        file_size: size,
    }
}

/// crée une nouvelle entrée de répertoire
/// 
/// # Arguments
/// 
/// * `name` - nom au format 8.3
/// * `cluster` - premier cluster du répertoire
pub fn create_dir_entry(name: [u8; 11], cluster: u32) -> DirEntry {
    DirEntry {
        name,
        attributes: ATTR_DIRECTORY,
        nt_reserved: 0,
        creation_time_tenth: 0,
        creation_time: 0,
        creation_date: 0,
        last_access_date: 0,
        first_cluster_high: (cluster >> 16) as u16,
        last_write_time: 0,
        last_write_date: 0,
        first_cluster_low: (cluster & 0xFFFF) as u16,
        file_size: 0,
    }
}

/// convertit un nom court en format 8.3
pub fn format_short_name(name: &str) -> [u8; 11] {
    let mut result = [b' '; 11];
    let parts: Vec<&str> = name.split('.').collect();
    
    if parts.len() == 2 {
        let basename = parts[0].as_bytes();
        let ext = parts[1].as_bytes();
        
        for (i, &byte) in basename.iter().take(8).enumerate() {
            result[i] = byte.to_ascii_uppercase();
        }
        
        for (i, &byte) in ext.iter().take(3).enumerate() {
            result[8 + i] = byte.to_ascii_uppercase();
        }
    } else {
        for (i, &byte) in name.as_bytes().iter().take(8).enumerate() {
            result[i] = byte.to_ascii_uppercase();
        }
    }
    
    result
}

/// vérifie si un nom est valide pour FAT32
pub fn is_valid_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 12 {
        return false;
    }
    
    for c in name.chars() {
        if !c.is_ascii_alphanumeric() && c != '.' && c != '_' && c != '-' {
            return false;
        }
    }
    
    true
}

/// compare deux noms courts
pub fn names_match(name1: &[u8; 11], name2: &[u8; 11]) -> bool {
    name1 == name2
}

