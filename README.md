# WebAssembly Comparisons

This repo contains a few examples of WebAssembly languages, frameworks, and tools for evaluation.

## Running

| Project        | Command                                  |
| -------------- | ---------------------------------------- |
| AssemblyScript | `docker-compose run --rm assemblyscript` |
| Blazor         | `docker-compose run --rm blazor`         |
| Go             | `docker-compose run --rm go`             |
| Rust           | `docker-compose run --rm rust`           |

## Bundle Size

Bundle size is a major issue with most frameworks that compile to WebAssembly. The smallest bundle size (asside from writing raw WebAssembly Text) appears to be Rust, presumably because of the minimal overhead required for the memory allocator. On the opposite spectrum, Go requires a minimum of ~2MB for its core requirements. Some of this concern may be addressed through the [Reference Types Proposal](https://github.com/WebAssembly/reference-types).
