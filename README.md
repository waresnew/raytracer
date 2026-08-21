# Raytracer

This program simulates realistic lighting through raytracing. For each ray that hits the camera, the program traces the path it took to get there while accumulating its attenuation.

Both a CPU (Rust) and GPU (WESL) implementation are provided.

## GPU implementation notes

- The shaders are written in WESL, which is a superset of WGSL that has nice features like file splitting and namespaces. However, the lack of IDE tooling for WESL made it very easy to make syntax mistakes.
- To run BVH queries on the GPU, the tree is converted to a 1D array, and a stack is used to simulate recursion.
- The CPU implementation uses enums (tagged unions) extensively. WGSL does not have tagged unions, so structs with shared fields are used to emulate this.

## Usage

```text
Usage: raytracer [OPTIONS]

Options:
  -o, --output <OUTPUT>
          Specify an output file name. If this is not provided then the output image will be printed to the terminal
  -c, --cpu
          Use the CPU for raytracing instead of the GPU
  -s, --scene <SCENE>
          Which scene to render [default: cornell-box] [possible values: cornell-box, cornell-box-glass, random-balls, mixed-light]
      --gpu-chunk-height <GPU_CHUNK_HEIGHT>
          When GPU mode is used, each dispatch will process image_width*(this parameter) pixels. Reduce this to avoid your computer freezing at the cost of slower runtimes. If this parameter is not set, then the program will process as many pixels as a storage buffer can hold for each dispatch
  -v, --verbose...
          Increase logging verbosity
  -q, --quiet...
          Decrease logging verbosity
  -h, --help
          Prints this help message
  -V, --version
          Print version
```

## Showcase

Runtimes were measured on an M4 Macbook Air and the GPU implementation was used.

### Cornell Box

<table>
  <tr>
    <td>Image resolution</td>
    <td>800x800</td>
  </tr>
  <tr>
    <td>Samples per pixel</td>
    <td>10,000</td>
  </tr>
  <tr>
    <td>Max depth</td>
    <td>8</td>
  </tr>
  <tr>
    <td>Total rays</td>
    <td>29,818,764,244</td>
  </tr>
  <tr>
    <td>Execution time</td>
    <td>2m 52s</td>
  </tr>
  <tr>
    <td>Preview</td>
    <td>
      <img src="https://github.com/waresnew/raytracer/releases/download/examples/cornell-box.png">
      <br>
      Standard Cornell box.
    </td>
  </tr>
</table>

### Cornell Box Glass

<table>
  <tr>
    <td>Image resolution</td>
    <td>800x800</td>
  </tr>
  <tr>
    <td>Samples per pixel</td>
    <td>10,000</td>
  </tr>
  <tr>
    <td>Max depth</td>
    <td>8</td>
  </tr>
  <tr>
    <td>Total rays</td>
    <td>31,341,678,660</td>
  </tr>
  <tr>
    <td>Execution time</td>
    <td>2m 31s</td>
  </tr>
  <tr>
    <td>Preview</td>
    <td>
      <img src="https://github.com/waresnew/raytracer/releases/download/examples/cornell-box-glass.png">
      <br>
      Cornell box but the cube is replaced with a glass sphere and the light is slightly tinted.
    </td>
  </tr>
</table>

### Random Balls

<table>
  <tr>
    <td>Image resolution</td>
    <td>800x450</td>
  </tr>
  <tr>
    <td>Samples per pixel</td>
    <td>5,000</td>
  </tr>
  <tr>
    <td>Max depth</td>
    <td>16</td>
  </tr>
  <tr>
    <td>Total rays</td>
    <td>5,374,862,952</td>
  </tr>
  <tr>
    <td>Execution time</td>
    <td>1m 45s</td>
  </tr>
  <tr>
    <td>Preview</td>
    <td>
      <img src="https://github.com/waresnew/raytracer/releases/download/examples/random-balls.png">
      <br>
      403 spheres with diffuse, metal, and glass materials.
    </td>
  </tr>
</table>

### Mixed Light

<table>
  <tr>
    <td>Image resolution</td>
    <td>800x450</td>
  </tr>
  <tr>
    <td>Samples per pixel</td>
    <td>100,000</td>
  </tr>
  <tr>
    <td>Max depth</td>
    <td>50</td>
  </tr>
  <tr>
    <td>Total rays</td>
    <td>68,595,210,026</td>
  </tr>
  <tr>
    <td>Execution time</td>
    <td>1m 28s</td>
  </tr>
  <tr>
    <td>Preview</td>
    <td>
      <img src="https://github.com/waresnew/raytracer/releases/download/examples/mixed-light.png">
      <br>
      Basic arrangement of red, green, and blue light sources to demonstrate colour mixing.
    </td>
  </tr>
</table>
