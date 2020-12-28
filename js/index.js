import("../pkg/index.js")
    .catch(console.error)
    .then(wasm => {
        window.wasm = wasm;

        let gameboy = wasm.Console.new();
        window.gameboy = gameboy;
    });

window.onFileLoad = (event) => {
        const input = event.target.files[0];
        const fileReader = new FileReader();
        fileReader.onloadend = e => {
                const jsValue = Array.from(new Uint8Array(fileReader.result));
                gameboy.load(jsValue);
                gameboy.reset();
                console.log("Loaded!");
        };
        fileReader.readAsArrayBuffer(input);
}

const SCREEN_W = 160;
const SCREEN_H = 144;

let screen = document.getElementById('screen');
screen.width = SCREEN_W;
screen.height = SCREEN_H;
let canvas = screen.getContext('2d');

window.runningFlag = true;
window.runRustyBoy = () => {
        setTimeout(function() {
                if (runningFlag) requestAnimationFrame(window.runRustyBoy);
                window.gameboy.execute_ticks(200000);
                let frame = window.gameboy.get_frame();
                let buffer = canvas.getImageData(0, 0, SCREEN_W, SCREEN_H);

                let index = 0;
                for (let i = 0; i < frame.length; i += 3) {
                        let r = frame[i + 0];
                        let g = frame[i + 1];
                        let b = frame[i + 2];

                        buffer.data[index + 0] = r;
                        buffer.data[index + 1] = g;
                        buffer.data[index + 2] = b;
                        buffer.data[index + 3] = 255;

                        index += 4;
                }
                canvas.putImageData(buffer, 0, 0);
        }, 1000 / 60);
}



