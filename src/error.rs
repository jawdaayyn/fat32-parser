//! types d'erreurs
//! 
//! définit les différents types d'erreurs qui peuvent survenir
//! lors de l'utilisation du parser FAT32.

/// erreurs du parser FAT32
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fat32Error {
    /// signature invalide dans le boot sector ou FSInfo
    InvalidSignature,
    /// numéro de secteur invalide
    InvalidSector,
    /// numéro de cluster invalide
    InvalidCluster,
    /// erreur de lecture
    ReadError,
    /// erreur d'écriture
    WriteError,
    /// élément non trouvé
    NotFound,
    /// disque plein
    DiskFull,
    /// élément existe déjà
    AlreadyExists,
}

/// type résultat pour les opérations FAT32
pub type Result<T> = core::result::Result<T, Fat32Error>;

