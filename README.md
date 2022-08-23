# AAARGH
### Adequate And Anxiety-Reducing, General HELPME

## docs

the others aren't here, it's just me, KS writing this  

### support

so far releases only contain executables for windows & linux  
specifically, binaries built with `x86_64-pc-windows-msvc` and `x86_64-unknown-linux-musl`,
whatever that means  
mac users are gonna have to build this on their own, frick you tim cook

### usage

much of the arguments should be documented [here](src/main.rs), but i'll put some general usage here again

the simplest call goes something like this:
```shell
aargh prog.cpp --fin input.txt --fout ans.txt
```
this will compile run the program with `input.txt` redirected into standard input
(console input, whatever you like to call it)

but there's a _whole_ buncha options, i'll go over them
* `--gen` or `-g`- the generator script that should print random test cases to stdout
  * `--ans` or `-a`- the checker program that outputs the actual answer for each test case
  * `--gen-amt`- how many times do you want to run the generator? (default is 20)
* `--fin`- file (or directory) to use for input
  * `--fout`- file (or directory) to use for actual output (must be same type as `fin`)
* `--fin-fmt` & `--fout-fmt`- if `fin` and `fout` are directories, i'm gonna need a format for what the files
                              in each directory are like
  * the program starts from `1` and stops when it can't find input or output files that match the criteria
  * you define where the number goes with `{}`
    * for example, `test{}.in` would have the program try `test1.in`, `test2.out`, etc.
  * program dies if you don't give at least one `{}`
* `--prog-fin` & `--prog-fout`- these arguments don't depend on each other, but they're grouped really close together
  * `--prog-fin` determines what _file_ your _own program_ reads from- `aargh` will create the file, dump the input in,
  then execute the program (if unfilled, standard input will be used)
  * `--prog-fout` has you give what file the program will put its output in
  (if unfilled, standard output will be used)
* `--whitespace-fmt`- some graders just care about the numbers, not the spacing between them. if your
                      grader isn't one of these, put this option here
* `--str-case`- when comparing strings, should case matter? i.e. should `abc` count as being different from `AbC`?
* `--prog-stdout` & `--prog-stderr`

and sometimes you wanna pass in special commands to the compiler as well  
that's completely fine, just put a `--` at the end of your command, then type in your arguments like so:
```shell
aaargh -- test/test.cpp --fin test/input.txt --fout test/ans.txt --prog-stdout --prog-stderr -- -std=c++17
```
so now `-std=c++17` will get passed to `g++`
