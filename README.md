# Fil de Fer
## Wireframe modeler

This is a recoded version of one of my projects in 42 school, but in Rust. It consists of a program that takes a map file and renders it's 3D representation.

This project uses SDL2 for window management and pixel printing.

### Usage
```
cargo run --release -- <map_file>
```

### Map file format

The map file must be a text file containing only numbers, with the following format:
```
0 0 0 0 0 0 0 0 0 0
0 1 1 1 1 1 1 1 1 0
0 1 2 2 2 2 2 2 1 0
0 1 2 3 3 3 3 2 1 0
0 1 2 3 4 4 3 2 1 0
0 1 2 3 4 3 3 2 1 0
0 0 0 0 0 0 0  0 0 0
```
Where each number represents a height, and the number of numbers in each line represents the width of the map.

### Controls

- `WASD` to move around
- `R` to reset the camera
- `ESC` to quit
- Arrow keys to scale the map

### Screenshots

![Capturar](https://github.com/MarcosFlavioGS/Fil_de_Fer_Rust/assets/95108526/d932e3ca-657c-4315-9fcb-9c1773dd0ae8)

![Capturar1](https://github.com/MarcosFlavioGS/Fil_de_Fer_Rust/assets/95108526/e6bf9773-ad4c-406d-8b0d-50601f7af3b9)

![42](https://github.com/MarcosFlavioGS/Fil_de_Fer_Rust/assets/95108526/49fe8624-b4f5-4f85-bf56-4f42960fc78a)

![pylone](https://github.com/MarcosFlavioGS/Fil_de_Fer_Rust/assets/95108526/fe1fc6bd-9b92-4e61-a34a-dd5070497829)
