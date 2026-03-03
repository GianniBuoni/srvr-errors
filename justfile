lint:
  cargo fmt --check
  cargo clippy --all-targets --all-features -- -D warnings

test: _init_db
  cargo test -F postgres

build: test
  cargo build -F postgres

@_start_db:
  if [ "$(pg_ctl status | grep 'no server running')" = "" ]; then \
    echo "=== DB already running ==="; \
  else \
    echo "=== Starting DB ==="; \
    pg_ctl start -l $PGDATA/logfile -o --unix_socket_directories=$PWD/$PGDATA; \
  fi

@_init_db:
  if [ "$(pg_ctl status)" ]; then \
    just _start_db; \
  else \
    echo "=== Initalizing DB ==="; \
    pg_ctl init; \
    just _start_db; \
    sqlx database create; \
    sqlx migrate run; \
  fi
