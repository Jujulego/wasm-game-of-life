import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// Constants
const CELL_SIZE = 5;
const DEAD_COLOR = "#FFFFFFDD";
const ALIVE_COLOR = "#000000";
const FRAME_RATE = 50;

// Setup universe
const universe = Universe.random(256, 128);
const size = universe.size();

// Setup canvas
const canvas = document.getElementById("game-of-life-canvas");
canvas.width = size.dx * CELL_SIZE;
canvas.height = size.dy * CELL_SIZE;

const ctx = canvas.getContext('2d');

// Render loop
let last = 0;

function loop(time) {
    if (time - last > FRAME_RATE) {
        last = time;

        // Update state
        universe.tick();

        // Draw cells
        const cells = new Uint8Array(memory.buffer, universe.cells(), size.dx * size.dy);

        ctx.beginPath();

        for (let row = 0; row < size.dy; ++row) {
            for (let col = 0; col < size.dx; ++col) {
                const idx = row * size.dx + col;

                ctx.fillStyle = cells[idx] === Cell.Alive ? ALIVE_COLOR : DEAD_COLOR;
                ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
            }
        }

        ctx.stroke();
    }

    requestAnimationFrame(loop);
}

requestAnimationFrame(loop);