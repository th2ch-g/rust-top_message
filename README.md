<p align="center">
  <img width="200" src="img/logo.png" />
  <h2 align="center"> rtm_mpi </h2>
  <p align="center">✨✨ rust-top_message OpenMPI version: Display message on top 😱😭</p>
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


# rtm_mpi
- rtm_mpi is rust-top_message OpenMPI version

![GIF](img/rtm_example.gif)

- [rtm\_mpi](#rtm_mpi)
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
# 1. rtm_mpi build
git clone -b mpi_dev https://github.com/th2ch-g/rust-top_message.git && \
cd rust_top_message && \
cargo build -r

# 2. load openmpi, intel-mpi (If you don't have module command, please install these dependencies)
module load openmpi intel-mpi
~~~

### Dependencies
- [Rust](https://www.rust-lang.org/tools/install) >= 1.62.0
    - rust-top_message requires Rust environment
- OpenMPI >= 4.0.3
- Intel-MPI >= 19.0

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
mpirun -np 4  single -m hello_world -@ 4 -t 20 & top
~~~

<a id="multiple"></a>
### Multiple mode
~~~
mpirun -np 4 rtm_mpi multiple -m hello_world -@ 4 -t 20 & top
~~~

<a id="multiple2"></a>
### Multiple2 mode
~~~
mpirun -np 4 rtm_mpi multiple2 -m "hello_world1 hello_world2 hello_world3" -t 20 & top
~~~

<a id="long"></a>
### Long mode
~~~
mpirun -np 4 rtm_mpi long -m Rust_is_the_greatest_and_best_programming_language_ever -t 20 & top
~~~

<a id="vertical"></a>
### Vertical mode
~~~
mpirun -np 4 rtm_mpi vertical -m "ThankYou GoodLuck" -t 20 & top
~~~

### Wave mode
~~~
mpirun -np 4 rtm_mpi wave -m 123456789 -@ 4 & top
~~~

### Check mode
check if the directories created by rust-top_message remains in current directory
~~~
$ rtm_mpi check --onlycheck
[CHECK] rustc seems to be installed
[CHECK] .tmp_20220731131724_487375867_68549 is seems to be directory created by rtm_mpi
[CHECK] .tmp_20220731131730_903474437_68694 is seems to be directory created by rtm_mpi
[CHECK] If you want to delete these directory, please execute --rmcheck option
~~~

check the directory created by rust-top_message && remove those directory
~~~
$ rtm_mpi check --rmcheck
[CHECK] rustc seems to be installed
[CHECK] .tmp_20220731131724_487375867_68549 is seems to be directory created by rtm_mpi
[CHECK] .tmp_20220731131730_903474437_68694 is seems to be directory created by rtm_mpi
[REMOVED] All directories that rtm_mpi may have created were successfully deleted
~~~