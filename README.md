This is a Rust port of [Zenibou](https://github.com/Nostress767/Zenibou) (which is made in C).

Firstly, get [Visual Studio's Build Tools For C/C++ Development](https://visualstudio.microsoft.com/downloads/?q=build+tools) if you don't have it already to be able to compile for windows.

Then, make sure you have the i686-pc-windows-msvc target, if you don't, just run:

    rustup target add i686-pc-windows-msvc

Now this should just work:

    cargo run

If you wish to compile without the std library, just uncomment the lines marked on cargo.toml, and run as usual.

This program was made using the nightly channel (with rustc on version 1.61.0-nightly), but it shouldn't make too much of a difference.
