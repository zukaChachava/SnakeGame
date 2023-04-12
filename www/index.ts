import init, {World} from 'snake_game';

init().then(_ => {
    const cellSize = 40;
    const world = World.new();
    const worldWidth = world.width();

    const canvas = <HTMLCanvasElement> document.getElementById('canvas');
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

    function drawSnake(){
        const head = world.snake_head();
        const col =  head % world.width();
        const row = Math.floor(head / world.width());

        ctx.beginPath();
        ctx.fillRect(col * cellSize, row * cellSize, cellSize, cellSize);
        ctx.stroke();
    }

    function update(){
        setTimeout(() => {
            ctx.clearRect(0, 0, world.width() * cellSize, world.width() * cellSize);
            drawWorld();
            drawSnake();
            world.update();
            requestAnimationFrame(update);
        }, 500);
    }

   update();
});