subatomic
=========

subatomic is a small toy OS, written in Rust (with a bit of x86 ASM where it's needed).

It's roughly based on [intermezzOS](https://github.com/intermezzOS/kernel), [Philipp Oppermann's blog_os](https://github.com/phil-opp/blog_os), and [toyos-rs](https://github.com/emk/toyos-rs).

Currently, it just has a simple VGA driver and polling keyboard driver, but more stuff is being worked on :)

To run it, use `make run`. You'll need the following to be installed:

* Rust nightly (I suggest using `rustup`)
* xargo (via `cargo install xargo`)
* nasm
* ld
* grub-mkrescue
* xorriso
* qemu

Here's what it looks like:

![screenshot](https://github.com/wesleyac/subatomic/raw/master/docs/screenshot.png "Basically like every other toy OS :)")
