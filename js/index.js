import {HEIGHT, Screen, WIDTH} from './screen';
import tippy from 'tippy.js';

tippy('[data-tippy-content]');

import("../pkg/index.js")
    .catch(console.error)
    .then(wasm => {
        window.wasm = wasm;

        let gameboy = wasm.Console.new();
        window.gameboy = gameboy;
        window.Button = window.wasm.Button;
    });

window.clickLoadRom = () => {
        document.getElementById("cartridge-input").click();
}

window.onFileLoad = (url) => {
        const input = event.target.files[0];
        window.loadRom(input);

}

window.loadRom = (url) => {
        const fileReader = new FileReader();
        fileReader.onloadend = e => {
                const jsValue = Array.from(new Uint8Array(fileReader.result));
                gameboy.load(jsValue);
                gameboy.reset();
                console.log("Loaded!");
                window.runRustyBoy();
        };
        fileReader.readAsArrayBuffer(url);
}

let canvas = document.getElementById('screen');
window.runningFlag = true;

// Web-gl Rendering
window.screen = new Screen(canvas);
window.runRustyBoy = () => {
        setTimeout(function() {
                if (runningFlag) requestAnimationFrame(window.runRustyBoy);
                window.gameboy.execute_ticks(27756);
                let frame = window.gameboy.get_frame();
                let buffer = screen.createBuffer();
                buffer.data.set(frame);
                screen.render(buffer);
        }, 1000 / 60);
}

export const key_mapping = {
        R: "T",
        L: "G",
        X: "Y",
        A: "H",
        RIGHT: "D",
        LEFT: "A",
        DOWN: "S",
        UP: "W",
        START: "B",
        SELECT: "N",
        Y: "U",
        B: "J",
}

window.document.onkeydown = (event) => {
        let code = event.key ? event.key.toUpperCase() : null;

        if (code === key_mapping.UP) {
                window.gameboy.press_button(window.Button.UP);
        } else if (code === key_mapping.DOWN) {
                window.gameboy.press_button(window.Button.DOWN);
        } else if (code === key_mapping.LEFT) {
                window.gameboy.press_button(window.Button.LEFT);
        } else if (code === key_mapping.RIGHT) {
                window.gameboy.press_button(window.Button.RIGHT);
        } else if (code === key_mapping.START) {
                console.log("pressing start");
                window.gameboy.press_button(window.Button.START);
        } else if (code === key_mapping.SELECT) {
                window.gameboy.press_button(window.Button.SELECT);
        } else if (code === key_mapping.A) {
                window.gameboy.press_button(window.Button.A);
        } else if (code === key_mapping.B) {
                window.gameboy.press_button(window.Button.B);
        }
}

window.document.onkeyup = (event) => {
        let code = event.key ? event.key.toUpperCase() : null;

        if (code === key_mapping.UP) {
                window.gameboy.release_button(window.Button.UP);
        } else if (code === key_mapping.DOWN) {
                window.gameboy.release_button(window.Button.DOWN);
        } else if (code === key_mapping.LEFT) {
                window.gameboy.release_button(window.Button.LEFT);
        } else if (code === key_mapping.RIGHT) {
                window.gameboy.release_button(window.Button.RIGHT);
        } else if (code === key_mapping.START) {
                window.gameboy.release_button(window.Button.START);
        } else if (code === key_mapping.SELECT) {
                window.gameboy.release_button(window.Button.SELECT);
        } else if (code === key_mapping.A) {
                window.gameboy.release_button(window.Button.A);
        } else if (code === key_mapping.B) {
                window.gameboy.release_button(window.Button.B);
        }
}

// Canvas Rendering
// window.context = canvas.getContext('2d');
// window.runRustyBoy = () => {
//         setTimeout(function() {
//                 if (runningFlag) requestAnimationFrame(window.runRustyBoy);
//                 window.gameboy.execute_ticks(200000);
//                 let frame = window.gameboy.get_frame();
//                 let buffer = context.getImageData(0, 0, WIDTH, HEIGHT);
//
//                 let index = 0;
//                 for (let i = 0; i < frame.length; i += 4) {
//                         let r = frame[i + 0];
//                         let g = frame[i + 1];
//                         let b = frame[i + 2];
//                         let a = frame[i + 3];
//
//                         buffer.data[index + 0] = r;
//                         buffer.data[index + 1] = g;
//                         buffer.data[index + 2] = b;
//                         buffer.data[index + 3] = a;
//
//                         index += 4;
//                 }
//                 context.putImageData(buffer, 0, 0);
//         }, 1000 / 60)
// }


