/*!

**Creating a Simple GUI**

In this chapter we'll use the conrod library to create a simple GUI.

*Before going on, we recommend that you check out the [Getting Started] chapter if you haven't
already.*


## Creating a New Project

First, we're going to use cargo to set up a new project for developing an executable called
`simple_gui`:

1. Open up your command line.
2. Ensure you are in the directory where you'd like to create the project.
3. Enter `cargo new --bin simple_gui`
4. `cd simple_gui`

Now we should be inside the directory of our new project with the following:

```txt
├── Cargo.toml
└── src
    └── main.rs
```


If we open up the `Cargo.toml` in our text editor, it should look like this:

```toml
[package]
name = "simple_gui"
version = "0.1.0"
authors = ["mitchmindtree <mitchell.nordine@gmail.com>"]

[dependencies]
```

Since we want to use conrod to make our GUI, we'll add it as a dependency under the
`[dependencies]` tag, like so:

```toml
[dependencies]
conrod = "0.31.2"
```

*Note: You should use the latest version of conrod - it just so happens that as of writing this
guide, the latest version is 0.31.2.*

Now we should be ready to start writing some Rust!


## Setting Up a Basic Window


[Getting Started]:  [../chapter_2/index.html]   "Chapter 2: Getting Started"

*/

