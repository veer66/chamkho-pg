FROM rust:1.44 AS build
RUN wget https://ftp.postgresql.org/pub/source/v12.3/postgresql-12.3.tar.gz
RUN tar xzf postgresql-12.3.tar.gz
RUN cd postgresql-12.3; ./configure; make -j$(nproc); make install
RUN mkdir /work
COPY Cargo.toml Cargo.lock install.sh README.md /work/
COPY src/ /work/src/
COPY control/ /work/control/
COPY data/ /work/data/
COPY sql/ /work/sql
WORKDIR /work
RUN apt-get update; apt-get upgrade -y 
RUN apt-get install -y clang llvm-dev
RUN PG_INCLUDE_PATH=/usr/local/pgsql/include/server LLVM_CONFIG_PATH=/usr/bin/llvm-config-7 cargo build --release

FROM postgres:12.3
COPY --from=build /work/target/release/libchamkho_parser.so /usr/lib/postgresql/12/lib/chamkho_parser.so
COPY --from=build /work/control/*.control /work/sql/*.sql /usr/share/postgresql/12/extension/
#COPY --from=build /usr/local/cargo /usr/local/cargo
COPY --from=build /work /work


