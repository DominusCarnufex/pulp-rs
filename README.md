# pulp-rs
Bibliothèque pour l’exécution de *bytecode* PULP (https://github.com/ArnaudCalmettes/pulp), écrite en Rust.

## Compilation

Si vous utilisez `cargo`, la commande suivante permet de compiler la bibliothèque.

```
cargo build --release --features "<versions supportées>"
```

Les versions supportées sont les versions du *bytecode*. Elles ont la forme suivante, et doivent être séparées les unes des autres par une espace :

- `v0_1_0` pour une version précise ;
- `v0_1_x` pour toutes les versions patchlevel d’une version mineure ;
- `v0_x`   pour toutes les versions mineures et patchlevel d’une version majeure.

Pour simplement lancer les tests, la commande est la suivante.

```
cargo test --features "<versions supportées>"
```

Malheureusement, `cargo` ne permet pas de passer n’importe quelle option au compilateur. Aussi, si vous voulez que la bibliothèque pulp soit liée dynamiquement à la bibliothèque standard de Rust, vous devrez compiler avec la commande suivante (n’oubliez pas les `\`, ils sont indispensables).

```
rustc -O -o target/libpulp.so -C prefer-dynamic --crate-type dylib [--cfg feature=\"<version supportée>\"] src/lib.rs 

```

La portion entre crochets doit être répétée autant de fois qu’il y a de version différente à supporter.

## Exécution

S’agissant d’une bibliothèque, elle n’est pas exécutable directement. Il faudra faire appel à la fonction suivante.

```rust
pub fn run(bytecode : &[u8]) -> Result<(), String>
```

À l’heure actuelle, il n’existe pas d’interface permettant d’appeler cette fonction depuis un programme en C (ou tout autre langage disposant d’un système de FFI utilisant les conventions d’appel du C), le programme appelant devra nécessairement être en Rust. Cela est amené à changer dans un futur proche.

## Évolutions futures

Voyez les *issues* ou discutez-en [sur le forum de Zeste de Savoir](https://zestedesavoir.com/forums/sujet/6401/pulp-un-environnement-dexecution-multivitamine/).
