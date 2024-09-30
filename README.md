# Rust FFI C Library

## To run rust code with c library
```
gcc -o libmymath.so -fpic -shared c-lib/mymath/mymath.c
```
```
LD_LIBRARY_PATH=./:$LD_LIBRARY_PATH cargo run -r
```

### additional command to debug

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
