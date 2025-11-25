//! types d'erreurs

/// erreurs du parser FAT32
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fat32Error {
    InvalidSignature,
    InvalidSector,
    InvalidCluster,
    ReadError,
    WriteError,
    NotFound,
    DiskFull,
    AlreadyExists,
}

