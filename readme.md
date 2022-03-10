# FFMPEG UI

I'm using this project to learn WASM and improve my Rust.

- [FFMPEG UI](#ffmpeg-ui)
    - [Dependencies](#dependencies)
    - [Rust](#rust)
    - [Yew](#yew)
    - [FFmpeg for wasm](#ffmpeg-for-wasm)
      - [Install a container runtime](#install-a-container-runtime)
      - [Run build](#run-build)
      - [Bindgen headers](#bindgen-headers)
  - [Running](#running)
    - [Note for the future](#note-for-the-future)

### Dependencies

### Rust

Install rust for your OS how you see fit, currently there are no OS requirements.

<https://www.rust-lang.org/tools/install>

This app is currently developed with the following rust version: `rustc 1.59.0 (9d1b2106e 2022-02-23)`

### Yew

<https://yew.rs/> Yew is the frontend framework, it's entirely rust and you probably will not see any `.js` files in the project.

Follow the [install instructions](https://yew.rs/docs/getting-started/introduction) and make sure to add the compile option to rust with:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

### FFmpeg for wasm

ffmpeg is not included in the repo as it libraries can be pretty heavy

#### Install a container runtime

To make our life easier we build using emsdk image from dockerhub. 
It's possible to build ffmpeg manualy but it can be painfull.

The commands showcase podman but docker works as well.

#### Run build

```
podman build -t ffmpeg-wasm .
```

FFmpeg is built during the build time, but to get the files into our file system you will need to run the container.

```bash
# The default cmd will copy the required files to src/ffmpeg
podman run -v "$(pwd)"/src/ffmpeg/:/out -it ffmpeg-wasm
```

This should result in a `ffmpeg` folder inside `/src/ffmpeg`. We still have more preparations to do.

In the generated ffmpeg folder, the libraries are in `/lib/*.a` move this folder up: `mv src/ffmpeg/ffmpeg/lib src/ffmpeg`

#### Bindgen headers

The bindings are commited in the repo so this can be skipped unless you want to regenerate then.

Then install bindgen:

```bash
# Make sure you have the requirements in your system https://rust-lang.github.io/rust-bindgen/requirements.html
cargo install bindgen
```

Now we need to generate the bindinges for all APIs we wanna use.

```bash
# x264 might not work this way as the required system lib is not explicitly included in the original header file.
bindgen src/ffmpeg/x264_wrapper.h -o src/ffmpeg/x264.rs -- -Isrc/ffmpeg/ffmpeg/include 
for lib in 'avcodec' 'avfilter' 'avformat' 'avutil' 'swresample' 'swscale'; do
  bindgen src/ffmpeg/ffmpeg/include/lib$lib/$lib.h -o src/ffmpeg/$lib.rs -- -Isrc/ffmpeg/ffmpeg/include
done
```

Note that after the bindgen, avlibs will have illegal code for rust ( duplicating variable declaration ), you will have to manually delete then.

## Running

Just type: `trunk serve` and you should be good to go :)


### Note for the future

FFmpeg will require SharedArrayBuffer which needs custom headers in the server, `trunk` still doesn't have a way to set headers so [server.rs](./server.rs) can be used instead.
To use it you will install `rust-script`:

```
cargo install rust-script

# might not work under Windows as it depends on the sh hashbang to find rust-script executable.
./server.rs

# In another terminal session
trunk watch
```
