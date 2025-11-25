// bibliothèque FAT32 avec support std pour le binaire et les tests

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

// point d'entrée du binaire
#[cfg(not(test))]
fn main() {
    use std::env;
    use std::process;
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <image.fat32>", args[0]);
        eprintln!("   ou: {} generate-img", args[0]);
        process::exit(1);
    }
    
    let arg = &args[1];
    
    // générer une image de test
    if arg == "generate-img" {
        match generate_test_image("test.img") {
            Ok(_) => {
                println!("✓ Image test.img générée avec succès");
                println!("\nTestez avec: cargo run test.img");
            }
            Err(e) => {
                eprintln!("Erreur: {}", e);
                process::exit(1);
            }
        }
        return;
    }
    
    // parser une image existante
    println!("\n=== PARSER FAT32 ===\n");
    println!("Image: {}", arg);
    
    match parse_fat32_image(arg) {
        Ok(_) => {
            println!("\nParsing réussi !");
        }
        Err(e) => {
            eprintln!("\nErreur: {}", e);
            process::exit(1);
        }
    }
}

#[cfg(not(test))]
fn parse_fat32_image(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::os::unix::fs::FileExt;
    use operations::parser::Fat32Parser;
    use traits::block_device::BlockDevice;
    use utils::error::Fat32Error;
    
    // device qui lit depuis un fichier
    struct FileDevice {
        file: File,
    }
    
    impl FileDevice {
        fn new(path: &str) -> std::io::Result<Self> {
            Ok(Self {
                file: File::open(path)?,
            })
        }
    }
    
    impl BlockDevice for FileDevice {
        fn read_sector(&self, sector: u32, buffer: &mut [u8]) -> Result<(), Fat32Error> {
            let offset = sector as u64 * 512;
            self.file.read_exact_at(buffer, offset)
                .map_err(|_| Fat32Error::ReadError)?;
            Ok(())
        }
        
        fn write_sector(&mut self, _sector: u32, _buffer: &[u8]) -> Result<(), Fat32Error> {
            Err(Fat32Error::WriteError) // lecture seule
        }
    }
    
    println!("Ouverture de l'image...\n");
    let device = FileDevice::new(path)?;
    
    println!("Lecture du boot sector...");
    let parser = Fat32Parser::new(device)
        .map_err(|e| format!("Erreur lors du parsing du boot sector: {:?}", e))?;
    
    let boot = parser.boot_sector();
    let signature = boot.signature;
    let bytes_per_sector = boot.bytes_per_sector;
    let sectors_per_cluster = boot.sectors_per_cluster;
    let num_fats = boot.num_fats;
    let fat_size = boot.fat_size_32;
    let total_sectors = boot.total_sectors_32;
    let root_cluster = boot.root_cluster;
    let oem_name = boot.oem_name;
    let volume_label = boot.volume_label;
    
    println!("\nBOOT SECTOR:");
    println!("  ├─ Signature: 0x{:04X} {}", signature, 
        if boot.is_valid() { "✓" } else { "✗" });
    println!("  ├─ OEM: {:?}", std::str::from_utf8(&oem_name).unwrap_or("???"));
    println!("  ├─ Octets/secteur: {}", bytes_per_sector);
    println!("  ├─ Secteurs/cluster: {}", sectors_per_cluster);
    println!("  ├─ Taille cluster: {} octets", boot.cluster_size());
    println!("  ├─ Nombre de FAT: {}", num_fats);
    println!("  ├─ Taille FAT: {} secteurs", fat_size);
    println!("  ├─ Total secteurs: {}", total_sectors);
    println!("  ├─ Cluster racine: {}", root_cluster);
    println!("  └─ Volume: {:?}", 
        std::str::from_utf8(&volume_label).unwrap_or("???").trim());
    
    println!("\nLecture du répertoire racine...");
    let entries = parser.read_root_dir()
        .map_err(|e| format!("Erreur lecture répertoire: {:?}", e))?;
    
    println!("\nCONTENU:\n");
    
    let mut count = 0;
    for entry in entries.iter() {
        if !entry.is_empty() && !entry.is_long_name() {
            count += 1;
            let name = utils::helpers::short_name_to_string(&entry.name);
            let name_str = std::str::from_utf8(&name).unwrap_or("???")
                .trim_end_matches('\0').trim();
            
            if entry.is_directory() {
                println!("{} (cluster: {})", name_str, entry.first_cluster());
            } else {
                let size = entry.file_size;
                println!("{} ({} octets, cluster: {})", 
                    name_str, size, entry.first_cluster());
            }
        }
    }
    
    println!("\n✓ {} éléments trouvés", count);
    
    Ok(())
}

