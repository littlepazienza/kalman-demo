import { Universe } from "../pkg";

const GRID_COLOR = "#CCCCCC";
const KALMAN_COLOR = "#850c5d";

// Construct the universe, and get its width and height.
const CELL_SIZE = 10;
const universe = Universe.new(30.0, 30.0);
const width = universe.width() / 10;
const height = universe.height() / 10;

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("canvas");
canvas.height = universe.width() + CELL_SIZE;
canvas.width = universe.height() + CELL_SIZE;

const ctx = canvas.getContext('2d');

const renderLoop = async () => {
    universe.tick();

    ctx.reset()

    drawGrid();
    drawUniverse();

    // The tick rate is 1ms
    await delay(1);

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

const drawUniverse = () => {
    ctx.beginPath();

    ctx.strokeStyle = KALMAN_COLOR;
    ctx.fillRect(
        universe.kalman().get_x(),
        universe.kalman().get_y(),
        CELL_SIZE,
        CELL_SIZE
    );

    ctx.stroke();
};

drawGrid();
drawUniverse();
renderLoop();

document.getElementById('set_goal').addEventListener('click', function(e)
{
    setGoal();
}, false);

const setGoal = () => {
    universe.set_kalman_goal(document.getElementById("x").value, document.getElementById("y").value)
}

