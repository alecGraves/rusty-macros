# rusty-macros
My attempt at making windows cmd macros/aliases using rust... and also my intro to the rust language.

I could easily do this in C++ or C or python, but what's the fun in that? This way I get to jump on the hype train and see what this newfangled 'fireflower' is all about. Will I turn into fire mario? idk, but it's worth a shot.

## List of Macros:
* [X] ls
* [ ] rm
* [ ] mv
* [ ] cp
* [ ] touch
* [X] cat
* [ ] wget

## Usage:


## Notes:
I could use cargo, it is a nice system with plenty of functionality, but my use case is pretty simple, so instead, I am just using rustc:
```
rustc -C opt-level=3 <prog_name.rs>
```