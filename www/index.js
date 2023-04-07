import init, {greet, World} from 'snake_game';

init().then(_ => {
    greet("zura");
    const world = World.new();
    console.log(world.width);
});