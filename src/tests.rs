//! test d'intégration FAT32

#[cfg(test)]
mod tests {
    // pour les tests on a accès à println!
    extern crate std;
    use crate::structures::boot_sector::BootSector;
    use crate::structures::dir_entry::*;
    use crate::operations::file_ops::*;
    use crate::utils::helpers::*;
    
    #[test]
    fn test_integration_fat32_complet() {
        use crate::mock_device::MockDevice;
        use crate::operations::parser::Fat32Parser;
        use crate::traits::block_device::BlockDevice;
        
        std::println!("\n=== TEST D'INTÉGRATION FAT32 ===\n");
        
        // créer un device mock
        let mut device = MockDevice::new();
        
        // créer un boot sector valide
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
        
        // écrire le boot sector au secteur 0
        let bs_bytes: [u8; 512] = unsafe {
            core::ptr::read(&boot_sector as *const BootSector as *const [u8; 512])
        };
        device.write_sector(0, &bs_bytes).unwrap();
        
        let root_cluster = boot_sector.root_cluster;
        
        std::println!("✓ Boot sector créé");
        std::println!("  - Volume: TEST VOL");
        std::println!("  - Cluster size: {} octets", boot_sector.cluster_size());
        std::println!("  - Root cluster: {}", root_cluster);
        
        // créer quelques entrées de répertoire dans la racine (cluster 2)
        let file1 = create_file_entry(format_short_name("readme.txt"), 3, 1024);
        let file2 = create_file_entry(format_short_name("data.bin"), 4, 2048);
        let dir1 = create_dir_entry(format_short_name("docs"), 5);
        
        // écrire les entrées dans le cluster 2 (secteur 2032)
        let first_sector = boot_sector.cluster_to_sector(2);
        let mut cluster_data = [0u8; 4096];
        
        // copier les entrées
        unsafe {
            let entry_bytes1 = core::ptr::read(&file1 as *const DirEntry as *const [u8; 32]);
            let entry_bytes2 = core::ptr::read(&file2 as *const DirEntry as *const [u8; 32]);
            let entry_bytes3 = core::ptr::read(&dir1 as *const DirEntry as *const [u8; 32]);
            
            cluster_data[0..32].copy_from_slice(&entry_bytes1);
            cluster_data[32..64].copy_from_slice(&entry_bytes2);
            cluster_data[64..96].copy_from_slice(&entry_bytes3);
        }
        
        // écrire le cluster
        for i in 0..8 {
            let offset = (i * 512) as usize;
            device.write_sector(first_sector + i, &cluster_data[offset..offset + 512]).unwrap();
        }
        
        std::println!("\n✓ Fichiers et dossiers créés:");
        std::println!("  - readme.txt (1024 octets, cluster 3)");
        std::println!("  - data.bin (2048 octets, cluster 4)");
        std::println!("  - docs/ (dossier, cluster 5)");
        
        // maintenant parser le FAT32
        std::println!("\n=== PARSING DU VOLUME ===\n");
        
        std::println!("📖 Lecture du secteur 0 (Boot Sector)...");
        let parser = Fat32Parser::new(device).unwrap();
        
        std::println!("✓ Parser initialisé");
        std::println!("✓ Boot sector décodé depuis les octets bruts");
        
        let boot = parser.boot_sector();
        let signature = boot.signature;
        let bytes_per_sector = boot.bytes_per_sector;
        let sectors_per_cluster = boot.sectors_per_cluster;
        let reserved_sector_count = boot.reserved_sector_count;
        let num_fats = boot.num_fats;
        let fat_size = boot.fat_size_32;
        let total_sectors = boot.total_sectors_32;
        let root_cluster = boot.root_cluster;
        let oem_name = boot.oem_name;
        let volume_label = boot.volume_label;
        
        std::println!("\n📋 DÉTAILS DU BOOT SECTOR PARSÉ:");
        std::println!("  ├─ Signature: 0x{:04X} (valide: {})", signature, boot.is_valid());
        std::println!("  ├─ OEM Name: {:?}", core::str::from_utf8(&oem_name).unwrap_or("???"));
        std::println!("  ├─ Octets/secteur: {}", bytes_per_sector);
        std::println!("  ├─ Secteurs/cluster: {}", sectors_per_cluster);
        std::println!("  ├─ Taille cluster: {} octets", boot.cluster_size());
        std::println!("  ├─ Secteurs réservés: {}", reserved_sector_count);
        std::println!("  ├─ Nombre de FAT: {}", num_fats);
        std::println!("  ├─ Taille d'une FAT: {} secteurs", fat_size);
        std::println!("  ├─ Total secteurs: {}", total_sectors);
        std::println!("  ├─ Cluster racine: {}", root_cluster);
        std::println!("  ├─ Secteur début FAT: {}", boot.fat_start_sector());
        std::println!("  ├─ Secteur début données: {}", boot.data_start_sector());
        std::println!("  ├─ Secteur du cluster 2: {}", boot.cluster_to_sector(2));
        std::println!("  └─ Volume label: {:?}", core::str::from_utf8(&volume_label).unwrap_or("???").trim());
        
        // lire le répertoire racine
        std::println!("\n📖 Lecture du cluster {} (répertoire racine)...", root_cluster);
        std::println!("   Secteur de début: {}", boot.cluster_to_sector(root_cluster));
        std::println!("   Nombre de secteurs à lire: {}", sectors_per_cluster);
        
        let entries = parser.read_root_dir().unwrap();
        
        std::println!("✓ {} octets lus et décodés en entrées de répertoire", boot.cluster_size());
        std::println!("\n📁 CONTENU DU RÉPERTOIRE RACINE (PARSÉ):\n");
        
        let mut count = 0;
        for (idx, entry) in entries.iter().enumerate() {
            if !entry.is_empty() && !entry.is_long_name() {
                count += 1;
                let name = short_name_to_string(&entry.name);
                let name_str = core::str::from_utf8(&name).unwrap_or("???");
                let entry_name = entry.name;
                let attributes = entry.attributes;
                let cluster_high = entry.first_cluster_high;
                let cluster_low = entry.first_cluster_low;
                let full_cluster = entry.first_cluster();
                
                std::println!("  Entrée #{} (offset 0x{:04X}):", idx, idx * 32);
                std::println!("  ├─ Nom court: {:?}", name_str.trim_end_matches('\0').trim());
                std::println!("  ├─ Nom raw: {:?}", &entry_name);
                std::println!("  ├─ Attributs: 0x{:02X} ({})", attributes,
                    if entry.is_directory() { "DIRECTORY" }
                    else if entry.is_read_only() { "READ_ONLY" }
                    else { "ARCHIVE" });
                
                if entry.is_directory() {
                    std::println!("  ├─ Type: 📁 DOSSIER");
                } else {
                    let size = entry.file_size;
                    std::println!("  ├─ Type: 📄 FICHIER");
                    std::println!("  ├─ Taille: {} octets (0x{:08X})", size, size);
                }
                
                std::println!("  ├─ Premier cluster (high): 0x{:04X}", cluster_high);
                std::println!("  ├─ Premier cluster (low): 0x{:04X}", cluster_low);
                std::println!("  ├─ Premier cluster (32-bit): {} (0x{:08X})", full_cluster, full_cluster);
                std::println!("  └─ Secteur physique: {}\n", boot.cluster_to_sector(full_cluster));
            }
        }
        
        std::println!("✅ RÉSULTAT: {} éléments trouvés et parsés avec succès", count);
        std::println!("\n=== TEST RÉUSSI ===\n");
        
        assert_eq!(count, 3);
    }
}

