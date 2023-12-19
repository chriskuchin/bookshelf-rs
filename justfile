check:
  biome check --apply ./public

fmt:
  biome format --write ./public

lint:
  biome lint ./public

fix:
  biome check --apply ./public