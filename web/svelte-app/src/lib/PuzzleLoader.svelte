<script>
  import { puzzleFields } from '../stores/puzzle.js';

  let loading = false;

  const examplePuzzles = [
    { label: 'JGH NVY EID ORP', value: 'JGHNVYEIDORP' },
    { label: 'YFA OTK LGW RNI', value: 'YFAOTKLGWRNI' },
    { label: 'LHM CIB ANK OUP', value: 'LHMCIBANKOUP' },
    { label: 'GIY ERC XHA LOP', value: 'GIYERCXHALOP' },
    { label: 'PRC YAN LKH SIO', value: 'PRCYANLKHSIO' },
    { label: 'VYQ FIG OTE XLU', value: 'VYQFIGOTEXLU' }
  ];

  async function loadTodaysPuzzle() {
    loading = true;
    try {
      const url = 'https://www.nytimes.com/puzzles/letter-boxed';
      let response;

      // Try direct fetch first (works on deployed HTTPS sites)
      try {
        response = await fetch(url);
      } catch (e) {
        // Fall back to CORS proxy for localhost
        const proxyUrl = 'https://corsproxy.io/?' + encodeURIComponent(url);
        response = await fetch(proxyUrl);
      }

      const html = await response.text();
      const regex = /window\.gameData.*?"sides"\s*:\s*(\[.*?\])/;
      const match = html.match(regex);

      if (!match) {
        alert('Failed to find puzzle data on the NYT page. The page format may have changed.');
        return;
      }

      const sidesData = JSON.parse(match[1]);
      // Convert sides array to fields array
      const fields = sidesData.flatMap(side => side.split(''));
      puzzleFields.set(fields);
    } catch (error) {
      alert('Failed to load today\'s puzzle: ' + error.message);
    } finally {
      loading = false;
    }
  }

  function loadExample(event) {
    const value = event.target.value;
    if (!value) return;

    // Convert string of 12 letters to array
    const fields = value.split('');
    puzzleFields.set(fields);

    // Reset dropdown
    setTimeout(() => {
      event.target.value = '';
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
    color: #333;
  }

  .pill-button {
    background: #007bff;
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
    background: #0056b3;
  }

  .pill-button:disabled {
    background: #ccc;
    cursor: not-allowed;
  }

  .pill-select-wrapper {
    position: relative;
    display: inline-block;
  }

  .pill-select {
    background: #007bff;
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
    background: #0056b3;
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
