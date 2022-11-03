# Clean Architecture by Rust

referenced by
[https://eh-career.com/engineerhub/entry/2022/09/12/093000](https://eh-career.com/engineerhub/entry/2022/09/12/093000)

## Requirements

* Rust
* Docker
* VSCode (Remote Container)

## Directory structure

```
.
├── app                 ... entry point
│
├── context             ... di
│   ├── app_context     ... provide usecase
│   └── mock_context
│
├── domain              ... domain logic
│   ├── entity
│   └── repository
│
├── infra               ... I/O with the outside
│   ├── db_schmea
│   ├── grpc_handler
│   └── repository_impl
│
├── proto               ... gRPC
│   └── user
│
└── usecase             ... application usecases
```
