<script>
  import { puzzleSides } from '../stores/puzzle.js';

  let fields = [];

  // Initialize fields as individual characters from the sides
  $: {
    fields = [];
    for (let i = 0; i < 4; i++) {
      const side = $puzzleSides[i] || '';
      for (let j = 0; j < 3; j++) {
        fields.push(side[j] || '');
      }
    }
  }

  // Map field indices to their side index
  const FIELD_TO_SIDE = [
    0, 0, 0,  // fields 0-2: side 0 (top)
    1, 1, 1,  // fields 3-5: side 1 (right)
    2, 2, 2,  // fields 6-8: side 2 (left)
    3, 3, 3   // fields 9-11: side 3 (bottom)
  ];

  function updateSides() {
    const newSides = ['', '', '', ''];
    fields.forEach((char, index) => {
      const sideIndex = FIELD_TO_SIDE[index];
      newSides[sideIndex] += char.toUpperCase();
    });
    puzzleSides.set(newSides);
  }

  function handleInput(index, event) {
    const value = event.target.value.toUpperCase();

    // Only allow single uppercase letter
    if (value.length > 0) {
      fields[index] = value[value.length - 1].replace(/[^A-Z]/g, '');
      event.target.value = fields[index];

      updateSides();

      // Auto-advance to next field
      if (fields[index] && index < 11) {
        const nextField = document.getElementById(`char${String(index + 1).padStart(2, '0')}`);
        if (nextField) {
          nextField.focus();
          nextField.select();
        }
      }
    } else {
      fields[index] = '';
      event.target.value = '';
      updateSides();
    }
  }

  function handleKeydown(index, event) {
    // Handle backspace to go to previous field
    if (event.key === 'Backspace' && !event.target.value && index > 0) {
      event.preventDefault();
      const prevField = document.getElementById(`char${String(index - 1).padStart(2, '0')}`);
      if (prevField) {
        prevField.focus();
        prevField.select();
      }
    }
  }

  function handleClick(event) {
    event.target.select();
  }
</script>

<div class="letter-box-container">
  <!-- Top side: char00, char01, char02 (left to right) -->
  <input type="text" id="char00" class="letter-field" maxlength="1"
    value={fields[0]}
    on:input={(e) => handleInput(0, e)}
    on:keydown={(e) => handleKeydown(0, e)}
    on:click={handleClick}
    on:focus={handleClick}>
  <input type="text" id="char01" class="letter-field" maxlength="1"
    value={fields[1]}
    on:input={(e) => handleInput(1, e)}
    on:keydown={(e) => handleKeydown(1, e)}
    on:click={handleClick}
    on:focus={handleClick}>
  <input type="text" id="char02" class="letter-field" maxlength="1"
    value={fields[2]}
    on:input={(e) => handleInput(2, e)}
    on:keydown={(e) => handleKeydown(2, e)}
    on:click={handleClick}
    on:focus={handleClick}>

  <!-- Right side: char03, char04, char05 (top to bottom) -->
  <input type="text" id="char03" class="letter-field" maxlength="1"
    value={fields[3]}
    on:input={(e) => handleInput(3, e)}
    on:keydown={(e) => handleKeydown(3, e)}
    on:click={handleClick}
    on:focus={handleClick}>
  <input type="text" id="char04" class="letter-field" maxlength="1"
    value={fields[4]}
    on:input={(e) => handleInput(4, e)}
    on:keydown={(e) => handleKeydown(4, e)}
    on:click={handleClick}
    on:focus={handleClick}>
  <input type="text" id="char05" class="letter-field" maxlength="1"
    value={fields[5]}
    on:input={(e) => handleInput(5, e)}
    on:keydown={(e) => handleKeydown(5, e)}
    on:click={handleClick}
    on:focus={handleClick}>

  <!-- Left side: char06, char07, char08 (top to bottom) -->
  <input type="text" id="char06" class="letter-field" maxlength="1"
    value={fields[6]}
    on:input={(e) => handleInput(6, e)}
    on:keydown={(e) => handleKeydown(6, e)}
    on:click={handleClick}
    on:focus={handleClick}>
  <input type="text" id="char07" class="letter-field" maxlength="1"
    value={fields[7]}
    on:input={(e) => handleInput(7, e)}
    on:keydown={(e) => handleKeydown(7, e)}
    on:click={handleClick}
    on:focus={handleClick}>
  <input type="text" id="char08" class="letter-field" maxlength="1"
    value={fields[8]}
    on:input={(e) => handleInput(8, e)}
    on:keydown={(e) => handleKeydown(8, e)}
    on:click={handleClick}
    on:focus={handleClick}>

  <!-- Bottom side: char09, char10, char11 (left to right) -->
  <input type="text" id="char09" class="letter-field" maxlength="1"
    value={fields[9]}
    on:input={(e) => handleInput(9, e)}
    on:keydown={(e) => handleKeydown(9, e)}
    on:click={handleClick}
    on:focus={handleClick}>
  <input type="text" id="char10" class="letter-field" maxlength="1"
    value={fields[10]}
    on:input={(e) => handleInput(10, e)}
    on:keydown={(e) => handleKeydown(10, e)}
    on:click={handleClick}
    on:focus={handleClick}>
  <input type="text" id="char11" class="letter-field" maxlength="1"
    value={fields[11]}
    on:input={(e) => handleInput(11, e)}
    on:keydown={(e) => handleKeydown(11, e)}
    on:click={handleClick}
    on:focus={handleClick}>
</div>

<style>
  .letter-box-container {
    width: 100%;
    max-width: 500px;
    aspect-ratio: 1;
    position: relative;
    display: grid;
    grid-template-columns: repeat(5, 1fr);
    grid-template-rows: repeat(5, 1fr);
    gap: 8px;
    padding: 20px;
  }

  .letter-field {
    width: 100%;
    height: 100%;
    text-align: center;
    font-size: clamp(16px, 3vw, 32px);
    font-weight: bold;
    text-transform: uppercase;
    border: 2px solid #333;
    border-radius: 4px;
    background: white;
    padding: 0;
  }

  .letter-field:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
  }

  /* Top side - char00, char01, char02 (left to right) */
  #char00 { grid-column: 2; grid-row: 1; }
  #char01 { grid-column: 3; grid-row: 1; }
  #char02 { grid-column: 4; grid-row: 1; }

  /* Right side - char03, char04, char05 (top to bottom) */
  #char03 { grid-column: 5; grid-row: 2; }
  #char04 { grid-column: 5; grid-row: 3; }
  #char05 { grid-column: 5; grid-row: 4; }

  /* Left side - char06, char07, char08 (top to bottom) */
  #char06 { grid-column: 1; grid-row: 2; }
  #char07 { grid-column: 1; grid-row: 3; }
  #char08 { grid-column: 1; grid-row: 4; }

  /* Bottom side - char09, char10, char11 (left to right) */
  #char09 { grid-column: 2; grid-row: 5; }
  #char10 { grid-column: 3; grid-row: 5; }
  #char11 { grid-column: 4; grid-row: 5; }
</style>