#[cfg(not(test))]
fn generate_test_image(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs::File;
    use std::io::Write;
    use structures::boot_sector::BootSector;
    use structures::dir_entry::DirEntry;
    
    println!("Génération de l'image FAT32...\n");
    
    // créer boot sector
    let boot_sector = BootSector {
        jmp_boot: [0xEB, 0x58, 0x90],
        oem_name: *b"MSWIN4.1",
        bytes_per_sector: 512,
        sectors_per_cluster: 8,
        reserved_sector_count: 32,
        num_fats: 2,
        root_entry_count: 0,
        total_sectors_16: 0,
        media_type: 0xF8,
        fat_size_16: 0,
        sectors_per_track: 63,
        num_heads: 255,
        hidden_sectors: 0,
        total_sectors_32: 1024000,
        fat_size_32: 1000,
        ext_flags: 0,
        fs_version: 0,
        root_cluster: 2,
        fs_info_sector: 1,
        backup_boot_sector: 6,
        reserved: [0; 12],
        drive_number: 0x80,
        reserved1: 0,
        boot_signature: 0x29,
        volume_id: 0x12345678,
        volume_label: *b"TEST VOL   ",
        fs_type: *b"FAT32   ",
        boot_code: [0; 420],
        signature: 0xAA55,
    };
    
    // fichiers de test
    let file1 = DirEntry {
        name: *b"README  TXT",
        attributes: 0x20,
        nt_reserved: 0,
        creation_time_tenth: 0,
        creation_time: 0,
        creation_date: 0,
        last_access_date: 0,
        first_cluster_high: 0,
        last_write_time: 0,
        last_write_date: 0,
        first_cluster_low: 3,
        file_size: 1024,
    };
    
    let file2 = DirEntry {
        name: *b"DATA    BIN",
        attributes: 0x20,
        nt_reserved: 0,
        creation_time_tenth: 0,
        creation_time: 0,
        creation_date: 0,
        last_access_date: 0,
        first_cluster_high: 0,
        last_write_time: 0,
        last_write_date: 0,
        first_cluster_low: 4,
        file_size: 2048,
    };
    
    let dir1 = DirEntry {
        name: *b"DOCS       ",
        attributes: 0x10,
        nt_reserved: 0,
        creation_time_tenth: 0,
        creation_time: 0,
        creation_date: 0,
        last_access_date: 0,
        first_cluster_high: 0,
        last_write_time: 0,
        last_write_date: 0,
        first_cluster_low: 5,
        file_size: 0,
    };
    
    let mut file = File::create(path)?;
    
    // écrire boot sector
    let bs_bytes: [u8; 512] = unsafe {
        core::ptr::read(&boot_sector as *const BootSector as *const [u8; 512])
    };
    file.write_all(&bs_bytes)?;
    
    // remplir jusqu'au cluster racine (secteur 2032)
    let first_sector = boot_sector.cluster_to_sector(2);
    let zeros = [0u8; 512];
    for _ in 1..first_sector {
        file.write_all(&zeros)?;
    }
    
    // écrire les entrées dans le cluster racine
    let mut cluster_data = [0u8; 4096]; // 8 secteurs
    unsafe {
        let e1 = core::ptr::read(&file1 as *const DirEntry as *const [u8; 32]);
        let e2 = core::ptr::read(&file2 as *const DirEntry as *const [u8; 32]);
        let e3 = core::ptr::read(&dir1 as *const DirEntry as *const [u8; 32]);
        cluster_data[0..32].copy_from_slice(&e1);
        cluster_data[32..64].copy_from_slice(&e2);
        cluster_data[64..96].copy_from_slice(&e3);
    }
    file.write_all(&cluster_data)?;
    
    println!("✓ Boot sector écrit");
    println!("✓ Fichiers créés:");
    println!("  - README.TXT (1024 octets)");
    println!("  - DATA.BIN (2048 octets)");
    println!("  - DOCS/ (répertoire)\n");
    
    Ok(())
}