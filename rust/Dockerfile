FROM rust:latest
VOLUME [ "/usr/src/app" ]
WORKDIR /usr/src/app
# RUN cargo install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
COPY . .
RUN wasm-pack build
# COPY ./build.sh ./build.sh
# RUN ./build.sh
# ENTRYPOINT [ "./build.sh" ]