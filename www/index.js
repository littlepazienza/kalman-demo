import { Graph, Cell } from "kalman-demo";
import { memory } from "kalman-demo/kalman_demo_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const EMPTY_COLOR = "#FFFFFF";
const WALL_COLOR = "#000000";
const KALMAN_COLOR = "#850c5d";


// Construct the universe, and get its width and height.
const graph = Graph.new(3, 3);
const width = graph.width();
const height = graph.height();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

function start_animate(duration) {
    var requestID;
    var startTime =null;
    var time ;

    var animate = function(time) {


        time = new Date().getTime(); //millisecond-timstamp

        if (startTime === null) {
            startTime = time;
        }
        var progress = time - startTime;

        if (progress < duration ) {


        }
        else{
            cancelAnimationFrame(requestID);
        }
        requestID=requestAnimationFrame(animate);
    }
    animate();
}

const renderLoop = async () => {
    graph.tick();

    drawGrid();
    drawCells();

    await delay(1000);

    requestAnimationFrame(renderLoop);
};

function delay(milliseconds){
    return new Promise(resolve => {
        setTimeout(resolve, milliseconds);
    });
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = graph.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            if (cells[idx] === Cell.Empty) {
                ctx.fillStyle = EMPTY_COLOR;
            } else if (cells[idx] === Cell.Wall) {
                ctx.fillStyle = WALL_COLOR;
            } else if (cells[idx] === Cell.Kalman) {
                ctx.fillStyle = KALMAN_COLOR;
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);


