# Rust FFI C Library

> cd c-lib

> gcc -c main.c

> nm main.o

> cd c-lib/mymath

> gcc -o libmymath.so -fpic -shared mymath.c

> file libmymath.so

> nm libmymath.so

> cd c-lib

> gcc -c main.c -o main.o

> gcc -o main main.o -lmymath -L.

> ldd main

> LD_LIBRARY_PATH=./mymath ./main

> LIBRARY_PATH="$(pwd)/c-lib/mymath:$LIBRARY_PATH" cargo run -r
