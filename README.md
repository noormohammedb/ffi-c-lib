# Rust FFI C Library

> cd c-lib

> gcc -c main.c

> nm main.o

> cd c-lib/mymath

> gcc -o libmymath.so -fpic -shared mymath.c

> file libmymath.so

> nm libmymath.so

> cp libmymath.so ../

> cd c-lib

> gcc -c main.c -o main.o

> gcc -o main main.o -lmymath -L.

> ldd main

> LD_LIBRARY_PATH=. ./main
