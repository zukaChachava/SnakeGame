import init, { World, Direction, GameStatus } from "snake_game/snake_game";

init().then((_) => {
  const cellSize = 40;
  const width = 8;
  const start_index = Date.now() % (width * width);
  const speed = 2;

  const startBtn = document.getElementById("Start");
  const failBtn = document.getElementById("Fail");
  const wonBtn = document.getElementById("Won");

  function failed() {
    failBtn.classList.remove("hide");
  }

  function won() {
    wonBtn.classList.remove("hide");
  }

  function initGame(): () => void {
    const world = World.new(width, start_index);
    const worldWidth = world.width();

    const canvas = <HTMLCanvasElement>document.getElementById("canvas");
    const ctx = canvas.getContext("2d");

    canvas.width = worldWidth * cellSize;
    canvas.height = worldWidth * cellSize;

    return function startGame() {
      console.log("here 1");
      console.log(world.get_game_status());
      world.start_game();
      console.log("here 2");

      function drawWorld() {
        ctx.beginPath();

        for (let x = 0; x < worldWidth + 1; x++) {
          ctx.moveTo(cellSize * x, 0);
          ctx.lineTo(cellSize * x, cellSize * worldWidth);
        }

        for (let y = 0; y < worldWidth + 1; y++) {
          ctx.moveTo(0, cellSize * y);
          ctx.lineTo(cellSize * worldWidth, cellSize * y);
        }

        ctx.stroke();
      }

      function drawSnake() {
        const len = world.snake_len();

        for (let i = 0; i < len; i++) {
          ctx.beginPath();
          const position = world.snake_body_position(i);
          const col = position % world.width();
          ctx.fillStyle = i === 0 ? "white" : "black";
          const row = Math.floor(position / world.width());
          const radius = cellSize / 2;
          ctx.arc(
            col * cellSize + radius,
            row * cellSize + radius,
            radius,
            0,
            Math.PI * 2 * radius
          );
          ctx.fill();
        }
      }

      function drawReward() {
        const position = world.get_reward_cell();
        const col = position % world.width();
        const row = Math.floor(position / world.width());
        ctx.beginPath();
        ctx.fillStyle = "red";
        ctx.fillRect(col * cellSize, row * cellSize, cellSize, cellSize);
        ctx.fill();
      }

      function update() {
        const gameStatus = world.get_game_status();

        switch (gameStatus) {
          case 2:
            won();
            return;
          case 3:
            failed();
            return;
        }

        setTimeout(() => {
          ctx.clearRect(
            0,
            0,
            world.width() * cellSize,
            world.width() * cellSize
          );
          drawWorld();
          drawReward();
          drawSnake();
          world.update();
          requestAnimationFrame(update);
        }, 500 / speed);
      }

      function change_directions() {
        window.addEventListener("keydown", (e) => {
          switch (e.code) {
            case "ArrowUp":
              change_direction(Direction.Up);
              break;
            case "KeyW":
              change_direction(Direction.Up);
              break;
            case "ArrowDown":
              change_direction(Direction.Down);
              break;
            case "KeyS":
              change_direction(Direction.Down);
              break;
            case "ArrowLeft":
              change_direction(Direction.Left);
              break;
            case "KeyA":
              change_direction(Direction.Left);
              break;
            case "ArrowRight":
              change_direction(Direction.Right);
              break;
            case "KeyD":
              change_direction(Direction.Right);
              break;
          }
        });
      }

      function change_direction(direction: Direction) {
        world.change_direction(direction);
      }

      update();
      change_directions();
    };
  }

  startBtn.addEventListener("click", onStart);
  failBtn.addEventListener("click", onFail);
  wonBtn.addEventListener("click", onWon);

  function onStart(e: MouseEvent) {
    startBtn.classList.add("hide");
    initGame()();
  }

  function onFail(e: MouseEvent) {
    failBtn.classList.add("hide");
    startBtn.classList.remove("hide");
    initGame();
  }

  function onWon(e: MouseEvent) {
    wonBtn.classList.add("hide");
    startBtn.classList.remove("hide");
    initGame();
  }
});
