#![no_std]
#![no_main]

//! parser FAT32 en environnement no_std
//! 
//! ce crate fournit les outils pour parser un système de fichiers FAT32
//! sans dépendre de la bibliothèque standard.
//! 
//! # Modules principaux
//! 
//! - [`parser`] : structure principale Fat32Parser
//! - [`boot_sector`] : structure du boot sector
//! - [`dir_entry`] : entrées de répertoire
//! - [`block_device`] : trait pour les dispositifs de stockage

use core::panic::PanicInfo;

// modules du parser FAT32
pub mod boot_sector;
pub mod fsinfo;
pub mod dir_entry;
pub mod fat;
pub mod error;
pub mod block_device;
pub mod parser;
pub mod file_ops;
pub mod file_info;
pub mod constants;
pub mod utils;
pub mod validator;

#[cfg(test)]
mod tests;
#[cfg(test)]
mod mock_device;

// boucle pour les alertes no_std.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}