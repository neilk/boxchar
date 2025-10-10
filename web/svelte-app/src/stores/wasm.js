import { writable } from 'svelte/store';

export const wasmModule = writable(null);
export const dictionaryInitialized = writable(false);
export const initializationError = writable(null);

let wasmInstance = null;
let dictInitialized = false;

export async function initializeWasm() {
  try {
    // Import WASM module from src/pkg folder
    const { default: init, initialize_dictionary, solve_game } = await import('../pkg/boxchar.js');

    // First, initialize WASM module
    wasmInstance = await init();
    console.log('WASM module loaded');
    wasmModule.set(wasmInstance);

    // Then load and initialize dictionary
    const response = await fetch(`${import.meta.env.BASE_URL}dictionary.txt`);
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
    console.error('WASM initialization failed:', error);
    initializationError.set(error.message);
    throw error;
  }
}

export function getWasmInstance() {
  return wasmInstance;
}

export function isDictionaryInitialized() {
  return dictInitialized;
}
