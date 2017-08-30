# A piet interpreter
### The piet programming language
The [Piet programming language](http://www.dangermouse.net/esoteric/piet.html)
is similar to Brainf\*ck, except it is way more interesting. Piet programs are
[pictures](http://www.dangermouse.net/esoteric/piet/samples.html)!

### My very own extension: SysPiet
SysPiet respects all the rules of the Piet programming language. However, it
adds one single instruction: Syscall. Syscall is represented by a Smoke codel.
The smoke codel has the color #C0C0C0. When encountered the smoke codel behaves
as follow:
- The first value of the stack is popped and used as the number of the syscall
  to be called.
- The second value of the stack is popped and used as the number of arguments
  to be used as a syscall.
- Two times the number of arguments are popped.
	- For each couple of values, the first one is the type of the argument.
	- The second is the argument itself.
- The syscall is called.
- The return value of the syscall is pushed in the stack.
- Resume execution, do not try to use the difference between the Smoke codel
  and the next codel as an operation (same behavior as when going from a white
  codel to a colored codel).

There are two types of arguments:
- 1: The argument is a regular number
- 2: The argument is a pointer. This means that the address of the beginning of
  the piet stack has to be added to the argument

Here's an example:
```
+-----+-------------------+-------------------------------------------+----------------------------------------+
| pos | Stack (grows up ) |     What it means to the interpreter      |      What it means to the syscall     |
+-----+-------------------+-------------------------------------------+---------------------------------------+
|  8  |          0        | Number of the syscall                     | Syscall nb. 0 is read()               |
|  7  |          3        | Number of arguments for the syscall       |                                       |
|  6  |          1        | The first argument is a number            | First arg is the file descriptor,     |
|  5  |          0        | The first argument is 0                   | 0 means stdin                         |
|  4  |          2        | The second argument is a pointer          | Second arg is the buffer that should  |
|  3  |          0        | The second argument is the address of the | be filled, here it's the beginning    |
|     |                   | first element of the piet stack plus 0    | of the piet stack                     |
|  2  |          1        | The third argument is a number            | Third arg is the number of bytes      |
|  1  |          1        | The third argument is 1                   | that should be read                   |
|  0  |         42        | Doesn't mean anything                     | This is where the data will be placed |
+-----+-------------------+-------------------------------------------+---------------------------------------+
```
If the stack looks like this when arriving on a Smoke codel, the read(stdin,
piet\_stack, 1) syscall will be executed. If the string "Hello world!" is
available in stdin, the piet\_stack will look like this once leaving the smoke
codel:
```
+-----+-------------------+-----------------------------------------------------------------------------------+
| pos | Stack (grows up ) |                                     What it means                                 | 
+-----+-------------------+-------------------------------------------+---------------------------------------+
|  1  |          1        | Return value of the syscall, here it is 1 because read() read a single byte.      |
|  0  |         72        | This is the byte read by read(), H is '72' in the ascii table.                    |
+-----+-------------------+-------------------------------------------+---------------------------------------+
```
This is an example syspiet program with a codel size of 1 and a codel size of
10 that will execute write(stdin, piet\_stack, 1):
![Codel Size=1](https://raw.githubusercontent.com/glacambre/piet/master/data/syspiet_read1.png)
![Codel Size=10](https://raw.githubusercontent.com/glacambre/piet/master/data/syspiet_read1_big.png)

### How to use
Clone this repository, cd into the directory, build using a nightly version of
the rust toolchain and you should be good to go!
```sh
git clone 'https://github.com/glacambre/piet'
cd piet
cargo build --release
target/release/pieti $piet_program.png
```

- Codel size defaults to 1. In order to change it, use the -c flag.
- Unknow codels default to white. In order to default to black, use the -b flag.
- Syspiet is disabled by default. In order to enable it, use the -s flag.

The program and its execution can be displayed using the --view flag. This
should show the picture and highlight the current codel. Using syspiet to
access stdin/stdout might make the display crash.
Currently only the png format is supported. This might change.
