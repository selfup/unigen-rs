# OXIDIZY

An ambitious Open Source project to mimic the smaller things in life.

The idea is to build a cli tool that can generate enough life, and *sublife* (neutrons, protons, electrons), that when you give the tool atomic weights, it can "generate" appropriate elements.

Another ambitious goal, would be to set up bonding logic to create water or other compounds.

The most difficult part will be setting up an electro-magnetic field. That is why we are starting with the simplest form of an EMF.

# Rust

### OSX

If you are using a Mac (OSX), please use `homebrew` to install rust.

`brew install rust`

If you prefer to not use `homebrew`, please install the official stable binaries.

`curl -sSf https://static.rust-lang.org/rustup.sh | sh`

### Linux

If you are running Linux, you may install the official stable binaries.

`curl -sSf https://static.rust-lang.org/rustup.sh | sh`

### Windows

If you are using Win 7, 8, or 8.1, upgrade to Windows 10.

If you are running 10, go to this [PAGE](https://www.rust-lang.org/downloads.html#win-foot)

There are two different builds. Visual Studio ready builds, and GNU gcc builds.

The GNU gcc build is the more mature one.

Try to use the GNU build.

```
There are two prominent ABIs in use on Windows: the native (MSVC) ABI used by Visual Studio, and the GNU ABI used by the GCC toolchain. Which version of Rust you need depends largely on what C/C++ libraries you want to interoperate with: for interop with software produced by Visual Studio use the MSVC build of Rust; for interop with GNU software built using the MinGW/MSYS2 toolchain use the GNU build.

MSVC builds of Rust additionally require an installation of Visual Studio 2013 (or later) so rustc can use its linker. Make sure to check the "C++ tools" option. No additional software installation is necessary for basic use of the GNU build.

Rust's support for the GNU ABI is more mature, and is recommended for typical uses.
```
