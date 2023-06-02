import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

// Constants
const CELL_SIZE = 5;
const DEAD_COLOR = "#FFFFFFDD";
const ALIVE_COLOR = "#000000";
const FRAME_RATE = 50;

// Setup universe
const universe = Universe.new(256, 128);
const height = universe.height();
const width = universe.width();

// Setup canvas
const canvas = document.getElementById("game-of-life-canvas");
canvas.width = CELL_SIZE * width;
canvas.height = CELL_SIZE * height;

const ctx = canvas.getContext('2d');

// Render loop
let last = 0;

function loop(time) {
    if (time - last > FRAME_RATE) {
        last = time;

        // Update state
        universe.tick();

        // Draw cells
        const cells = new Uint8Array(memory.buffer, universe.cells(), width * height);

        ctx.beginPath();

        for (let row = 0; row < height; ++row) {
            for (let col = 0; col < width; ++col) {
                const idx = row * width + col;

                ctx.fillStyle = cells[idx] === Cell.Alive ? ALIVE_COLOR : DEAD_COLOR;
                ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
            }
        }

        ctx.stroke();
    }

    requestAnimationFrame(loop);
}

requestAnimationFrame(loop);