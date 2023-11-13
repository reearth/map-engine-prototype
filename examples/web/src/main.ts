import ("map-engine-prototype").then(wasm => wasm.default()).then(wasm => {
  wasm.greet();
})
