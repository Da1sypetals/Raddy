
alias gen := generate

default:
    @just --list

generate:
    python meta/operators.py
    python meta/scalar_matrix_mul.py   

test n:
    #!/bin/bash
    for i in {1..{{n}}}; do cargo test; done
    for i in {1..{{n}}}; do cargo test --release; done

hub branch:
    #!/bin/bash
    git push github {{branch}}
         
        
    