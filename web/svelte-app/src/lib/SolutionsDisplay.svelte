<script>
  import { solutions, solveTime } from '../stores/puzzle.js';

  let visibleCounts = {};
  let solutionsByWordCount = {};

  $: {
    // Segment solutions by word count
    solutionsByWordCount = {};
    $solutions.forEach(solution => {
      const wordCount = solution.split('-').length;
      if (!solutionsByWordCount[wordCount]) {
        solutionsByWordCount[wordCount] = [];
      }
      solutionsByWordCount[wordCount].push(solution);
    });

    // Initialize visible counts (show top 3 for each segment)
    visibleCounts = {};
    Object.keys(solutionsByWordCount).forEach(wordCount => {
      visibleCounts[wordCount] = 3;
    });
  }

  function loadMoreSolutions(wordCount) {
    visibleCounts[wordCount] = Math.min(
      visibleCounts[wordCount] + 10,
      solutionsByWordCount[wordCount].length
    );
  }
</script>

{#if $solutions.length > 0}
  <div class="solutions-container">
    <h2>Solutions</h2>

    {#if $solutions[0].startsWith('Error:')}
      <div class="error">{$solutions[0]}</div>
    {:else}
      <p>
        Found {$solutions.length} solution{$solutions.length === 1 ? '' : 's'}
        {#if $solveTime !== null}
          in <span class="timing">{$solveTime}ms</span>
        {/if}:
      </p>

      {#each Object.keys(solutionsByWordCount).sort((a, b) => a - b) as wordCount}
        {@const segmentSolutions = solutionsByWordCount[wordCount]}
        {@const visible = visibleCounts[wordCount]}
        {@const total = segmentSolutions.length}

        <div class="solution-segment">
          <div class="segment-header">
            <div class="segment-title">
              {wordCount}-Word Solution{wordCount === '1' ? '' : 's'}
            </div>
            <div class="segment-count">{total} total</div>
          </div>

          {#each segmentSolutions.slice(0, visible) as solution}
            <div class="solution">
              <div class="solution-words">{solution}</div>
            </div>
          {/each}

          {#if visible < total}
            <button
              class="load-more-btn"
              on:click={() => loadMoreSolutions(wordCount)}
            >
              Load 10 More ({total - visible} remaining)
            </button>
          {/if}
        </div>
      {/each}
    {/if}
  </div>
{/if}

<style>
  .solutions-container {
    margin-top: 20px;
  }

  h2 {
    margin-top: 0;
  }

  .error {
    color: #dc3545;
    background: #f8d7da;
    padding: 10px;
    border-radius: 4px;
    margin: 10px 0;
  }

  .timing {
    color: #28a745;
    font-weight: bold;
  }

  .solution-segment {
    background: white;
    padding: 20px;
    margin: 20px 0;
    border-radius: 8px;
    border: 2px solid #007bff;
  }

  .segment-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 15px;
    padding-bottom: 10px;
    border-bottom: 2px solid #e0e0e0;
  }

  .segment-title {
    font-size: 20px;
    font-weight: bold;
    color: #007bff;
  }

  .segment-count {
    font-size: 16px;
    color: #666;
    background: #f0f0f0;
    padding: 5px 12px;
    border-radius: 12px;
  }

  .solution {
    background: white;
    padding: 15px;
    margin: 10px 0;
    border-radius: 4px;
    border-left: 4px solid #007bff;
  }

  .solution-words {
    font-size: 18px;
    font-weight: bold;
    color: #007bff;
  }

  .load-more-btn {
    background: #6c757d;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 4px;
    font-size: 16px;
    cursor: pointer;
    margin-top: 10px;
    width: 100%;
  }

  .load-more-btn:hover {
    background: #5a6268;
  }
</style>
