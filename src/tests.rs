//! tests unitaires

#[cfg(test)]
mod tests {
    use crate::boot_sector::BootSector;
    use crate::fat::*;
    use crate::dir_entry::*;
    use crate::file_ops::*;

    #[test]
    fn test_fat_is_eoc() {
        assert!(is_eoc(0x0FFFFFF8));
        assert!(is_eoc(0x0FFFFFFF));
        assert!(!is_eoc(0x00000002));
    }

    #[test]
    fn test_fat_is_free() {
        assert!(is_free(0x00000000));
        assert!(!is_free(0x00000002));
    }

    #[test]
    fn test_fat_is_bad() {
        assert!(is_bad(0x0FFFFFF7));
        assert!(!is_bad(0x00000002));
    }

    #[test]
    fn test_boot_sector_size() {
        assert_eq!(core::mem::size_of::<BootSector>(), 512);
    }

    #[test]
    fn test_dir_entry_size() {
        assert_eq!(core::mem::size_of::<DirEntry>(), 32);
    }

    #[test]
    fn test_dir_entry_attributes() {
        let mut entry = DirEntry {
            name: [0; 11],
            attributes: ATTR_READ_ONLY,
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            first_cluster_high: 0,
            last_write_time: 0,
            last_write_date: 0,
            first_cluster_low: 0,
            file_size: 0,
        };
        
        assert!(entry.is_read_only());
        assert!(!entry.is_directory());
        
        entry.attributes = ATTR_DIRECTORY;
        assert!(entry.is_directory());
        assert!(!entry.is_read_only());
    }

    #[test]
    fn test_dir_entry_cluster() {
        let entry = DirEntry {
            name: [0; 11],
            attributes: 0,
            nt_reserved: 0,
            creation_time_tenth: 0,
            creation_time: 0,
            creation_date: 0,
            last_access_date: 0,
            first_cluster_high: 0x0001,
            last_write_time: 0,
            last_write_date: 0,
            first_cluster_low: 0x0002,
            file_size: 0,
        };
        
        assert_eq!(entry.first_cluster(), 0x00010002);
    }
    
    #[test]
    fn test_boot_sector_cluster_size() {
        let bs = BootSector {
            jmp_boot: [0; 3],
            oem_name: [0; 8],
            bytes_per_sector: 512,
            sectors_per_cluster: 8,
            reserved_sector_count: 32,
            num_fats: 2,
            root_entry_count: 0,
            total_sectors_16: 0,
            media_type: 0xF8,
            fat_size_16: 0,
            sectors_per_track: 0,
            num_heads: 0,
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
            volume_id: 0,
            volume_label: [0; 11],
            fs_type: [0; 8],
            boot_code: [0; 420],
            signature: 0xAA55,
        };
        
        assert_eq!(bs.cluster_size(), 4096);
        assert!(bs.is_valid());
    }
    
    #[test]
    fn test_boot_sector_calculations() {
        let bs = BootSector {
            jmp_boot: [0; 3],
            oem_name: [0; 8],
            bytes_per_sector: 512,
            sectors_per_cluster: 8,
            reserved_sector_count: 32,
            num_fats: 2,
            root_entry_count: 0,
            total_sectors_16: 0,
            media_type: 0xF8,
            fat_size_16: 0,
            sectors_per_track: 0,
            num_heads: 0,
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
            volume_id: 0,
            volume_label: [0; 11],
            fs_type: [0; 8],
            boot_code: [0; 420],
            signature: 0xAA55,
        };
        
        assert_eq!(bs.fat_start_sector(), 32);
        assert_eq!(bs.data_start_sector(), 2032);
        assert_eq!(bs.cluster_to_sector(2), 2032);
        assert_eq!(bs.cluster_to_sector(3), 2040);
    }
    
    #[test]
    fn test_format_short_name() {
        let name1 = format_short_name("test.txt");
        assert_eq!(&name1[0..4], b"TEST");
        assert_eq!(&name1[8..11], b"TXT");
        
        let name2 = format_short_name("readme");
        assert_eq!(&name2[0..6], b"README");
    }
    
    #[test]
    fn test_create_file_entry() {
        let name = format_short_name("test.txt");
        let entry = create_file_entry(name, 0x12345, 1024);
        
        assert_eq!(entry.first_cluster(), 0x12345);
        assert_eq!(entry.file_size, 1024);
        assert!(entry.is_file());
        assert!(!entry.is_directory());
    }
    
    #[test]
    fn test_create_dir_entry() {
        let name = format_short_name("mydir");
        let entry = create_dir_entry(name, 0x100);
        
        assert_eq!(entry.first_cluster(), 0x100);
        assert_eq!(entry.file_size, 0);
        assert!(entry.is_directory());
        assert!(!entry.is_file());
    }
    
    #[test]
    fn test_is_valid_name() {
        assert!(is_valid_name("test.txt"));
        assert!(is_valid_name("readme"));
        assert!(is_valid_name("a.b"));
        assert!(!is_valid_name(""));
        assert!(!is_valid_name("verylongname.txt"));
        assert!(!is_valid_name("bad*name"));
    }
    
    #[test]
    fn test_names_match() {
        let name1 = format_short_name("test.txt");
        let name2 = format_short_name("test.txt");
        let name3 = format_short_name("other.txt");
        
        assert!(names_match(&name1, &name2));
        assert!(!names_match(&name1, &name3));
    }
}

