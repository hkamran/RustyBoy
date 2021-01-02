import {Screen} from './screen';
import {initializeKeyboard} from "./keyboard";

import tippy from 'tippy.js';
import {initializeSound} from "./sound";

tippy('[data-tippy-content]');

import("../pkg/index.js")
    .catch(console.error)
    .then(wasm => {
        window.wasm = wasm;

        let gameboy = wasm.Console.new();
        window.gameboy = gameboy;
        window.Button = window.wasm.Button;

        try {
                initializeKeyboard();
                initializeSound();
        } catch (e) {
                console.error(e);
        }
    });

window.clickLoadRom = () => {
        document.getElementById("cartridge-input").click();
}

window.onFileLoad = (event) => {
        const input = event.target.files[0];
        const fileReader = new FileReader();
        fileReader.onloadend = e => {
                const jsValue = Array.from(new Uint8Array(fileReader.result));
                gameboy.load(jsValue);
                gameboy.reset();
                window.runRustyBoy();
        };
        fileReader.readAsArrayBuffer(input);
        setupSound();
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
