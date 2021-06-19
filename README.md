
# Overview of rust-playground - Calculator

### Why?
Hello, my name is Hunter Wilhelm and welcome to my portfolio! I am trying to tackle the learning curve of Rust. This is the first time that I am using it, and it is rather exciting.


### What is this calculator?
> This calculator goes to show that most calculators, like this one, do not have
order of operations, rather they mutate the result one operation at a time.
This does have one extra feature that most basic calculators don't have: undo.
The instructions will be shown on screen.

[Overview video](https://youtu.be)

### How to use this calculator
When it first starts it shows
```
My Calculator
Enter a command to continue
 - e to enter a number
 - u to undo
 - q to quit
```
Press one of those keys then press enter.

#### Enter a number
```
e
Enter a number
1
Stack:   (1 )
Result: 1
Enter a command to continue
 - e to enter a number
 - u to undo
 - q to quit
e
Enter an operation to continue (+, -, /, *)
-  
Enter a number
2
Stack:   (1 - (2 ))
Result: -1
```

#### Undo
```
Stack:   (1 - (2 ))
Result: -1
Enter a command to continue
 - e to enter a number
 - u to undo
 - q to quit
u
Stack:   (1 )
Result: 1
```

#### Quit
```
Enter a command to continue
 - e to enter a number
 - u to undo
 - q to quit
q
```

### Development Environment
I used VS Code with these extensions:
* [VsCode Action Buttons](https://marketplace.visualstudio.com/items?itemName=seunlanlege.action-buttons)
* [Rust](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

I installed Rust using this website
* [Rust Install](https://www.rust-lang.org/tools/install)

###  Useful commands
- `cargo check` (to check for any errors without compiling)
- `cargo build` (to check and make the executable)
- `cargo run` (to build and run)

###  Useful links
- [Rust documentation book](https://doc.rust-lang.org/book/ch00-00-introduction.html)
- [Rust Tutorial—TutorialsPoint](https://www.tutorialspoint.com/rust/index.htm)
- [Rust Official site](https://www.rust-lang.org/)

### Future work
* Add order of operations
* Add other operations like ^ * > <
* More user friendly