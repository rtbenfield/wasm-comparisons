FROM node:latest
VOLUME [ "/usr/scr/app" ]
WORKDIR /usr/src/app
RUN npm install -g assemblyscript
ENTRYPOINT [ "asc", "assembly/index.ts", "-b", "assemblyscript.wasm", "--optimize" ]