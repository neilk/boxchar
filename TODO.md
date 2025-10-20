
  Basic Usage

# List available templates

  instruments -s devices
  instruments -s templates

# Profile with Time Profiler

  instruments -t "Time Profiler" -D trace_output.trace cargo run

# Profile with Allocations

  instruments -t "Allocations" -D trace_output.trace cargo run

# Profile with System Trace (shows all system activity)

  instruments -t "System Trace" -D trace_output.trace cargo run

  View Results

# Open trace file in Instruments GUI

  open trace_output.trace

# Or launch Instruments directly

  open -a Instruments trace_output.trace

  For Your Solver

# Profile your solver

  instruments -t "Time Profiler" -D solver_profile.trace cargo run

# Then view in GUI

  open solver_profile.trace

  The Time Profiler template will show you:

- Call tree with counts
- Time spent in each function
- Number of times each method was called

  Note: The binary must be built with debug symbols. Use cargo build --release with debug symbols:

  RUSTFLAGS="-g" cargo build --release
  instruments -t "Time Profiler" -D trace.trace ./target/release/letter-bounced



## Bugs!

- Can get a WASM not ready bug (thought we awaited that, should we loop)
- Are errors always visible? Maybe we fixed that

## TODO

- some playwright tests, this is getting gnarly

