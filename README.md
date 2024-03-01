# `wasi-sql-demo`

This repo. contains a demo. of [`wasi-sql`](https://github.com/WebAssembly/wasi-sql) in action.

## Repository Structure

- `guest/`: contains an example guest implementation of the interface.
- `host/`: contains an example host implementation of the interface.

## Run

To run the demo.:

```sh
make build
make componentize
make run
```

> Note: This repo. adds a "fake-handler" export to the original `wasi-sql` proposal, just to highlight calling something from the host to the guest. This is not part of the proposal – another way to do this would be to have the host call the guest's `_start` fxn.
