<p align="center">
  <img width="200" src="img/logo.png" />
  <h2 align="center"> rust-top_message </h2>
  <p align="center">✨✨ Display message on top 😱😭</p>
</p>

<p align="center">
  <a>
    <img src="https://img.shields.io/github/stars/th2ch-g/rust-top_message"/>
  </a>
  <a>
    <img src="https://img.shields.io/github/license/th2ch-g/rust-top_message"/>
  </a>
  <a>
    <img src="https://github.com/th2ch-g/rust-top_message/actions/workflows/rust.yaml/badge.svg"/>
  </a>
  <a>
   <img src="https://img.shields.io/github/languages/top/th2ch-g/rust-top_message"/>
  </a>
  <a>
    <img src="https://img.shields.io/github/last-commit/th2ch-g/rust-top_message"/>
  </a>
  <a>
    <img src="https://img.shields.io/github/repo-size/th2ch-g/rust-top_message"/>
  </a>
  <a>
    <img src="https://img.shields.io/badge/rust-1.62.0+-blueviolet.svg?logo=rust"/>
  </a>
</p>


# rust-top_message
![GIF](img/rtm_example.gif)

- [rust-top\_message](#rust-top_message)
  - [Install](#install)
    - [Dependencies](#dependencies)
  - [Gallery](#gallery)
  - [Quick start \& Examples](#quick-start--examples)
    - [Single mode](#single-mode)
    - [Multiple mode](#multiple-mode)
    - [Multiple2 mode](#multiple2-mode)
    - [Long mode](#long-mode)
    - [Vertical mode](#vertical-mode)
    - [Wave mode](#wave-mode)
    - [Check mode](#check-mode)

## Install
~~~
cargo install --git https://github.com/th2ch-g/rust-top_message.git
~~~
### Dependencies
- [Rust](https://www.rust-lang.org/tools/install) >= 1.62.0
    - rust-top_message requires Rust environment

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

## Quick start & Examples
<a id="single"></a>
### Single mode
~~~
rust-top_message single -m hello_world -@ 4 -t 20 & top
~~~

<a id="multiple"></a>
### Multiple mode
~~~
rust-top_message multiple -m hello_world -@ 4 -t 20 & top
~~~

<a id="multiple2"></a>
### Multiple2 mode
~~~
rust-top_message multiple2 -m "hello_world1 hello_world2 hello_world3" -t 20 & top
~~~

<a id="long"></a>
### Long mode
~~~
rust-top_message long -m Rust_is_the_greatest_and_best_programming_language_ever -t 20 & top
~~~

<a id="vertical"></a>
### Vertical mode
~~~
rust-top_message vertical -m "ThankYou GoodLuck" -t 20 & top
~~~

### Wave mode
~~~
rust-top_message wave -m 123456789 -@ 4 & top
~~~

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