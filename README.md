### Try

- Install rust         // https://www.rust-lang.org/tools/install
- Install nodejs & npm // https://crates.io/crates/cargo-generate

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
wasm-pack build
cd www
npm i
npm run start
```
