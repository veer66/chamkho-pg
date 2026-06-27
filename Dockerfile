FROM docker.io/postgres:18 AS build
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y clang llvm-dev curl pkg-config postgresql-server-dev-18
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install cargo-pgrx --version 0.18.0 --locked
RUN cargo pgrx init --pg18 /usr/bin/pg_config
RUN mkdir /work
COPY Cargo.toml Cargo.lock chamkho_parser.control build.rs install.sh /work/
COPY src/ /work/src/
COPY tsearch_data/ /work/tsearch_data/
WORKDIR /work
RUN ./install.sh

FROM docker.io/postgres:18
COPY --from=build /usr/lib/postgresql/18/lib/chamkho_parser.so /usr/lib/postgresql/18/lib/chamkho_parser.so
COPY --from=build /usr/share/postgresql/18/extension/chamkho_parser.control /usr/share/postgresql/18/extension/chamkho_parser.control
COPY --from=build /usr/share/postgresql/18/extension/chamkho_parser--0.6.0.sql /usr/share/postgresql/18/extension/chamkho_parser--0.6.0.sql
COPY --from=build /usr/share/postgresql/18/tsearch_data/chamkho_dict.txt /usr/share/postgresql/18/tsearch_data/chamkho_dict.txt
