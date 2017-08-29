# A piet interpreter
### The piet programming language
The [Piet programming language](http://www.dangermouse.net/esoteric/piet.html)
is similar to Brainf\*ck, except it is way more interesting. Piet programs are
[pictures](http://www.dangermouse.net/esoteric/piet/samples.html)!

### How to use
Clone this repository, cd into the directory, build using a nightly version of
the rust toolchain and you should be good to go!
```sh
git clone 'https://github.com/glacambre/piet'
cd piet
cargo build --release
target/release/pieti $piet_program.png
```

The program and its execution can be displayed using the --view flag. This
should show the picture and highlight the current codel.
Currently only the png format is supported. This might change.
