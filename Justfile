
alias gen := generate
alias doc := document

default:
    @just --list

# execute codegen
generate:
    python meta/operators.py
    python meta/scalar_matrix_mul.py   

# run test for n times
test n:
    #!/bin/bash
    for i in {1..{{n}}}; do cargo test --lib; done
    for i in {1..{{n}}}; do cargo test --lib --release; done

# build documentation
document:
    cargo doc --no-deps