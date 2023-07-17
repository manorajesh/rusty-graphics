# threedee raycaster
A 3D raycaster built with Rust

![Demo picture](https://github.com/manorajesh/rusty-graphics/blob/threedee/images/demo1.png)
![Demo picture](https://github.com/manorajesh/rusty-graphics/blob/threedee/images/demo2.png)
![Demo picture](https://github.com/manorajesh/rusty-graphics/blob/threedee/images/demo3.png)

## Installation
```
git clone https://github.com/manorajesh/rusty-graphics.git && cd rusty-graphics
cargo run
```

## Usage
Use the arrow keys to traverse the extremely entertaining room. Fog adds that scary touch

## Why
I wanted to explore graphics programming with this being the stepping stone to `wgpu` and the world of GPU programming. This repo chronicles my long but fulfilling journey with 3D and 2D graphics.

#### Important Code
The [`draw`](https://github.com/manorajesh/rusty-graphics/blob/9a29953aac353d34af41111cc6ac0443a011c3f8/src/raycaster.rs#L151-L241) function is responsible for casting the rays and rendering them accordingly.
