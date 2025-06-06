FROM docker.io/postgres:16.9 AS build
RUN apt-get update; apt-get upgrade -y
RUN apt-get install -y clang llvm-dev curl postgresql-server-dev-16
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN mkdir /work
COPY Cargo.toml Cargo.lock install.sh README.md build.rs wrapper.h /work/
COPY src/ /work/src/
COPY control/ /work/control/
COPY data/ /work/data/
COPY sql/ /work/sql
WORKDIR /work
RUN cargo build --release

FROM docker.io/postgres:16.9
COPY --from=build /work/target/release/libchamkho_parser.so /usr/lib/postgresql/16/lib/chamkho_parser.so
COPY --from=build /work/control/*.control /work/sql/*.sql /usr/share/postgresql/16/extension/
COPY --from=build /work/data/chamkho_dict.txt /usr/share/postgresql/16/tsearch_data/
COPY --from=build /work /work


