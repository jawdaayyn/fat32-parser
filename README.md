# fat32-parser

Parser FAT32 en environnement `no_std` pour Rust.

## Compilation

```bash
cargo build
```

## Utilisation

Le programme a besoin d'une image FAT32 :

```bash
cargo run <image.fat32>
```

Exemple :
```bash
cargo run disk.img
```

## Tests

Tests avec détails :

```bash
cargo test -- --show-output
```

