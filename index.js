import { time_wasm } from 'test-wasm';

setInterval(() => {
  document.getElementById("time").innerHTML = time_wasm(String(Date.now()));
}, 1000);
