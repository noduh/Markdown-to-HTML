# Overview

This project is one I chose in an attempt to learn some basic Rust programming.

The software is a work in progress Markdown to HTML converter.
When finished, it will take an input Markdown file and output HTML.
It will do this by parsing the Markdown and putting it into a custom Abstract Syntax Tree (AST).
After putting it into the AST, it will recursively read the AST and begin writing the HTML.

Currently, all that is finished is the basic structure of the [AST](src/formatted_text.rs).
However, even given that many of the methods and associated functions are broken.
The main focus is the `get_end()` helper function.
This recursively finds the next place to add formatting and text.
Once that is done, many of the remaining functions should fall in to place.

I wanted a project that would be challenging, teach me Rust effectively, and be fun.
I am a huge fan of Markdown, and I figured it would be a lot of fun to write a compiler for it.
Instead, I found that the biggest problem is how strict Rust it.
It's interestingly both a language that I'm loving, and one I'm getting very frustrated with.

Currently, since I do not have a working product, there's no demonstration. When it's finished, the video will be found below.

[Markdown to HTML in Rust]()

# Development Environment

The main tools used were Git, VSCode, and Cargo.
VSCode had some extensions running to make the development environment user friendly (for me at least).
Git was actually a lifesaver.
I made at least one mistake where I totally deleted everything and was only able to recover it thanks to Git.
Finally, I discovered that Cargo is _amazing_.
It handles so much of rust for you.
It can take care of compatibility, versioning, libraries, and more.
It even adds things to the `.gitignore`!

As said before, this was done in Rust.
Rust is a very stubborn language.
The compiler complains about every little thing, and _forces_ you to handle things.
It does this in the name of memory safety, and honestly it's a very clever language.
The thing I love about it is also the thing I hate about it.
It's **so** stubborn!
You have to handle stuff like ownership and stuff so that it can accurately determine how to handle memory.
It's frustrating.
_However_, that's also why I love it.
It's difficult to make a mistake that will ruin the memory, and it does it in an extremely clever way.
That's why I also love it.

So far the only libraries I've been using are part of `std`.
Naturally that may change, and I'll try to update this if it does.

# Useful Websites

I have found the following resources very helpful in my development so far.

- [The Rust Programming Language](https://doc.rust-lang.org/book/) (awesome book, highly recommend)
- [Google Gemini](https://gemini.google.com/app) (it wasn't writing the code, it was explaining it and giving _ideas_)
- [GitHub](https://github.com/) (probably where you're reading this lol)

# Future Work

Here's what I still need to work on:

- Finish the functions in [`formatted_text.rs`](src/formatted_text.rs)
- Create the lexer/parser to take use of the AST
- Add what will read the AST and write the HTML
- Make a demo video
- Extent Markdown syntax!
- User interface (UI)  
   _This will probably just be a CLI for a while (maybe forever?)_
- Polish the code
