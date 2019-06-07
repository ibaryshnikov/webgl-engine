import init, { Scene } from './pkg/webgl_engine.js';

window.addEventListener('load', async () => {
    const infoBlock = document.createElement('div');
    document.body.appendChild(infoBlock);

    let fps = 0;

    let lastTickFps = 0;

    function updateInfo() {
        lastTickFps = fps;
        fps = 0;
        setTimeout(updateInfo, 1000);
    }

    updateInfo();

    await init('./pkg/webgl_engine_bg.wasm');

    const scene = new Scene();
    scene.update_state();

    function draw() {
        requestAnimationFrame(draw);

        scene.update_state();
        scene.draw();
        fps += 1;
        infoBlock.innerHTML = `fps is ${lastTickFps}`;
    }

    draw();
});
