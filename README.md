# glipho

Software to create **gli**tchy **pho**tos

> [!NOTE]  
> This project is still in early stage of development, so lot of the stuff mentioned here might not work (yet!)

## About

With this ultra amazing very cool program, you can apply various effects and glitches to images  
Program loads an image, looks at it's pixels as numbers (`0` to `255`) and then applies different operations to each number  
These operations are called `Steps`

### Steps

Steps define sequence of actions, which will be applied to your image  
One step can be either `Layer` (applies an effect) or `Save` (saves to a file)

### Layers

Layers are kind of effects that are applied to your image  
In this table, you can see all currently supported layer types

| Before                     | After                               | Layer                | Expected values | Description                                                                                                                             |
| -------------------------- | ----------------------------------- | -------------------- | --------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| ![](./examples/sample.jpg) | ![](./examples/brightness.jpg)      | Brightness (64)      | `-255` to `255` | Make image brighter/darker                                                                                                              |
| ![](./examples/sample.jpg) | ![](./examples/wrap_brightness.jpg) | Wrap Brightness (64) | `-255` to `255` | Make image brighter/darker. When pixel value exceeds maximal value (`255`) it "wraps" around, making the brightest pixels the most dark |
| ![](./examples/sample.jpg) | ![](./examples/invert.jpg)          | Invert               | _None_          | Inverts the image colors                                                                                                                |
| ![](./examples/sample.jpg) | ![](./examples/reverse_bits.jpg)    | Reverse Bits         | _None_          | Reverse the order of bits in each pixel. Looks at pixel like `01000001` and makes it `10000010`                                         |
| ![](./examples/sample.jpg) | ![](./examples/min.jpg)             | Min (192)            | `0` to `255`    | Applies minimal threshold to all pixels.                                                                                                |
| ![](./examples/sample.jpg) | ![](./examples/max.jpg)             | Max (64)             | `0` to `255`    | Applies maximal threshold to all pixels.                                                                                                |

By combining these steps, you can achieve cool effects, for example...

1. `Max(150)`
2. `Min(50)`
3. `Invert`
4. `ReverseBits`
5. `WrapBrightness(-30)`
6. `ReverseBits`
7. `Invert`

...gave this result

![example1](./examples/example1.jpg)

These might not be explained too well, it's best to try it yourself :)

### Save

Yeah like there is not much to say about Save, it just saves the image to file

## Usage

Program comes in two variants

- CLI - console app - lightweight, best for automated scripts, with possibility of interactive mode
- GUI - desktop app or web app - easy use

### CLI

You can run it either via `cargo run --bin cli` or use precompiled binary from [Releases page](https://github.com/cyprich/glipho/releases)

You can specify multiple parameters

| Short format | Long format     | Description                            |
| ------------ | --------------- | -------------------------------------- |
| `-f`         | `--image-file`  | Path of file, which you want to modify |
| `-s`         | `--steps-file`  | Path of file containing steps          |
| `-i`         | `--interactive` | Run interactively                      |
| `-h`         | `--help`        | Show help                              |

So basically you have two ways of running the program - Interactively (with `-i`) or Non-interactively (without `-i`)  
If you are running Non-interactively, you **have to** specify the `-f` and `-s` files, otherwise the program will have nothing to do and will crash

If you are running it via cargo, you can specify the parameters like this: `cargo run --bin cli -- -i`

### GUI

We have GUI version via [Slint](https://slint.dev/)!

Just download the version for your operating system from the [Releases page](https://github.com/cyprich/glipho/releases)

You can compile and run it manually with `cargo run --bin gui`

## TODOs and Future plans

- Make it work :)
- Parallel processing
- Audio Layers
- GPU acceleration
