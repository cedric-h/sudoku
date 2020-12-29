cargo build --target=wasm32-unknown-unknown --release
cp ./target/wasm32-unknown-unknown/release/sudoku.wasm ./sudoku.wasm
wasm-strip sudoku.wasm
