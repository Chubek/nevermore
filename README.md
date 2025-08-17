## Nevermore: a Minimal Terminal Pager

I built this pager to learn Rust. I realize I have other Rust projects, but those are _noise_, so to speak!

### Building

Just use Cargo's build system. 

### Input

Input is given either via STDIN, or as the first argument:

```sh
for i in (seq 1 10000); echo $i; end | nevermore
```

or

```sh
nevermore foo.txt
```

### Key bindings

* `q` -> Quit;
* `k`/UpArrow -> Up;
* `j`/DownArrow -> Down;
* Home -> Scroll to top;
* End -> Scroll to bottom;
* PageUp -> Jump screen up;
* PageDown -> Jump screen down;

You may also send a SIGINT to quit (Ctrl + C).


### Windows

This project uses `crossterm` so it should work on Windows without an issue.


### Unused Packages/Imports

I have plans for this project, that's why there's some packages/imports that are unused. For example, I want to add syntax highlighting, ala `bat`.


### My Other Work

I have [+70 projects](https://github.com/Chubek) on my Github frontpage, give them a looksie!
