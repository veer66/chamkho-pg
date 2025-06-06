FROM docker.io/rust:1.84 AS build
RUN apt-get update; apt-get upgrade -y
RUN apt-get install -y clang llvm-dev
RUN wget https://ftp.postgresql.org/pub/source/v16.9/postgresql-16.9.tar.gz
RUN tar xzf postgresql-16.9.tar.gz
RUN cd postgresql-16.9; ./configure --prefix=/usr/local; make -j$(nproc); make install
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
COPY --from=build /work/data/chamkho-dict.txt /usr/share/postgresql/16/tsearch_data/
COPY --from=build /work /work


