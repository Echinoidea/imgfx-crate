# imgfx

Provides configurable low-level operations (arithmetic, logical, bitwise) and high-level
operations (bloom, blending modes, pixel sorting) for images.

Functions:
- add, sub, mult, div
- bitshift left | right
- and, or, xor (and their complements with -n flag)
- screen, overlay, average
- bloom

All functions support operand reordering, allowing precise control over how the image's color channels
are processed. Operand reordering lets you redefine how the source image's R, G, and B channels
are mapped during the operation.

Each function accepts an image::DynamicImage and returns an image::RgbaImage.

```rust
use imgfx::{add};

let img = image::open(path).expect("Failed to open image.");

// Add each pixel's color, mapped as R R B and FF0000.
let output = add(img, ["r".to_string(), "r".to_string(), "b".to_string()], None, RgbColor(255, 0, 0))
```

Future developments:
- Pixel sorting by some condition like hue, luminance, etc
- Option to make bloom faster for use in videos

