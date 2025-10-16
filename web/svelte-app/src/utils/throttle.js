/**
 * Creates a throttled function that only invokes func at most once per every delay milliseconds.
 * The function is invoked immediately on the first call, then rate-limited for subsequent calls.
 * @param {Function} fn - The function to throttle
 * @param {number} delay - The number of milliseconds to throttle invocations to
 * @returns {Function} The throttled function
 */
export function throttle(fn, delay) {
  let lastCall = 0;
  let timeoutId = null;

  return (...args) => {
    const now = Date.now();
    const timeSinceLastCall = now - lastCall;

    // If enough time has passed, execute immediately
    if (timeSinceLastCall >= delay) {
      lastCall = now;
      fn(...args);
    } else {
      // Otherwise, schedule execution for when the delay period is over
      clearTimeout(timeoutId);
      timeoutId = setTimeout(() => {
        lastCall = Date.now();
        fn(...args);
      }, delay - timeSinceLastCall);
    }
  };
}
