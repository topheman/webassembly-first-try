import("../crate/pkg")
  .then(module => {
    module.run();
    return module;
  })
  .then(module => {
    document
      .querySelector("[data-id=hello-js]")
      .addEventListener("click", () => {
        module.run_alert("Calling native alert through WebAssembly(Rust)");
      });
  });
