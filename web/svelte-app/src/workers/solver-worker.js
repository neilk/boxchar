import init, { initialize_dictionary, solve_game_streaming, cancel_current_solve } from '../pkg/boxchar.js';

let wasmReadyResolve = () => { };
let wasmReady = new Promise((resolve) => {
  // This promise stays pending until INIT completes
  wasmReadyResolve = resolve;
});
let currentSolveId = null;

self.addEventListener('message', async (e) => {
  const { type, payload, solveId } = e.data;

  if (type === 'INIT') {
    try {
      await init();
      await initialize_dictionary(payload.dictionaryData);
      wasmReadyResolve(); // Resolve the pending promise
      self.postMessage({ type: 'READY' });
    } catch (error) {
      self.postMessage({ type: 'ERROR', error: error.message });
    }
  }

  if (type === 'CANCEL') {
    await wasmReady;
    cancel_current_solve();
    currentSolveId = null;
  }


  if (type === 'SOLVE') {
    // Wait for WASM to be ready
    await wasmReady;

    // Cancel any previous solve
    if (currentSolveId !== null) {
      cancel_current_solve();
    }

    currentSolveId = solveId;
    const { sides, maxSolutions } = payload;

    let totalReceived = 0;

    // Callback function that WASM will call for each batch
    const onBatch = (solutionBatch) => {
      // Check if this solve is still current
      if (currentSolveId === solveId) {
        totalReceived += solutionBatch.length;

        // Convert JS array to regular array of strings
        const solutions = Array.from(solutionBatch);

        self.postMessage({
          type: 'BATCH',
          solveId,
          solutions,
          totalReceived
        });
      }
    };

    try {
      const startTime = performance.now();
      const totalCount = solve_game_streaming(sides, maxSolutions, onBatch);
      const duration = Math.round(performance.now() - startTime);

      // Only send complete message if this solve wasn't cancelled
      if (currentSolveId === solveId) {
        self.postMessage({
          type: 'COMPLETE',
          solveId,
          totalCount,
          duration
        });
        currentSolveId = null;
      } else {
        self.postMessage({
          type: 'CANCELLED',
          solveId,
          totalCount: totalReceived
        });
      }
    } catch (error) {
      self.postMessage({
        type: 'ERROR',
        error: error.message || error.toString()
      });
      currentSolveId = null;
    }
  }
});
