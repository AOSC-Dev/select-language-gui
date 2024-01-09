build:
    @echo "Building js ..."
    yarn vite build
    @echo "Building rust ..."
    cd src-tauri && cargo build --release --features custom-protocol

clean:
    rm -rf ./dist
    cd src-tauri && cargo clean

