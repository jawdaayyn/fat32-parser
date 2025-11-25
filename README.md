# Parser FAT32

Parser FAT32 en environnement `no_std` pour systèmes embarqués.

## Fonctionnalités

- Lecture du Boot Sector et FSInfo
- Lecture des entrées de répertoire
- Navigation dans la FAT (File Allocation Table)
- Lecture et écriture de clusters
- Création d'entrées de fichiers et répertoires
- Gestion de l'allocation de clusters
- Libération de chaînes de clusters

## Architecture

### Modules principaux

- `boot_sector` : structure du boot sector FAT32
- `fsinfo` : structure FSInfo pour les métadonnées
- `dir_entry` : entrées de répertoire (fichiers/dossiers)
- `fat` : constantes et fonctions pour la FAT
- `parser` : structure principale `Fat32Parser`
- `block_device` : trait pour les dispositifs de stockage
- `file_ops` : opérations sur les fichiers
- `file_info` : informations sur les fichiers

## Utilisation

```rust
use fat32_parser::parser::Fat32Parser;
use fat32_parser::block_device::BlockDevice;

// implémenter BlockDevice pour votre dispositif
let parser = Fat32Parser::new(mon_device)?;

// charger FSInfo
parser.load_fsinfo()?;

// lire le répertoire racine
let files = parser.list_root_files()?;
```

## Tests

Le projet inclut des tests unitaires complets :

```bash
cargo test
```

Tests disponibles :
- Tests unitaires pour toutes les structures
- Tests des fonctions FAT (is_eoc, is_free, is_bad, mask_cluster)
- Tests des attributs de fichiers
- Tests de création d'entrées
- Tests de validation de noms
- Tests du MockDevice
- Tests de validation du Boot Sector
- Tests des utilitaires

Pour exécuter avec verbose :
```bash
cargo test -- --nocapture
```

## Sécurité

Les fonctions unsafe sont documentées avec la section `# Safety` de rustdoc.
Les structures packed utilisent `ptr::read_unaligned` pour éviter les problèmes d'alignement.

## Environnement no_std

Ce crate fonctionne sans la bibliothèque standard Rust, adapté aux systèmes embarqués.

