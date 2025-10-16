<script lang="ts">
  import { puzzleFields } from '../stores/puzzle';

  interface ExamplePuzzle {
    label: string;
    value: string;
  }

  let loading: boolean = false;

  const examplePuzzles: ExamplePuzzle[] = [
    { label: 'JGH NVY EID ORP', value: 'JGHNVYEIDORP' },
    { label: 'YFA OTK LGW RNI', value: 'YFAOTKLGWRNI' },
    { label: 'LHM CIB ANK OUP', value: 'LHMCIBANKOUP' },
    { label: 'GIY ERC XHA LOP', value: 'GIYERCXHALOP' },
    { label: 'PRC YAN LKH SIO', value: 'PRCYANLKHSIO' },
    { label: 'VYQ FIG OTE XLU', value: 'VYQFIGOTEXLU' }
  ];

  async function loadTodaysPuzzle(): Promise<void> {
    loading = true;
    try {
      const url: string = 'https://www.nytimes.com/puzzles/letter-boxed';
      let response: Response;

      // Try direct fetch first (works on deployed HTTPS sites)
      try {
        response = await fetch(url);
      } catch (e) {
        // Fall back to CORS proxy for localhost
        const proxyUrl: string = 'https://corsproxy.io/?' + encodeURIComponent(url);
        response = await fetch(proxyUrl);
      }

      const html: string = await response.text();
      const regex: RegExp = /window\.gameData.*?"sides"\s*:\s*(\[.*?\])/;
      const match: RegExpMatchArray | null = html.match(regex);

      if (!match) {
        alert('Failed to find puzzle data on the NYT page. The page format may have changed.');
        return;
      }

      const sidesData: string[] = JSON.parse(match[1]) as string[];
      // Convert sides array to fields array
      const fields: string[] = sidesData.flatMap((side: string) => side.split(''));
      puzzleFields.set(fields);
    } catch (error) {
      const message: string = error instanceof Error ? error.message : 'Unknown error';
      alert('Failed to load today\'s puzzle: ' + message);
    } finally {
      loading = false;
    }
  }

  function loadExample(event: Event): void {
    const target = event.target as HTMLSelectElement;
    const value: string = target.value;
    if (!value) return;

    // Convert string of 12 letters to array
    const fields: string[] = value.split('');
    puzzleFields.set(fields);

    // Reset dropdown
    setTimeout(() => {
      target.value = '';
    }, 100);
  }
</script>

<div class="puzzle-loader">
  <span class="puzzle-loader-label">Get a puzzle:</span>
  <button
    class="pill-button"
    on:click={loadTodaysPuzzle}
    disabled={loading}
  >
    {loading ? 'Loading...' : 'NYT'}
  </button>
  <div class="pill-select-wrapper">
    <select class="pill-select" on:change={loadExample}>
      <option value="">Sample</option>
      {#each examplePuzzles as puzzle}
        <option value={puzzle.value}>{puzzle.label}</option>
      {/each}
    </select>
  </div>
</div>

<style>
  .puzzle-loader {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 12px;
    margin: 20px 0;
  }

  .puzzle-loader-label {
    font-weight: 600;
    color: var(--color-text);
  }

  .pill-button {
    background: var(--color-primary);
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 20px;
    font-size: 14px;
    cursor: pointer;
    transition: background 0.2s;
    white-space: nowrap;
  }

  .pill-button:hover:not(:disabled) {
    background: var(--color-primary-hover);
  }

  .pill-button:disabled {
    background: var(--color-disabled);
    cursor: not-allowed;
  }

  .pill-select-wrapper {
    position: relative;
    display: inline-block;
  }

  .pill-select {
    background: var(--color-primary);
    color: white;
    border: none;
    padding: 10px 35px 10px 20px;
    border-radius: 20px;
    font-size: 14px;
    cursor: pointer;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    transition: background 0.2s;
  }

  .pill-select:hover {
    background: var(--color-primary-hover);
  }

  .pill-select-wrapper::after {
    content: '▼';
    position: absolute;
    right: 12px;
    top: 50%;
    transform: translateY(-50%);
    pointer-events: none;
    color: white;
    font-size: 10px;
  }
</style>
