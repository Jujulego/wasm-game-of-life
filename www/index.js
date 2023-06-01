import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();
let last = 0;

function loop(time) {
    if (time - last > 200) {
        pre.textContent = universe.render();
        universe.tick();
        last = time;
    }

    requestAnimationFrame(loop);
}

requestAnimationFrame(loop);