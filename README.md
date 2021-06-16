![Logo](idiomatic-rust.png)

![Build Status - Check Links](https://github.com/mre/idiomatic-rust/workflows/Check%20Links/badge.svg)

This repository collects resources for writing clean, idiomatic Rust code. [Please bring your own.](https://github.com/mre/idiomatic-rust/blob/master/CONTRIBUTING.md) :blush:

> *Idiomatic* coding means following the conventions of a given language. It is the most concise, convenient, and common way of accomplishing a task in that language, rather than forcing it to work in a way the author is familiar with from a different language. - Adapted from [Tim Mansfield](https://github.com/tim-hr/stuff/wiki/Idiomatic-coding)

## ⚙ Projects

* [Rust Anthology](https://github.com/brson/rust-anthology) - The best short-form writing about Rust, collected.
* [cheat.rs - Idiomatic Rust tips](https://cheats.rs/#idiomatic-rust) - A list of quick tips to make your code more idiomatic.
* [clippy](https://github.com/rust-lang/rust-clippy) - A bunch of lints to catch common mistakes and improve your Rust code.
* [Patterns](https://github.com/rust-unofficial/patterns) - A catalogue of Rust design patterns.
* [Elements of Rust](https://github.com/ferrous-systems/elements-of-rust) - A collection of software engineering techniques for effectively expressing intent with Rust.
* [Possible Rust](https://www.possiblerust.com/) - A blog for intermediate Rust programmers exploring real-world code and design patterns. 
* [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - An extensive list of recommendations for idiomatic Rust APIs.
* [Rust by Example](https://rustbyexample.com/) - A community driven collection of example code which follow Rust best practices.

## 🏋 Workshops

* [Build your own JIRA with Rust](https://github.com/LukeMathWalker/build-your-own-jira-with-rust/) - A test-driven workshop to learn Rust building your own JIRA clone!
* [Ferrous Systems Teaching Material](https://ferrous-systems.github.io/teaching-material/index.html) - Free workshop material produced by Ferrous Systems for trainings. The time for the full course is around three to four days.
* [PingCAP talent plan](https://github.com/pingcap/talent-plan) - A series of training courses about writing distributed systems in Rust. The courses primarily consist of projects where coding problems are presented, along with a partial implementation or API description, and a test suite.
* [Procedural Macros Workshop](https://github.com/dtolnay/proc-macro-workshop) - This repo contains a selection of projects designed to learn to write Rust procedural macros — Rust code that generates Rust code.

## 📖 Books

* [Command Line Applications in Rust](https://rust-cli.github.io/book) - A tutorial on how to write CLI apps in Rust, learning many aspects of the ecosystem along the way.
* [Discover the world of microcontrollers through Rust!](https://rust-embedded.github.io/discovery/) - This book is an introductory course on microcontroller-based embedded systems that uses Rust as the teaching language rather than the usual C/C++.
* [Rust Cookbook](https://github.com/rust-lang-nursery/rust-cookbook) - Examples that demonstrate good practices to accomplish common programming tasks in Rust.
* [Rust for Rustaceans](https://nostarch.com/rust-rustaceans) (Early Access) by [Jon Gjengset](https://github.com/jonhoo) - Covers how to design reliable, idiomatic, and ergonomic Rust programs based on best principles.

## 📰 Articles

### 2021

* [Naming Your Lifetimes](https://www.possiblerust.com/pattern/naming-your-lifetimes) by [Possible Rust](https://www.possiblerust.com) - Explains how using longer, declarative lifetime names can help to disambiguate which borrow is which, and where it’s coming from.
* [Aiming for idiomatic Rust](https://shane-o.dev/blog/aiming-for-idiomatic-rust) by [Shane Osbourne](https://shane-o.dev/) - Discusses different ways to solve a popular coding puzzle, "balanced brackets", in Rust.
* [Wrapping errors in Rust](https://edgarluque.com/blog/wrapping-errors-in-rust) by [Edgar Luque](https://github.com/edg-l) - Wrapping `reqwest::Error` and a custom error type as an enum to make library usage easier. Also mentions [thiserror](https://github.com/dtolnay/thiserror) to automate that process.

### 2020

* [Refactoring Rust Transpiled from C](https://immunant.com/blog/2020/09/transpiled_c_safety/) by [Per Larsen](https://github.com/thedataking) - Describes how to lift a C-project that was automatically converted to unsafe Rust with C2Rust to safer, more idiomatic Rust with some human intervention. 
* [Learning Rust through open source and live code reviews](https://loige.co/learning-rust-through-open-source-and-live-code-reviews/) by [Luciano Mammino](https://github.com/lmammino) and [Stefano Abalsamo](https://github.com/stefanoabalsamo79) - Covers patterns like [FromStr](https://doc.rust-lang.org/std/str/trait.FromStr.html) and exposing a CLI and a library in one crate.
* [Guide on how to write documentation for a Rust crate](https://blog.guillaume-gomez.fr/articles/2020-03-12+Guide+on+how+to+write+documentation+for+a+Rust+crate) - Writing good documentation with rustdoc including many examples.
* [Are out parameters idiomatic in Rust?](https://steveklabnik.com/writing/are-out-parameters-idiomatic-in-rust) - Discusses the pros and cons of functions returning a value vs. modifying a parameter in-place.

### 2019

* [Await a minute](https://docs.rs/dtolnay/0.0.3/dtolnay/macro._01__await_a_minute.html) by [David Tolnay](https://github.com/dtolnay) - Example code for moving from raw futures to async/await syntax to improve error handling, native control flow, and borrowing. 
* [Taking string arguments in Rust](http://xion.io/post/code/rust-string-args.html) by [@Xion](https://github.com/Xion) - Discussing how to avoid subtle issues with string handling and when to use `str` (the string slice) and `String` (the owned/allocated string).
* [Rust Patterns: Enums Instead Of Booleans](http://blakesmith.me/2019/05/07/rust-patterns-enums-instead-of-booleans.html) by [Blake Smith](https://github.com/blakesmith) - Discusses how using enums instead of booleans can be useful to express intent more clearly without sacrificing safety thanks to Rust's strong semantics (like exhaustive pattern matching).
 
### 2018

* [Programming an ARM microcontroller in Rust at four different levels of abstraction](https://pramode.in/2018/02/20/programming-a-microcontroller-in-rust-at-four-levels-of-abstraction/) by [Pramode C.E ](https://pramode.in/) - Demonstrates how Rust helps to move from brittle, low-level embedded code to high-level abstractions with zero cost.

### 2017

* [Iteration patterns for Result & Option](http://xion.io/post/code/rust-iter-patterns.html) by [@Xion](https://github.com/Xion) - Explores how to filter and partition iterators of Result and Option types idiomatically.
* [Lessons learned redesigning and refactoring a Rust Library](https://blog.mgattozzi.dev/refactor-rust) by [@mgattozzi](https://github.com/mgattozzi) - `RefCell`, the builder pattern and more.
* [Math with distances in Rust: safety and correctness across units](https://ferrisellis.com/content/rust-implementing-units-for-types/) by [@code-ape](https://github.com/code-ape) - How to create a system to cleanly and safely do arithmetic with lengths.
* [The balance between cost, useability and soundness in C bindings, and Rust-SDL2&#39;s release](https://web.archive.org/web/20190509123207/https://cobrand.github.io/rust/sdl2/2017/05/07/the-balance-between-soundness-cost-useability.html) by [@Cobrand](https://github.com/Cobrand) - Writing safe, sound, idiomatic libraries despite the limitations of the borrow checker.

### 2016

* [Russian Dolls and clean Rust code](https://blog.mgattozzi.dev/russian-dolls) by [@mgattozzi](https://github.com/mgattozzi) - How to use the full power of `Option` and `Result` (especially `and_then()` and `unwrap_or()`).
* [Elegant Library APIs in Rust](https://deterministic.space/elegant-apis-in-rust.html) by [@killercup](https://github.com/killercup) - Many helpful tips and tricks for writing libraries in Rust.
* [Teaching libraries through good documentation](https://deterministic.space/teaching-libraries.html) by [@killercup](https://github.com/killercup) - How to use the full power of Rust's documentation support (e.g. doc tests)
* [Pretty State Machine Patterns in Rust](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/) by [@hoverbear](https://github.com/Hoverbear) - How to represent a State Machine in an expressive and understandable way in Rust.
* [Ripgrep Code Review](https://blog.mbrt.dev/posts/ripgrep/) by [@mbrt](https://github.com/mbrt) - An analysis of the popular `ripgrep` tool's source code.
* [Rustic Bits](https://llogiq.github.io/2016/02/11/rustic.html) by [@llogiq](https://github.com/llogiq/) - Small things that make for rustic code.
* [Convenient and idiomatic conversions in Rust](https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust) by [meqif](https://github.com/meqif) - Explains `From<T>`, `Into<T>`, `TryFrom<T>`, `TryInto<T>`, `AsRef<T>` and `AsMut<T>` with pratical examples.
* [Idiomatic tree and graph like structures in Rust](https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/) by [saschagrunert](https://github.com/saschagrunert) - Introduction to safe, dynamic, arena based tree structures without using lifetimes.

### 2015

* [Rust traits for developer friendly libraries](https://benashford.github.io/blog/2015/05/24/rust-traits-for-developer-friendly-libraries/) by [@benashford](https://github.com/benashford) - Thoughts about implementing good Rust libraries.
* [Error Handling in Rust](https://blog.burntsushi.net/rust-error-handling/) by [@BurntSushi](https://github.com/BurntSushi) - Understanding and handling errors in Rust in an idiomatic way.
* [Creating a Rust function that accepts String or &str](https://hermanradtke.com/2015/05/06/creating-a-rust-function-that-accepts-string-or-str.html) by [@hjr](https://github.com/hjr3) - How to make calling your code both ergonomic and fast (zero-allocation).
* [Creating a Rust function that returns a &str or String](https://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html) by [@hjr](https://github.com/hjr3) - How `Into` and `Cow` (Clone-on-write) work together to avoid allocations for string types.
* [Effectively Using Iterators In Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html) by [@hjr](https://github.com/hjr3) - Explanation of the `Iter` and `IntoIter` traits and how loops actually work in Rust.
* [Strategies for solving 'cannot move out of' borrowing errors in Rust](https://hermanradtke.com/2015/06/09/strategies-for-solving-cannot-move-out-of-borrowing-errors-in-rust.html) by [@hjr](https://github.com/hjr3) - Practical tips to help understand the borrow-checker and move semantics.
* [Rayon: data parallelism in Rust](https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/) by [@nikomatsakis](https://github.com/nikomatsakis) - Writing elegant parallel code in Rust.

## 🎤 Talks

### 2020

Macros for a More Productive Rust [[Video](https://www.youtube.com/watch?v=dZiWkbnaQe8)] by [@jam1garner](https://github.com/jam1garner) - RustConf 2020

### 2019

Making Rust Delightful [[Video](https://www.youtube.com/watch?v=YSEx8wtlPWc)] by [@nrc](https://github.com/nrc/) - RustCon Asia 2019  

### 2018

Idiomatic Rust - Writing Concise and Elegant Rust Code [[Video](https://www.youtube.com/watch?v=P2mooqNMxMs)] [[Slides](https://speakerdeck.com/mre/idiomatic-rust-writing-concise-and-elegant-rust-code)] by [@mre](https://github.com/mre) - FOSDEM 2018

### 2017

Idiomatic Rust Libraries [[Video](https://www.youtube.com/watch?v=0zOg8_B71gE)] [[Slides](https://killercup.github.io/rustfest-idiomatic-libs/index.html#/)] by [@killercup](https://github.com/killercup) - Rustfest Kiev  
What's so hard about writing a Slack Client in Rust? [[Video](https://www.youtube.com/watch?v=rrtJh1kz1Ms)] [[Slides](https://speakerdeck.com/mre/whats-so-hard-about-writing-a-slack-client-in-rust)] by [@mre](https://github.com/mre) - Rust Cologne Meetup

## 💬 Forum

### 2020

* [Preferred way of passing `Path`-like types around?](https://www.reddit.com/r/rust/comments/cekeq9/preferred_way_of_passing_pathlike_types_around/)

### 2017

* [Which is more idiomatic? Functional, imperative or a mix?](https://users.rust-lang.org/t/which-is-more-idiomatic-functional-imperative-or-a-mix/11278)
* [An idiomatic way to sum up values in a multidimensional Array](https://users.rust-lang.org/t/an-idiomatic-way-to-sum-up-values-in-a-multidimensional-array/9485)

## 📜 History

Coming from Python, I loved the guidelines on how *idiomatic Python* looks like. I was inspired by the likes of Peter Norvig, who wrote amazing articles on [spellcheckers](https://norvig.com/spell-correct.html) and [sudoku solvers](https://norvig.com/sudoku.html); and, of course, the [Zen of Python](https://www.python.org/dev/peps/pep-0020/). For Rust, there is no such thing as the Zen of Python, however, so I started collecting my own resources.
The goal of this project is to create a peer-reviewed collection of articles/talks/repos, which teach idiomatic Rust style. It's a community project and you can contribute.

## 🔏 License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
Logo adapted from [FreePik.com](https://www.freepik.com/free-vector/crabs-pattern-design_1093131.htm).
