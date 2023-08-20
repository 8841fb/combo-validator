## [combo-validator](/)

> a valid combolist checker (mail:pass) written in rust

### features
- fast and lightweight
- removes duplicates and invalid emails
- easy to use


### running

```bash
git clone https://github.com/obstructive/combo-validator.git
cd combo-validator
cargo run --release <path/to/combo.txt>
```

### building

```bash
git clone https://github.com/obstructive/combo-validator.git
cd combo-validator
cargo build --release # use as ./target/release/combo-validator <path/to/combo.txt>
```


### notes
- you may need a decent pc to run this in a big combolist (500k+ lines)
- the code is kinda a mess.