//! informations sur les fichiers

/// informations sur un fichier ou répertoire
#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    pub name: [u8; 11],
    pub is_directory: bool,
    pub is_read_only: bool,
    pub is_hidden: bool,
    pub is_system: bool,
    pub first_cluster: u32,
    pub size: u32,
}

impl FileInfo {
    /// crée depuis une DirEntry
    pub fn from_dir_entry(entry: &crate::structures::dir_entry::DirEntry) -> Self {
        Self {
            name: entry.name,
            is_directory: entry.is_directory(),
            is_read_only: entry.is_read_only(),
            is_hidden: entry.is_hidden(),
            is_system: entry.is_system(),
            first_cluster: entry.first_cluster(),
            size: entry.file_size,
        }
    }
}

