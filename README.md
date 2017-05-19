![Logo](idiomatic-rust.png)

This repository collects resources for writing clean, idiomatic Rust code. Please bring your own. :blush:

> *Idiomatic* coding means following the conventions of a given language. It is the most concise, convenient, and common way of accomplishing a task in that language, rather than forcing it to work in a way the author is familiar with from a different language. - Adapted from [Tim Mansfield](https://github.com/tim-hr/stuff/wiki/Idiomatic-coding)

## Articles

###  2017

* [Lessons learned redesigning and refactoring a Rust Library](https://mgattozzi.com/refactor-rust) by [@mgattozzi](https://github.com/mgattozzi) - `RefCell`, the builder pattern and more.

### 2016

* [Russian Dolls and clean Rust code](https://mgattozzi.com/russian-dolls) by [@mgattozzi](https://github.com/mgattozzi) - How to use the full power of `Option` and `Result` (especially `and_then()` and `unwrap_or()`).
* [Elegant Library APIs in Rust](https://deterministic.space/elegant-apis-in-rust.html) by [@killercup](https://github.com/killercup) - Many helpful tips and tricks for writing libraries in Rust.
* [Teaching libraries through good documentation](https://deterministic.space/teaching-libraries.html) by [@killercup](https://github.com/killercup) - How to use the full power of Rust's documentation support (e.g. doc tests)
* [Pretty State Machine Patterns in Rust](https://hoverbear.org/2016/10/12/rust-state-machine-pattern/) by [@hoverbear](https://github.com/Hoverbear) - How to represent a State Machine in an expressive and understandable way in Rust.
* [Ripgrep Code Review](http://blog.mbrt.it/2016-12-01-ripgrep-code-review/) by [@mbrt](https://github.com/mbrt) - An analysis of the popular `ripgrep` tool's source code.
* [Rustic Bits](https://llogiq.github.io/2016/02/11/rustic.html) by [@llogiq](https://github.com/llogiq/) - Small things that make for rustic code.
* [Convenient and idiomatic conversions in Rust](https://ricardomartins.cc/2016/08/03/convenient_and_idiomatic_conversions_in_rust) by [meqif](https://github.com/meqif) - Explains `From<T>`, `Into<T>`, `TryFrom<T>`, `TryInto<T>`, `AsRef<T>` and `AsMut<T>` with pratical examples.


### 2015

* [Rust traits for developer friendly libraries](http://benashford.github.io/blog/2015/05/24/rust-traits-for-developer-friendly-libraries/) by [@benashford](https://github.com/benashford) - Thoughts about implementing good Rust libraries.
* [Error Handling in Rust](http://blog.burntsushi.net/rust-error-handling/) by [@BurntSushi](https://github.com/BurntSushi) - Understanding and handling errors in Rust in an idiomatic way.

## Talks

###  2017

Idiomatic Rust Libraries [[Video](https://www.youtube.com/watch?v=0zOg8_B71gE)] [[Slides](https://killercup.github.io/rustfest-idiomatic-libs/index.html#/)] by [@killercup](https://github.com/killercup)  - Rustfest Kiev  
What's so hard about writing a Slack Client in Rust? [[Video](https://www.youtube.com/watch?v=rrtJh1kz1Ms)] [[Slides](https://speakerdeck.com/mre/whats-so-hard-about-writing-a-slack-client-in-rust)] by [@mre](https://github.com/mre) - Rust Cologne Meetup

## Projects

* [clippy](https://github.com/Manishearth/rust-clippy) - A bunch of lints to catch common mistakes and improve your Rust code.
* [Patterns](https://github.com/nrc/patterns/) - A catalogue of Rust design patterns.
* [Rust by Example](http://rustbyexample.com/) - A community driven collection of example code which follow Rust best practices.
* [Rust Cookbook](https://github.com/brson/rust-cookbook) - Examples that demonstrate good practices to accomplish common programming tasks in Rust.
* [rust-api-guidelines](https://github.com/brson/rust-api-guidelines) - An extensive list of recommendations for idiomatic Rust APIs.

## History

Coming from Python I loved to have some guidelines on how "idiomatic Python" looks like. I was inspired by the likes of Peter Norvig, who wrote this amazing article about spellchecker for example: http://norvig.com/spell-correct.html.
And, of course, the [Zen of Python](https://www.python.org/dev/peps/pep-0020/)

The goal is to have a peer-reviewed collection of articles/talks/repos which teach idiomatic Rust style. It's a community project.
If this becomes a thing I plan to move it to some "semi-official" place like rust-nursery or so. But first I need YOUR help for that ;-)

## License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](http://matthias-endler.de) has waived all copyright and related or neighboring rights to this work.
Logo adapted from [FreePik.com](http://www.freepik.com/free-vector/crabs-pattern-design_1093131.htm).
