import init, { initialize_dictionary, solve_game_streaming, cancel_current_solve } from '../pkg/boxchar.js';

interface WorkerMessageData {
  type: 'INIT' | 'CANCEL' | 'SOLVE';
  payload?: {
    dictionaryData?: Uint8Array;
    sides?: string[];
    maxSolutions?: number;
  };
  solveId?: number;
}

interface OutgoingMessage {
  type: 'READY' | 'BATCH' | 'COMPLETE' | 'CANCELLED' | 'ERROR';
  solveId?: number;
  solutions?: string[];
  totalReceived?: number;
  totalCount?: number;
  duration?: number;
  error?: string;
}

let wasmReadyResolve: () => void = () => { };
let wasmReady: Promise<void> = new Promise((resolve) => {
  // This promise stays pending until INIT completes
  wasmReadyResolve = resolve;
});
let currentSolveId: number | null = null;

self.addEventListener('message', async (e: MessageEvent<WorkerMessageData>) => {
  const { type, payload, solveId } = e.data;

  if (type === 'INIT') {
    try {
      await init();
      if (payload?.dictionaryData) {
        await initialize_dictionary(payload.dictionaryData);
      }
      wasmReadyResolve(); // Resolve the pending promise
      self.postMessage({ type: 'READY' } as OutgoingMessage);
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      self.postMessage({ type: 'ERROR', error: errorMessage } as OutgoingMessage);
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

    currentSolveId = solveId ?? null;
    const sides = payload?.sides ?? [];
    const maxSolutions = payload?.maxSolutions ?? 10000;

    let totalReceived = 0;

    // Callback function that WASM will call for each batch
    const onBatch = (solutionBatch: string[]) => {
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
        } as OutgoingMessage);
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
        } as OutgoingMessage);
        currentSolveId = null;
      } else {
        self.postMessage({
          type: 'CANCELLED',
          solveId,
          totalCount: totalReceived
        } as OutgoingMessage);
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      self.postMessage({
        type: 'ERROR',
        error: errorMessage
      } as OutgoingMessage);
      currentSolveId = null;
    }
  }
});
