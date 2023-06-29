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

![Screenshot 1](https://i.imgur.com/6Z2ZQ8m.png)

![Screenshot 2](https://i.imgur.com/6Z2ZQ8m.png)

![Screenshot 3](https://i.imgur.com/6Z2ZQ8m.png)

![Screenshot 4](https://i.imgur.com/6Z2ZQ8m.png)

