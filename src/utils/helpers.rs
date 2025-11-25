//! fonctions utilitaires

/// convertit un nom court FAT en string
pub fn short_name_to_string(name: &[u8; 11]) -> [u8; 12] {
    let mut result = [0u8; 12];
    let mut pos = 0;
    
    // partie nom (8 caractères)
    for i in 0..8 {
        if name[i] != b' ' {
            result[pos] = name[i];
            pos += 1;
        } else {
            break;
        }
    }
    
    // vérifier s'il y a une extension
    let has_ext = name[8] != b' ';
    
    if has_ext {
        result[pos] = b'.';
        pos += 1;
        
        for i in 8..11 {
            if name[i] != b' ' {
                result[pos] = name[i];
                pos += 1;
            }
        }
    }
    
    result
}

/// calcule un checksum pour les entrées LFN
pub fn lfn_checksum(short_name: &[u8; 11]) -> u8 {
    let mut sum: u8 = 0;
    
    for &byte in short_name {
        sum = ((sum & 1) << 7)
            .wrapping_add(sum >> 1)
            .wrapping_add(byte);
    }
    
    sum
}

