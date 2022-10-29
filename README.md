# Clean Architecture by Rust

## Requirements

* Rust
* Docker
* diesel cli

## Directory structure

```
.
├── app                 ... entry point
│
├── context             ... di
│
├── domain              ... domain logic
│   ├── entity
│   └── repository
│
├── infra               ... I/O with the outside
│   ├── grpc_handler
│   └── repository_impl
│
├── proto               ... gRPC
│
└── usecase             ... application usecases
```
