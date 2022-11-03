# Clean Architecture by Rust

referenced by
[https://eh-career.com/engineerhub/entry/2022/09/12/093000/?PK=533056#tonic%E3%82%92%E4%BD%BF%E3%81%A3%E3%81%A6proto%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E3%81%8B%E3%82%89Rust%E3%81%AE%E5%9E%8B%E3%82%92%E7%94%9F%E6%88%90%E3%81%99%E3%82%8B](https://eh-career.com/engineerhub/entry/2022/09/12/093000/?PK=533056#tonic%E3%82%92%E4%BD%BF%E3%81%A3%E3%81%A6proto%E3%83%95%E3%82%A1%E3%82%A4%E3%83%AB%E3%81%8B%E3%82%89Rust%E3%81%AE%E5%9E%8B%E3%82%92%E7%94%9F%E6%88%90%E3%81%99%E3%82%8B)

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
