### Try

- Install nodejs & npm // https://nodejs.org/en/
- Install rust         // https://www.rust-lang.org/tools/install
- Install cargo-generate // https://crates.io/crates/cargo-generate

```
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
wasm-pack build
cd www
npm i
npm run start
```
