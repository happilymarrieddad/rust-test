FROM rust:latest

WORKDIR /app/

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch

# watch - for changes
# --why - when change happens cargo will output filename and what triggered changes
# -x - execute
# (could do) run instead of build (would run the webserver automatically)
CMD ["cargo", "watch", "--why", "-x", "build"]

# lightweight
# CMD ["cargo", "watch", "--why", "--", "echo"]