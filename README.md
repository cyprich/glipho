# Rusty Fotos

Simple image manipulation software to make glitchy photos

## About

With this ultra amazing program, you can apply various effects and glitches to images

### Layers

In this table, you can see all currently supported layer types

| Before                                                                     | After                                                                               | Layer                | Expected values | Description                                                                                                                                   |
| -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | -------------------- | --------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| [](https://github.com/cyprich/rusty_fotos/blob/master/examples/sample.jpg) | [](https://github.com/cyprich/rusty_fotos/blob/master/examples/brightness.jpg)      | Brightness (64)      | `-255` to `255` | Make image brighter/darker                                                                                                                    |
| [](https://github.com/cyprich/rusty_fotos/blob/master/examples/sample.jpg) | [](https://github.com/cyprich/rusty_fotos/blob/master/examples/wrap_brightness.jpg) | Wrap Brightness (64) | `-255` to `255` | Make image brighter/darker. When pixel value exceeds maximal value (`255`) it "wraps" around, making the brightest pixels the most dark       |
| [](https://github.com/cyprich/rusty_fotos/blob/master/examples/sample.jpg) | [](https://github.com/cyprich/rusty_fotos/blob/master/examples/invert.jpg)          | Invert               | None            | Inverts the image colors                                                                                                                      |
| [](https://github.com/cyprich/rusty_fotos/blob/master/examples/sample.jpg) | [](https://github.com/cyprich/rusty_fotos/blob/master/examples/reverse_bits.jpg)    | Reverse Bits         | None            | Reverse the order of bits in each pixel. Looks at pixel like `01000001` and makes it `10000010`                                               |
| [](https://github.com/cyprich/rusty_fotos/blob/master/examples/sample.jpg) | [](https://github.com/cyprich/rusty_fotos/blob/master/examples/min.jpg)             | Min                  | `0` to `255`    | Applies minimal threshold to all pixels. If the pixel is darker than threshold, it will be changed to the threshold value, making it brighter |
| [](https://github.com/cyprich/rusty_fotos/blob/master/examples/sample.jpg) | [](https://github.com/cyprich/rusty_fotos/blob/master/examples/max.jpg)             | Max                  | `0` to `255`    | Applies maximal threshold to all pixels. If the pixel is brighter than threshold, it will be changed to the threshold value, making it darker |

Comes in two variants

- CLI
- GUI

## Usage

### CLI

You can run the program in two ways

- Script mode (default)
- Interactive mode

### GUI

GUI version via [Dioxus](https://dioxuslabs.com/) coming soon
