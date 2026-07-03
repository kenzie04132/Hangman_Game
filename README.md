# Overview

I made this software to try implementing true memory safety and lower-level code structure, which is something I've always had a bit of a hard time understanding. Managing memory without a garbage collector is not my strong suit, so I knew I needed to tackle that insecurity by integrating references and strict variable controls, even if it's in a game as simple as Hangman.

I made a Hangman app that manages data in real time from a built-in word list. The software bases what word it's choosing from that bank on a math calculation tied to the computer's internal clock (which guarantees a new word choice every time you boot it up). It then takes user keyboard entries, cleans up text spacing, validates if the letter is a match, and loops the game in a nice, easy to read format.

The primary purpose of writing this software was to get comfortable with references and borrowing, but also to learn about Rust, which is not a language I've ever worked with before.

[Software Demo Video](https://youtu.be/d8lEdso1IRU)

# Development Environment

I used Visual Studio Code to develop this software as well as the standard Rust terminal tools.

I used the Rust language and the built-in standard I/O (Input/Output) and Time libraries.

# Useful Websites

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust STD](https://doc.rust-lang.org/std/)
- [Tutorials Point](https://www.tutorialspoint.com/rust/index.htm)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}

- I would like to add the ability for the program to pull from a massive external text file dictionary, but that would require learning how Rust handles file pathing and reading text off a hard drive.
- Adding the ability to ask the user if they want to play another round at the end of a match, rather than just shutting down the whole program.
- In the future it would be cool to add classic ASCII text art of a little hanging gallows that physically draws itself on screen every time the mistake counter goes up.