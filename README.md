# embedded-exercises
Repo for the second rust playground hosted by ALTEN

## 0. Introduction

You can run the tests either in vscode by clicking Run / Run Tests inline in the rs files. Alternatively you may use `cargo run` / `cargo test` to execute the assignments.

All exercises require you to create an implementation for them.
Some require you to add arguments to the function in order to match the tests.

## 1. A little C and a little Rust

Time for a blend! You are the proud owner of a large C code base. As you ponder whether this is viable for future work, your looking for options to include some Rust. One way is to create remote interfaces, have inter process communication or network based communication.

Not today! What we will do is compile the new Rust additions into your C program. This allows you to introduce new code whilst you keep all precious years of history and battle tested code.


## 2. A little Rust and a little C

The other version of exercise 1. You have Rust, want more Rust, but are forced to include some C in there. This exercises creates external functions in C which you can link and use in Rust.

## 3. Heapless

Memory? Memory! But just a bit.. In this exercise you will work with lists and strings without use of heap allocated memory. This allows you to define the amount you need to use and have a sure way to fit everything in your embedded device.

Can you use heapless for all instances of standard library versions?

## 4. Embedded graphics

Time to use your artistic skills and put it on hardware! Do you know Ferris? He is the Rust mascot you see in all stories regarding Rust. Could you draw Ferris on the ESP display?

*Did you know* you can run your application on your desktop sharing most of the code base? Only the hardware layer is separated. 

**NOTE**
Compilation of this project on Windows may require modifying the path of your project as Windows has a limit on the maximum path size. You can mount a directory to a drive letter as follows:

```
subst "R:" "rust-project"
```

Afterwards compile in `R:\` as your path
