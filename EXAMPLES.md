# Exemples d'utilisation

## Créer un parser FAT32

```rust
use fat32_parser::parser::Fat32Parser;
use fat32_parser::block_device::BlockDevice;

// avec votre implémentation de BlockDevice
let mut parser = Fat32Parser::new(mon_device)?;
```

## Lire le Boot Sector

```rust
let boot_sector = parser.boot_sector();
println!("Cluster size: {}", boot_sector.cluster_size());
println!("FAT start: {}", boot_sector.fat_start_sector());
```

## Charger FSInfo

```rust
parser.load_fsinfo()?;

if let Some(fsinfo) = parser.fsinfo() {
    if let Some(free) = fsinfo.free_clusters() {
        println!("Clusters libres: {}", free);
    }
}
```

## Lister les fichiers

```rust
let files = parser.list_root_files()?;

for file_opt in files.iter() {
    if let Some(file) = file_opt {
        let name = crate::utils::short_name_to_string(&file.name);
        println!("Fichier: {:?}, Taille: {}", name, file.size);
    }
}
```

## Lire un fichier

```rust
let cluster = 2; // premier cluster du fichier
let mut buffer = [0u8; 4096];

let bytes_read = parser.read_file(cluster, &mut buffer)?;
println!("Lu {} octets", bytes_read);
```

## Écrire dans un fichier

```rust
let data = b"Hello FAT32!";
parser.write_file(start_cluster, data)?;
```

## Allouer un nouveau cluster

```rust
// allouer un cluster libre
let new_cluster = parser.allocate_cluster(None)?;

// ou allouer et lier à un cluster existant
let next_cluster = parser.allocate_cluster(Some(current_cluster))?;
```

## Libérer des clusters

```rust
// libérer un cluster
parser.free_cluster(cluster)?;

// libérer toute une chaîne
parser.free_cluster_chain(start_cluster)?;
```

## Créer une entrée de fichier

```rust
use fat32_parser::file_ops::{create_file_entry, format_short_name};

let name = format_short_name("test.txt");
let entry = create_file_entry(name, cluster, size);
```

## Valider un Boot Sector

```rust
use fat32_parser::validator::validate_boot_sector;

if validate_boot_sector(boot_sector) {
    println!("Boot sector valide");
}
```

