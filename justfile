# `cargo test` for all workspaces
test:
    cargo test --all

# run `just test` on any change
watch-test:
    watchexec -- just test

# run the api backend
api:
    cargo run -p api

# cargo fmt
fmt:
    cargo fmt

# compile web to wasm
web:
    (cd web; wasm-pack build --target web --out-name package --dev)

# run `just web` on changes to web source
watch-web:
    watchexec -w web/src -- just web

# serve the compiled web
serve: web
    (cd web; microserver)

# `tree` but ignores build directories
tree:
    tree -I "pkg|target|migrations" --dirsfirst

# compiles scss using `grass`
css:
    (cd web; grass scss/index.scss > index.css)

# run `just css` on changes to `web/scss`
watch-css:
    watchexec -w web/scss -- just css
