FROM rust:buster
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

RUN apt install -y libpq-dev

WORKDIR /usr/src/app
COPY ./Cargo.toml .
COPY ./Cargo.lock .

# Create fake main.rs file in src and build
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release

RUN rm -rf ./src
COPY ./src ./src
COPY ./migrations ./migrations
COPY diesel.toml ./

RUN touch -a -m ./src/main.rs

RUN cargo build --release

CMD [ "./target/release/bokeh" ]