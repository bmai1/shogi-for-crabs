Shogi GUI in Rust with lishogi assets, shogi-rs, and apery_rust shogi engine. 

What is Shogi? It's Japanese chess, and I suck at it.

## Installation

1. Clone and navigate to the repository:
```bash
git clone https://github.com/bmai1/shogi-for-crabs.git
cd shogi-for-crabs
```

2. Specify the USI engine and the path to the evaluation files.  
For example, place the evaluation binaries for [apery_rust evaluation binaries](https://github.com/HiraokaTakuya/apery_rust) in the root directory (shogi-for-crabs).
```bash
git clone https://github.com/HiraokaTakuya/apery_rust.git && \
cd apery_rust && \
git submodule init && \
git submodule update && \
cargo build --release
```

3. Go back to root directory and run:
```bash
cd ..
cargo run -- --engine apery_rust/target/debug/apery --engine-option Eval_Dir=apery_rust/eval/20190617
```

## Demo

https://github.com/user-attachments/assets/58d2ca56-ae4a-4f7f-b5db-17d639507a50


## Reference

![pieces](/reference/moves.png)



