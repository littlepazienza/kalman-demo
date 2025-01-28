import { Universe } from "@littlepaz/kalman-demo";

const GRID_COLOR = "#CCCCCC";
const GOAL_COLOR = "#e60606";
const KALMAN_COLOR = "#000000";

// Construct the universe, and get its width and height.
const CELL_SIZE = 10;
const universe = Universe.new(1.0, 1.0);
const width = universe.width() / 10;
const height = universe.height() / 10;

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("canvas");
canvas.height = universe.width() + CELL_SIZE;
canvas.width = universe.height() + CELL_SIZE;

const ctx = canvas.getContext('2d');
let x = 0

/*
 #################################
 RENDER LOOP FUNCTIONS
 #################################
 */
const renderLoop = async () => {
    universe.tick();

    ctx.reset()

    drawGrid();
    drawUniverse();
    debugInfo();

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
    ctx.fillStyle = GOAL_COLOR;
    const goal = universe.kalman().get_goal()
    if (goal[0] > 0 && goal[1] > 0) {
        ctx.fillRect(
            goal[0],
            goal[1],
            CELL_SIZE,
            CELL_SIZE
        )
    }
    ctx.stroke();

    ctx.beginPath();
    ctx.fillStyle = KALMAN_COLOR;
    ctx.fillRect(
        universe.kalman().get_actual()[0],
        universe.kalman().get_actual()[1],
        CELL_SIZE,
        CELL_SIZE
    );
    ctx.stroke();
};

const debugInfo = () => {
    if (universe.kalman().get_belief()[2] > 0) {
        x += 1
        const actual = universe.kalman().get_actual()
        const belief = universe.kalman().get_belief()
        const actual_txt = document.getElementById('actual')
        const belief_txt = document.getElementById('belief')
        actual_txt.value = actual_txt.value + `${x} > [x: ${actual[0].toFixed(2)}, y: ${actual[1].toFixed(2)}, v: ${actual[2].toFixed(2)}, θ: ${actual[3].toFixed(2)}]\n`
        belief_txt.value = belief_txt.value + `${x} > [x: ${belief[0].toFixed(2)}, y: ${belief[1].toFixed(2)}, v: ${belief[2].toFixed(2)}, θ: ${belief[3].toFixed(2)}]\n`

        // Make sure the rolling text areas keep their scroll on the bottom by default
        actual_txt.scrollTop = actual_txt.scrollHeight;
        belief_txt.scrollTop = belief_txt.scrollHeight;
    }

    // Paint the error debug info
    const movement_error = universe.kalman().get_error_mean()
    const movemnet_cov = universe.kalman().get_error_covariance()
    const movement_txt = document.getElementById('movement_error')
    movement_txt.value = `N([${movement_error[0].toFixed(5)}, ${movement_error[1].toFixed(5)}], [${movemnet_cov[0].toFixed(5)}, ${movemnet_cov[1].toFixed(5)}, ${movemnet_cov[2].toFixed(5)}, ${movemnet_cov[3].toFixed(5)}])`
}

/*
 #################################
 REGISTER EVENTS
 #################################
 */
document.getElementById('set_goal').addEventListener('click', function(e)
{
    setGoal();
}, false);
document.getElementById('set_movement_error').addEventListener('click', function(e)
{
    setMovementError();
}, false);
const setGoal = () => {
    universe.set_kalman_goal(document.getElementById("x").value, document.getElementById("y").value)
}
const setMovementError = () => {
    let movement_m = JSON.parse(document.getElementById("movement_m").value);
    let movement_std = JSON.parse(document.getElementById("movement_s").value);
    universe.set_kalman_movement_error(movement_m, movement_std);
}

/*
 #################################
 Init
 #################################
 */
drawGrid();
drawUniverse();
debugInfo();
renderLoop();


