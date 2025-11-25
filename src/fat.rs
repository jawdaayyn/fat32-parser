//! constantes et fonctions pour la table FAT
//! 
//! la table FAT (File Allocation Table) stocke la chaîne des clusters
//! pour chaque fichier.

// valeurs pour fat
pub const FAT_FREE: u32 = 0x00000000;
pub const FAT_BAD: u32 = 0x0FFFFFF7;
pub const FAT_EOC: u32 = 0x0FFFFFF8;

/// vérifie si un cluster est la fin de chaîne
pub fn is_eoc(cluster: u32) -> bool {
    cluster >= FAT_EOC
}

/// vérifie si un cluster est libre
pub fn is_free(cluster: u32) -> bool {
    cluster == FAT_FREE
}

/// vérifie si un cluster est défectueux
pub fn is_bad(cluster: u32) -> bool {
    cluster == FAT_BAD
}

