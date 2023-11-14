import View from "./lib";

const target = document.getElementById("root");
if (!target) throw new Error("Root element not found");

const view = new View({
  target,
  debug: true,
});

await view.init();
