import { writable, type Writable } from 'svelte/store';

export const wasmModule: Writable<unknown> = writable(null);
export const dictionaryInitialized: Writable<boolean> = writable(false);
export const initializationError: Writable<string | null> = writable(null);

let wasmInstance: unknown = null;
let dictInitialized = false;

interface WasmExports {
  initialize_dictionary: (data: Uint8Array) => Promise<void>;
  solve_game: (sides: string[], maxSolutions: number) => string[];
}

export async function initializeWasm(): Promise<{ solve_game: (sides: string[], maxSolutions: number) => string[] }> {
  try {
    // Import WASM module from src/pkg folder
    const wasmModule = await import('../pkg/letter_bounced.js');
    const init = wasmModule.default;
    const { initialize_dictionary, solve_game } = wasmModule as unknown as WasmExports & { default: () => Promise<unknown> };

    // First, initialize WASM module
    wasmInstance = await init();
    console.log('WASM module loaded');
    wasmModule.set(wasmInstance);

    // Then load and initialize dictionary
    const response = await fetch(`${import.meta.env.BASE_URL}dictionary.txt`);
    if (!response.ok) {
      throw new Error(`Failed to load dictionary.txt: ${response.status} ${response.statusText}`);
    }

    const arrayBuffer = await response.arrayBuffer();
    const dictionaryData = new Uint8Array(arrayBuffer);

    console.log(`Loaded dictionary data: ${dictionaryData.length} bytes`);

    await initialize_dictionary(dictionaryData);
    dictInitialized = true;
    dictionaryInitialized.set(true);

    console.log('Dictionary initialized');

    // Return the solve_game function for use
    return { solve_game };
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    console.error('WASM initialization failed:', error);
    initializationError.set(errorMessage);
    throw error;
  }
}

export function getWasmInstance(): unknown {
  return wasmInstance;
}

export function isDictionaryInitialized(): boolean {
  return dictInitialized;
}
