# TASK: Implement a UX for entering in solutions to puzzles. 

## Revise the UI to be arranged around a square. 

Examine the screenshot in @NY_Times_Letter_Boxed.png. Note how the interface includes:

- An arrangement of a four-sided box
- Each side is one side of the box.
- Letters remain oriented normally.

Your task is to remove the four fields for inputting sides, and instead have an interface that resembles the Letter Boxed board as above.

There will be separate fields for every letter. Typing one letter into a field immediately advances the user to the next field. Typing backspace will allow the user to go back to a previous field.

The order of fields is this. The fields are named with the format charXX, where XX is an integer padded to two places.

Side1: Across the top, from the left to the right: (char00, char01, char02)
Side2: Across the right side, from the top to the bottom. (char03, char04, char05)
Side3: Across the left side, from the top to the bottom. (char06, char07, char08)
Side4: Across the bottom, from the left to the right: (char09, char10, char11)

When the user presses a button to solve the puzzle, we gather the "sides" data from the above fields, and solve
the puzzle as before. 

When we import sides from the New York Times website, we perform appropriate string-splitting and assigning 
to these fields, and then saving to the local database as before.

The layout of these fields must not be absolute. Their position should be derived entirely from CSS and be responsive to the viewport size. It should always be arranged as a perfect square, where all sides are visually equal. 

You should have access to Playwright MCP to check how your work looks. 