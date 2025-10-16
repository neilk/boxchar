/**
 * Creates a throttled function that only invokes func at most once per every delay milliseconds.
 * The function is invoked immediately on the first call, then rate-limited for subsequent calls.
 * @param fn - The function to throttle
 * @param delay - The number of milliseconds to throttle invocations to
 * @returns The throttled function
 */
export function throttle<T extends (...args: any[]) => void>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let lastCall = 0;
  let timeoutId: ReturnType<typeof setTimeout> | null = null;

  return (...args: Parameters<T>) => {
    const now = Date.now();
    const timeSinceLastCall = now - lastCall;

    // If enough time has passed, execute immediately
    if (timeSinceLastCall >= delay) {
      lastCall = now;
      fn(...args);
    } else {
      // Otherwise, schedule execution for when the delay period is over
      if (timeoutId !== null) {
        clearTimeout(timeoutId);
      }
      timeoutId = setTimeout(() => {
        lastCall = Date.now();
        fn(...args);
      }, delay - timeSinceLastCall);
    }
  };
}
