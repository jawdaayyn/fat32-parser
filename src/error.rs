//! types d'erreurs

/// erreurs du parser FAT32
#[derive(Debug, Clone, Copy)]
pub enum Fat32Error {
    InvalidSignature,
    InvalidSector,
    InvalidCluster,
}

