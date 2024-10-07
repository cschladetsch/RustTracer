# Rust Raytracer

A simple, yet powerful raytracer implemented in Rust. This project demonstrates basic raytracing techniques including reflection, soft shadows, and specular highlights.

## Features

- Raytracing with configurable bounce depth
- Reflective surfaces
- Soft shadows
- Specular highlights
- Ambient occlusion
- Customizable scene with multiple spheres
- Checkerboard ground plane
- Gradient sky background
- Multi-sampling for antialiasing
- BMP image output

## Requirements

- Rust (latest stable version)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/rust-raytracer.git
   cd rust-raytracer
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the raytracer with:

```
cargo run --release -- [max_bounces]
```

Where `[max_bounces]` is an optional parameter to set the maximum number of light bounces (default is 10).

Example:
```
cargo run --release -- 15
```

This will generate an `output.bmp` file in the project directory.

## Customization

You can customize the scene by modifying the `main.rs` file:

- Adjust the number and properties of spheres
- Change the camera position
- Modify the checkerboard scale
- Alter the sky gradient colors

For more advanced modifications, explore the `raytracer.rs` file to adjust lighting, shadows, and other rendering parameters.

## Performance

The raytracer is designed to be relatively efficient, but rendering times may vary depending on your hardware and the complexity of the scene. Increase the `samples` value in `main.rs` for higher quality output at the cost of longer rendering times.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source and available under the [MIT License](LICENSE).
