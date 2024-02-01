# Contributing to Idiomatic Rust

## Introduction

Thank you for your interest in contributing to Idiomatic Rust! This guide will
help you understand how to contribute effectively.

## `README` is Auto-Generated

Please note that the `README.md` file in this repository is auto-generated
*using a
Rust script. **Do not edit the `README.md` file directly.** Instead, make
changes to the `resources.json` file or the Rust script.

### Generating the README

To generate the README, ensure that Rust is installed on your machine. You can
then run the following command:

```sh
make render
```

This will update the `README.md` file based on the current contents of
`resources.json`.

### Adding Resources to `resources.json`

`resources.json` is the primary file where resources about idiomatic Rust are
listed. The file is structured in a JSON array, where each resource is an object
with specific fields.

### File Structure

Each resource object in `resources.json` should have the following structure:

```json
{
  "title": "Resource Title",
  "url": "https://resource.url",
  "description": "A brief description of the resource.",
  "tags": ["tag1", "tag2"],
  "official": true or false,
  "year": YearOfPublication,
  "difficultyLevel": "beginner | intermediate | advanced",
  "duration": "time duration (if audio/video) or null",
  "interactivityLevel": "low | medium | high",
  "free": true or false,
  "category": "project | workshop | book | article | talk | forum"
}
```

Check out the file for some examples.
Please ensure that your addition adheres to this format for consistency.

### Adding a New Resource

1. Fork the repository.
2. Add your resource to `resources.json`, following the structure above.
3. Run `make render` to locally to generate the README.
4. Create a pull request with your changes.

### Pull Requests

When you submit a pull request, please include the following:

* A brief explanation of the resource and why it's beneficial for idiomatic
  Rust.
* Confirmation that the resource links and information are current and valid.

### Questions or Issues

If you have any questions or encounter any issues, please open an issue in the
repository, and we'll get back to you as soon as possible.

Thank you for contributing to making Rust programming more idiomatic and
accessible!
