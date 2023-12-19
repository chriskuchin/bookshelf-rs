check-fix:
  biome check --apply ./public

fmt:
  biome format --write ./public

lint:
  biome lint ./public

lint-fix:
  biome lint --apply ./public

pkg-js:
  npx webpack

run:
  cargo run