# rust-top_message
Display message on top (all written in Rust)

![gif_example](img/multiple.gif)

- [rust-top_message](#rust-top_message)
  - [Dependencies](#dependencies)
  - [Install rust-top_message](#install-rust-top_message)
    - [1. Install by cargo](#1-install-by-cargo)
    - [2. Install from source](#2-install-from-source)
  - [Gallery](#gallery)
  - [Subcommand](#subcommand)
  - [Quick start & Examples](#quick-start--examples)
    - [Single mode](#single-mode)
    - [Multiple mode](#multiple-mode)
    - [Multiple2 mode](#multiple2-mode)
    - [Long mode](#long-mode)
    - [Vertical mode](#vertical-mode)
    - [Wave mode](#wave-mode)
    - [Check mode](#check-mode)
  - [Recommend alias](#recommend-alias)



## Dependencies
- [Rust](https://www.rust-lang.org/tools/install) >= 1.62.0
    - rust-top_message requires Rust environment
    - [How should I install Rust ?](rust-how-to-install.md)


## Install rust-top_message
There are 2 ways to install.


### 1. Install by cargo
~~~
cargo install --git https://github.com/th2ch-g/rust-top_message.git
~~~
The executable file is in `~/.cargo/bin/rust-top_message`


### 2. Install from source
~~~
git clone https://github.com/th2ch-g/rust-top_message.git && \
cd ./rust-top_message && \
cargo build --release
~~~
The executable file is in `./target/release/rust-top_message`


## Gallery
<a href=#single>
    <img src="img/single.png" class="galleryItem" width=200px></img>
</a>

<a href=#multiple>
    <img src="img/multiple.png" class="galleryItem" width=200px></img>
</a>

<a href=#multiple2>
    <img src="img/multiple2.png" class="galleryItem" width=200px></img>
</a>

<a href=#long>
    <img src="img/long.png" class="galleryItem" width=200px></img>
</a>

<a href=#vertical>
    <img src="img/vertical.png" class="galleryItem" width=200px></img>
</a>



## Subcommand
~~~
USAGE:
    rust-top_message <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    single       one message on one top
    multiple     one message on many top
    multiple2    many message on many top
    long         one long message on many top with newline
    vertical     message on many top vertically
    wave         one message on many top like electric bulletin board
    check        check if top_message can be executed normally
    help         Print this message or the help of the given subcommand(s)
~~~


## Quick start & Examples

<a id="single"></a>
### Single mode
~~~
$ rust-top_message single -m hello_world -@ 4 -t 20 & top
~~~


<a id="multiple"></a>
### Multiple mode
~~~
$ rust-top_message multiple -m hello_world -@ 4 -t 20 & top
~~~


<a id="multiple2"></a>
### Multiple2 mode
~~~
$ rust-top_message multiple2 -m "hello_world1 hello_world2 hello_world3" -t 20 & top
~~~



<a id="long"></a>
### Long mode
~~~
$ rust-top_message long -m Rust_is_the_greatest_and_best_programming_language_ever -t 20 & top
~~~


<a id="vertical"></a>
### Vertical mode
~~~
$ rust-top_message vertical -m "ThankYou GoodLuck" -t 20 & top
~~~


### Wave mode
~~~
$ rust-top_message wave -m 123456789 -@ 4 & top
~~~

Check by yourself!


### Check mode

check if the directories created by rust-top_message remains in current directory
~~~
$ rust-top_message check --onlycheck

[CHECK] rustc seems to be installed
[CHECK] .tmp_20220731131724_487375867_68549 is seems to be directory created by rust-top_message
[CHECK] .tmp_20220731131730_903474437_68694 is seems to be directory created by rust-top_message
[CHECK] If you want to delete these directory, please execute --rmcheck option
~~~


check the directory created by rust-top_message && remove those directory
~~~
$ rust-top_message check --rmcheck

[CHECK] rustc seems to be installed
[CHECK] .tmp_20220731131724_487375867_68549 is seems to be directory created by rust-top_message
[CHECK] .tmp_20220731131730_903474437_68694 is seems to be directory created by rust-top_message
[REMOVED] All directories that rust-top_message may have created were successfully deleted
~~~


## Recommend alias

Since rust-top_message is a long program name, it is recommended to register alias on your ~/.zshrc and so on.

~~~
alias rtm="[Your_rust-top_message_realpath]"
~~~


