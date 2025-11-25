// valeurs pour fat
pub const FAT_FREE: u32 = 0x00000000;
pub const FAT_BAD: u32 = 0x0FFFFFF7;
pub const FAT_EOC: u32 = 0x0FFFFFF8;

/// vérifie si un cluster est la fin de chaîne
pub fn is_eoc(cluster: u32) -> bool {
    cluster >= FAT_EOC
}

