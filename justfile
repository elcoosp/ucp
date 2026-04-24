wr:
    watchexec -w ./wr.sh --clear -r "sh ./wr.sh"

test:
    cargo nextest run
