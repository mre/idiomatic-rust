<!--
⚠️⚠️ WARNING: This file is generated by `make render`. Do not edit manually!
See `CONTRIBUTING.md` for more information.
-->

<picture >
  <source media="(prefers-color-scheme: dark)" srcset="assets/idiomatic-rust-dark.png">
  <img src="assets/idiomatic-rust.png" />
</picture>

[![Check Links](https://github.com/mre/idiomatic-rust/workflows/Check%20Links/badge.svg)](https://github.com/mre/idiomatic-rust/actions/workflows/check_links.yml)

This repository collects resources for writing clean, idiomatic Rust code.
You can find a sortable/searchable version of this list [here](https://corrode.dev/idiomatic-rust/).

> _Idiomatic_ coding means following the conventions of a given language. It is
> the most concise, convenient, and common way of accomplishing a task in that
> language, rather than forcing it to work in a way the author is familiar with
> from a different language. - Adapted from [Tim
> Mansfield](https://github.com/tim-hr/stuff/wiki/Idiomatic-coding)

Contributions welcome! To add missing resources, [please refer to the contributing documentation](https://github.com/mre/idiomatic-rust/blob/master/CONTRIBUTING.md).

## ⚙ Projects

{% for project in projects %}

- [{{ project.title }}]({{ project.url }}) — {{ project.description }}
  {%- endfor %}

## 🏋 Workshops

{% for workshop in workshops %}

- [{{ workshop.title }}]({{ workshop.url }}) — {{ workshop.description }}
  {%- endfor %}

## 📖 Books

{% for book in books %}

- [{{ book.title }}]({{ book.url }}) — {{ book.description }}
  {%- endfor %}

## 📰 Articles

{% for (year, resources) in articles %}

### {{ year }}

{% for resource in resources -%}

- [{{ resource.title }}]({{ resource.url }}) — {{ resource.description }}
  {% endfor %}
  {%- endfor %}

## 🎤 Talks

{% for (year, resources) in talks %}

### {{ year }}

{% for resource in resources -%}

- {{ resource.title }} — {{ resource.description }} [[Video]({{ resource.url }})]
  {% endfor %}
  {%- endfor %}

## 💬 Forum

{% for (year, resources) in forum %}

### {{ year }}

{% for resource in resources -%}

- [{{ resource.title }}]({{ resource.url }})
  {% endfor %}
  {%- endfor %}

## 📜 History

Coming from Python, I loved the guidelines on how _idiomatic Python_ looks like. I was inspired by the likes of Peter Norvig, who wrote amazing articles on [spellcheckers](https://norvig.com/spell-correct.html) and [sudoku solvers](https://norvig.com/sudoku.html); and, of course, the [Zen of Python](https://www.python.org/dev/peps/pep-0020/). For Rust, there is no such thing as the Zen of Python, however, so I started collecting my own resources.
The goal of this project is to create a peer-reviewed collection of articles/talks/repos, which teach idiomatic Rust style. It's a community project and you can contribute.

## 🔏 License

[![CC0](https://i.creativecommons.org/p/zero/1.0/88x31.png)](https://creativecommons.org/publicdomain/zero/1.0/)

To the extent possible under law, [Matthias Endler](https://endler.dev) has waived all copyright and related or neighboring rights to this work.
Logo adapted from [FreePik.com](https://www.freepik.com).
