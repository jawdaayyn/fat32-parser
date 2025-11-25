# Architecture du projet

## Structure des modules

```
fat32-parser/
├── src/
│   ├── main.rs              # point d'entrée et documentation principale
│   ├── boot_sector.rs       # structure Boot Sector FAT32
│   ├── fsinfo.rs            # structure FSInfo
│   ├── dir_entry.rs         # entrées de répertoire
│   ├── fat.rs               # constantes et fonctions FAT
│   ├── parser.rs            # parser principal
│   ├── block_device.rs      # trait BlockDevice
│   ├── file_ops.rs          # opérations fichiers
│   ├── file_info.rs         # informations fichiers
│   ├── error.rs             # types d'erreurs
│   ├── constants.rs         # constantes FAT32
│   ├── utils.rs             # utilitaires
│   ├── validator.rs         # validation
│   ├── tests.rs             # tests unitaires
│   └── mock_device.rs       # mock pour tests
```

## Dépendances entre modules

```
main.rs
  ├── boot_sector
  ├── fsinfo
  ├── dir_entry
  │   └── constants
  ├── fat
  ├── error
  ├── block_device
  │   └── error
  ├── parser
  │   ├── boot_sector
  │   ├── block_device
  │   ├── dir_entry
  │   ├── error
  │   ├── fat
  │   ├── file_info
  │   ├── file_ops
  │   └── fsinfo
  ├── file_ops
  │   └── dir_entry
  ├── file_info
  │   └── dir_entry
  ├── constants
  ├── utils
  └── validator
      ├── boot_sector
      └── constants
```

## Flux de données

### Lecture d'un fichier

1. `Fat32Parser::new()` lit le boot sector (secteur 0)
2. `load_fsinfo()` lit FSInfo (secteur 1)
3. `read_root_dir()` lit le répertoire racine
4. `read_fat_entry()` lit une entrée de la FAT
5. `read_cluster()` lit un cluster de données
6. `read_file()` suit la chaîne et lit tout le fichier

### Écriture d'un fichier

1. `find_free_cluster()` trouve un cluster libre
2. `allocate_cluster()` alloue le cluster dans la FAT
3. `write_cluster()` écrit les données
4. `write_fat_entry()` met à jour la FAT
5. Répéter pour chaque cluster

## Structures principales

### BootSector (512 octets)
- Informations sur le volume
- Géométrie du disque
- Localisation de la FAT et des données

### FSInfo (512 octets)
- Nombre de clusters libres
- Prochain cluster libre

### DirEntry (32 octets)
- Nom du fichier (format 8.3)
- Attributs
- Premier cluster
- Taille

### Fat32Parser<D>
- Device de stockage (D: BlockDevice)
- Boot sector chargé
- FSInfo optionnel

## Sécurité

### Fonctions unsafe

Toutes les fonctions unsafe sont documentées :
- `BootSector::from_bytes()` - lecture depuis pointeur non aligné
- `FSInfo::from_bytes()` - lecture depuis pointeur non aligné
- Structures `#[repr(C, packed)]` - correspondance exacte avec format disque

### Gestion des erreurs

Type `Result<T>` personnalisé avec `Fat32Error`:
- InvalidSignature
- InvalidSector
- InvalidCluster
- ReadError
- WriteError
- NotFound
- DiskFull
- AlreadyExists

## Tests

### Tests unitaires (tests.rs)
- Tests des structures
- Tests des fonctions FAT
- Tests des attributs
- Tests de validation

### Mock Device (mock_device.rs)
- Implémentation en mémoire
- Pour tests d'intégration
- Buffer de 512 KB

## Performance

### Optimisations
- Structures packed pour économie mémoire
- Pas d'allocation dynamique (no_std)
- Lecture/écriture directe par secteur
- Cache optionnel pour la FAT (à implémenter)

### Limitations actuelles
- Buffer fixe pour read_file (4096 octets)
- Pas de cache FAT
- Pas de support noms longs (LFN)
- Lecture séquentielle des clusters

