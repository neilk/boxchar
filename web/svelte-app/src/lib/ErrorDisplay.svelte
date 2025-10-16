<script>
  import { solverError } from '../stores/solver-worker.js';
</script>

{#if $solverError}
  <div class="error-container">
    <div class="error-message">
      <strong>Error:</strong> {$solverError}
    </div>
  </div>
{/if}

<style>
  .error-container {
    margin: 15px 0;
  }

  .error-message {
    color: var(--color-error);
    background: var(--color-error-bg);
    padding: 12px 16px;
    border-radius: 6px;
    border-left: 4px solid var(--color-error);
    font-size: 14px;
    line-height: 1.5;
  }

  .error-message strong {
    font-weight: 600;
  }
</style>
