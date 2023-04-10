import init, {greet, World} from 'snake_game';

init().then(_ => {
    const cellSize = 10;
    const world = World.new();
    const worldWidth = world.width();

    const canvas = document.getElementById('canvas');
    const ctx = canvas.getContext("2d");
    
    canvas.width = worldWidth * cellSize;
    canvas.height = worldWidth * cellSize;

    function drawWorld(){
        ctx.beginPath();

        for(let x = 0; x < worldWidth + 1; x++){
            ctx.moveTo(cellSize * x, 0);
            ctx.lineTo(cellSize * x, cellSize * worldWidth);
        }

        for(let y = 0; y < worldWidth + 1; y++){
            ctx.moveTo(0, cellSize * y);
            ctx.lineTo(cellSize * worldWidth, cellSize * y);
        }

        ctx.stroke();
    }

    drawWorld();
});