install:
  cd ui && npm install

build-wasm:
  cd wasm && wasm-pack build

dev:
  cd ui && npm run dev

build: build-wasm
  cd ui && npm run build

[parallel]
lint: lint-server lint-ui

lint-server:
  cargo clippy

lint-ui:
  cd ui && npm run lint

[parallel]
check: check-server check-ui

check-server:
  cargo check

check-ui:
  cd ui && npx tsc -b