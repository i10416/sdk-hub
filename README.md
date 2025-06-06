# SDK Hub
This repository contains a set of SDKs(HTTP/gRPC clients) at ./sdk directory and the specification for SDKs in spec directory.

`./sdk` contains implementations in several languages. Each language implementation is located under `./sdk/{language}`.

Handwritten or semi handwritten(ai assisted) SDKs are put under `sdk/{language}/handwritten` directory while fully automated code generation builds are put under `sdk/{language}/gen`.

Runtime mechanisms such as authentication and request signing are put under `sdk/{language}/runtime`.
