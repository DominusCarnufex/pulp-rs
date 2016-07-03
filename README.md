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

### En Rust

S’agissant d’une bibliothèque, elle n’est pas exécutable directement. Il faudra faire appel tout d’abord à la fonction suivante.

```rust
pub fn version(bytecode : &[u8]) -> Result<(u8, u8, u8), String>
```

Si le *bytecode* fourni en entrée est valide et utilise une version supportée par l’interpréteur tel qu’il est compilé, cette fonction renvoie la version du *bytecode* utilisée sous la forme d’un triplet de `u8`. Pour une version `x.y.z` donnée, il faudra ensuite appeler la fonction `run(bytecode : &[u8])` du module `v<x>_<y>_<z>`. Chacune a sa propre signature de type, car elle ne renvoie pas toujours la même chose.

### En C, ou autre langage ayant une FFI C

La bibliothèque expose l’équivalent de la fonction `verion()` ci-dessus, accessible depuis une ABI C.

```rust
pub extern fn version_c(entree : HeapVar) -> HeapVar
```

Quel est donc ce type HeapVar ? C’est tout simplement un pointeur et une longueur, permettant de faire passer n’importe quel type de donnée vers l’extérieur de la bibliothèque, sans que celles-ci soient désallouées à la fin de l’exécution de la bibliothèque. Voici sa définition en Rust.

```rust
pub struct HeapVar  {
    pointer : *mut u8,
    size    : usize
}
```

Le `HeapVar` fourni en entrée doit contenir le *bytecode* complet, sous forme d’un tableau de `u8` (alias `unsigned char` en C).

Le `HeapVar` renvoyé par la fonction `version_c()` peut contenir deux choses. Soit un triplet de `u8`, correspondant aux trois morceaux de la version du *bytecode* (`size` vaut alors 3). Soit un message d’erreur, sous la forme d’une chaîne de caractères *null terminated*, à la C (`size` vaut alors plus que 3).

Une fois obtenu la version, il est alors possible d’appeler la fonction `run_v<x>_<y>_<z>()`, qui prend en entrée le même `HeapVar` que `version_c()`, et renvoie en sortie quelque chose de différent selon la version. Reportez-vous à la définition de chaque fonction.

Notez également que vous trouverez dans le dossier `tests_ffi` des exemples d’utilisation possible en C des fonctions de la bibliothèque.

## Évolutions futures

Voyez les *issues* ou discutez-en [sur le forum de Zeste de Savoir](https://zestedesavoir.com/forums/sujet/6401/pulp-un-environnement-dexecution-multivitamine/).
