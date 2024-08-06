run *args:
    cargo watch -x 'run -- {{args}}'

fmt:
  cargo +nightly fmt