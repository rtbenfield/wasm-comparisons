FROM rust:latest
VOLUME [ "/usr/src/app" ]
WORKDIR /usr/src/app
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
CMD wasm-pack build