<script lang="ts">
  import { puzzleFields } from '../stores/puzzle';

  let jumpingIndex: number = -1;

  function handleInput(index: number, event: Event): void {
    const target = event.target as HTMLInputElement;
    const value = target.value.toUpperCase();

    // Only allow single uppercase letter
    if (value.length > 0) {
      const letter = value[value.length - 1]!.replace(/[^A-Z]/g, '');
      target.value = letter;

      // Update the store
      puzzleFields.update(fields => {
        const newFields = [...fields];
        newFields[index] = letter;
        return newFields;
      });

      // Trigger jump animation
      jumpingIndex = index;

      // Auto-advance to next field
      if (letter && index < 11) {
        const nextField = document.getElementById(`char${String(index + 1).padStart(2, '0')}`);
        if (nextField) {
          nextField.focus();
          nextField.select();
        }
      }
    } else {
      target.value = '';
      puzzleFields.update(fields => {
        const newFields = [...fields];
        newFields[index] = '';
        return newFields;
      });
    }
  }

  function handleAnimationEnd(): void {
    jumpingIndex = -1;
  }

  function handleKeydown(index: number, event: KeyboardEvent): void {
    const target = event.target as HTMLInputElement;
    // Handle backspace to go to previous field
    if (event.key === 'Backspace' && !target.value && index > 0) {
      event.preventDefault();
      const prevField = document.getElementById(`char${String(index - 1).padStart(2, '0')}`);
      if (prevField) {
        prevField.focus();
        prevField.select();
      }
    }
  }

  function handleClick(event: MouseEvent): void {
    const target = event.target as HTMLInputElement;
    target.select();
  }
</script>

<div class="letter-box-container">
  {#each Array(12) as _, index}
    <input
      type="text"
      id="char{String(index).padStart(2, '0')}"
      class="letter-field"
      class:jump={jumpingIndex === index}
      maxlength="1"
      value={$puzzleFields[index]}
      on:input={(e) => handleInput(index, e)}
      on:keydown={(e) => handleKeydown(index, e)}
      on:click={handleClick}
      on:focus={handleClick}
      on:animationend={handleAnimationEnd}>
  {/each}
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
    color: var(--color-text-input);
    border: 2px solid var(--color-border);
    border-radius: 4px;
    background: var(--color-bg-white);
    padding: 0;
  }

  .letter-field:focus {
    outline: none;
    border-color: var(--color-primary);
    box-shadow: 0 0 0 3px var(--color-primary-light);
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

  /* Jump animation */
  @keyframes jump {
    0% {
      transform: translateY(0);
    }
    50% {
      transform: translateY(-25%);
    }
    100% {
      transform: translateY(0);
    }
  }

  .letter-field.jump {
    animation: jump 0.4s ease-out;
  }
</style>
