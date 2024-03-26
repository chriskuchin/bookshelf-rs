check-fix:
  biome check --apply ./public

fmt:
  biome format --write ./public

lint:
  biome lint ./public

lint-fix:
  biome lint --apply ./public

pkg:
  npx webpack --mode=development

run:
  cargo run

update:
  kubectl rollout restart deploy/bookshelf-deployment

monitor:
  kubectl logs deployment/bookshelf-deployment -f