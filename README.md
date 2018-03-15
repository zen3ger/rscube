# RsCube

Simple 3x3x3 Rubik's cube simulator written in Rust.

<p align="center">
	<img
		style='height: 60%; width: 60%;'
		src="https://github.com/zen3ger/rscube/blob/master/assets/screenshot.png"
		alt="RsCube screenshot"
	>
</p>

## Usage:
- Zoom `[` and `]` keys
- Reset cube with `<C-r>`
- Scramble cube with `<C-s>`
- Execute move with `<Space>`
- Exit `<Esc>`

### How to turn the faces?
You can type any of `U`, `D`, `R`, `L`, `F`, `B` upper- or lowercase.
The letters correnspond to the Up, Down, Right, Left, Front
and Back faces of the cube.

### How to turn slices?
You can type `M`, `E` or `S` upper- or lowercase to turn the Middle,
Equitorial or Side slices.

### Turn modifiers
To invert the direction of face rotation type either `'` or `i`.
For example: `Ri Mi`

If you want to do a 180° turn on any face type `2` after the
letter marking the face.
For example: `M2 U2 M2 U2`

To rotate a face plus a splice type `w` after the letter marking the face.
For example: `Fw`

## Implemented:
- 2D cube rendering
- Simple turn parsing with inverted, double and wide turns

## On the TODO list
- [ ] algorithm macro definitions in REPL or in profile file
- [ ] loading custom color definitions from profile file
- [ ] saving solves
- [ ] nicer looking cube
- [ ] a real REPL ?
