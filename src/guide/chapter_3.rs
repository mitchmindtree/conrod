/*!

**Creating a Simple GUI**

In this chapter we'll use the conrod library to create a simple GUI.

*Before going on, we recommend that you check out the [Getting Started] chapter if you haven't
already.*


## Creating a New Project

First, we're going to use cargo to set up a new project for developing an executable called
`simple_gui`:

### `cargo new --bin simple_gui`

1. Open up your command line.
2. Ensure you are in the directory where you'd like to create the project.
3. Enter `cargo new --bin simple_gui`
4. `cd simple_gui`

Now we should be inside the directory of our new project that contains the following:

```txt
├── Cargo.toml
└── src
    └── main.rs
```

### Adding Assets

As a GUI designer, you'll want to use your own fonts and images with conrod. We'll make a new
directory called `assets` within our `simple_gui` directory so that we have somewhere to put these.

**Fonts**

Now we'll add a font to our project. Before doing so, we'll make a directory called `fonts` in our
`assets` directory.

Conrod should be compatible with any font formats that FreeType supports, however for this
demonstration we'll use one of the fonts that comes with the conrod repository as we know that it
works. You can find the `NotoSans-Regular.ttf` font [here][Conrod NotoSans Fonts]. We'll add it to
our newly created `fonts` directory.

**Images**

*Coming Soon*


### Editing the Cargo.toml


If we open up the `Cargo.toml` in our text editor, it should look like this:

```toml
[package]
name = "simple_gui"
version = "0.1.0"
authors = ["your_name <your_email@address.com>"]

[dependencies]
```

We'll add the dependencies that we choose to use under the `[dependencies]` tag:

- `conrod` - for our GUI!
- `find_folder` - to find our assets folder regardless of where our program is executed from.
- `piston_window` - to provide a simple window with events and a basic graphics context using
opengl. *Note: conrod is in fact generic over its window and graphics backends; we'll explain
this further in a later chapter*.

We should now have this at the bottom of our `Cargo.toml`:

```toml
[dependencies]
conrod = "0.31.2"
find_foler = "0.3.0"
piston_window = "0.34.0"
```

*Note: You should use the latest version of these dependencies if you can - it just so happens that
as of writing this guide, the latest versions are as above.*

Now we should be ready to start writing some Rust!


## Setting Up a Basic Window

It's time to open up the src/main.rs file in your text editor. Remove the "hello, world!" line and
add the crates that we depend on like so:

```
#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

fn main() {
}
```

Note the `#[macro_use]` attribute before the conrod crate. This is so that we can use a macro
provided by conrod for statically generating `WidgetId`s - but more on that later.

We'll instantiate a `PistonWindow`:

```
# #[macro_use] extern crate conrod;
# extern crate find_folder;
# extern crate piston_window;
use piston_window::{PistonWindow, WindowSettings};

fn main() {
    // Construct the window.
    let window: PistonWindow =
        WindowSettings::new("Simple GUI", [800, 600])
            .exit_on_esc(true) // Finish the event loop when `Esc` is pressed
            .vsync(true) // Turn on vertical sync https://en.wikipedia.org/wiki/Screen_tearing#V-sync
            .samples(4) // Use 4 samples when anti-aliasing https://en.wikipedia.org/wiki/Multisample_anti-aliasing
            .build() // Build the `PistonWindow` from these settings.
            .unwrap(); // Don't worry about handling errors for now.
}
```

You'll notice that the `WindowSettings` uses a chain of optional methods ending in a call to
`.build()` - for more information on this pattern, see [the Builder Pattern] section in chapter 1.

Now that we've built our `window`, we'll want to run it in a loop



[Getting Started]:          ../chapter_2/index.html                         "Chapter 2: Getting Started"
[Conrod NotoSans Fonts]:    https://github.com/PistonDevelopers/conrod/tree/master/assets/fonts/NotoSans    "NotoSans"
[the Builder Pattern]:      ../chapter_1/index.html#the-builder-pattern     "The Builder Pattern"

*/

