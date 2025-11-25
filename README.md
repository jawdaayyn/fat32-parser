# fat32-parser

Parser FAT32 en environnement `no_std` pour Rust.

## Compilation

```bash
cargo build
```

## Utilisation

Générer une image de test :

```bash
cargo run generate-img
```

Parser une image FAT32 :

```bash
cargo run <image.fat32>
```

Exemple complet :
```bash
cargo run generate-img
cargo run test.img
```

## Tests

Tests avec détails :

```bash
cargo test -- --show-output
```

