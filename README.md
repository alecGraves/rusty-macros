# rusty-macros
My attempt at making windows cmd macros/aliases using rust... and also my intro to the rust language.

I could easily do this in C++ or C or python, but what's the fun in that? This way I get to jump on the hype train and see what this newfangled 'fireflower' is all about. Will I turn into fire mario? idk, but it's worth a shot.

## List of Macros:
1. ls
2. rm
3. mv
4. cp
5. touch
6. cat
7. wget

## Usage:


## Notes:
I could use cargo, it is a nice system with plenty of functionality, but my use case is pretty simple, so instead, I am just using rustc:
```
rustc -C opt-level=3 <prog_name.rs>
```