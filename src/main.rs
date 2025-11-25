#![no_std]
#![no_main]

//! parser FAT32 en environnement no_std
//! 
//! ce crate fournit les outils pour parser un système de fichiers FAT32
//! sans dépendre de la bibliothèque standard.
//! 
//! # Modules principaux
//! 
//! - [`operations::parser`] : structure principale Fat32Parser
//! - [`structures::boot_sector`] : structure du boot sector
//! - [`structures::dir_entry`] : entrées de répertoire
//! - [`traits::block_device`] : trait pour les dispositifs de stockage

use core::panic::PanicInfo;

// structures de données FAT32
pub mod structures {
    pub mod boot_sector;
    pub mod fsinfo;
    pub mod dir_entry;
}

// opérations sur le système de fichiers
pub mod operations {
    pub mod parser;
    pub mod file_ops;
    pub mod file_info;
}

// traits
pub mod traits {
    pub mod block_device;
}

// utilitaires
pub mod utils {
    pub mod error;
    pub mod constants;
    pub mod fat;
    pub mod helpers;
    pub mod validator;
}

// ré-exports pour compatibilité
pub use structures::boot_sector;
pub use structures::dir_entry;
pub use structures::fsinfo;
pub use operations::file_info;
pub use operations::file_ops;
pub use operations::parser;
pub use traits::block_device;
pub use utils::constants;
pub use utils::error;
pub use utils::fat;
pub use utils::helpers as utils_helpers;
pub use utils::validator;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock_device;

// boucle pour les alertes no_std.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}