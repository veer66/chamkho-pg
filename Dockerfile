FROM docker.io/rust:1.77 AS build
RUN apt-get update; apt-get upgrade -y 
RUN apt-get install -y clang llvm-dev
RUN wget https://ftp.postgresql.org/pub/source/v15.6/postgresql-15.6.tar.gz
RUN tar xzf postgresql-15.6.tar.gz
RUN cd postgresql-15.6; ./configure --prefix=/usr/local; make -j$(nproc); make install
RUN mkdir /work
COPY Cargo.toml Cargo.lock install.sh README.md build.rs wrapper.h /work/
COPY src/ /work/src/
COPY control/ /work/control/
COPY data/ /work/data/
COPY sql/ /work/sql
WORKDIR /work
RUN cargo build --release

FROM docker.io/postgres:15.6
COPY --from=build /work/target/release/libchamkho_parser.so /usr/lib/postgresql/15/lib/chamkho_parser.so
COPY --from=build /work/control/*.control /work/sql/*.sql /usr/share/postgresql/15/extension/
COPY --from=build /work /work


