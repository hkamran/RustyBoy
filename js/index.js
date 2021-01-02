import {HEIGHT, Screen, WIDTH} from './screen';
import tippy from 'tippy.js';

tippy('[data-tippy-content]');

import("../pkg/index.js")
    .catch(console.error)
    .then(wasm => {
        window.wasm = wasm;

        let gameboy = wasm.Console.new();
        window.gameboy = gameboy;
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
        fileReader.readAsArrayBuffer(input);
        setupSound();
}

let canvas = document.getElementById('screen');
window.runningFlag = true;

// Web-gl Rendering
window.screen = new Screen(canvas);
window.synth = new Tone.Synth().toDestination();
window.runRustyBoy = () => {
        setTimeout(function() {
                if (runningFlag) requestAnimationFrame(window.runRustyBoy);
                window.gameboy.execute_ticks(27756);
                let frame = window.gameboy.get_frame();
                let buffer = screen.createBuffer();
                buffer.data.set(frame);
                window.synth.triggerAttackRelease("C4", "8n");
                screen.render(buffer);
        }, 1000 / 60);
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


window.audioCtx = new (window.AudioContext || window.webkitAudioContext)();
window.oscList = [];
window.masterGainNode = null;

window.setupSound = () => {
  window.masterGainNode = audioCtx.createGain();
  window.masterGainNode.connect(audioCtx.destination);
  window.masterGainNode.gain.vaule = 1;

  let sineTerms = new Float32Array([0, 0, 1, 0, 1]);
  let cosineTerms = new Float32Array(sineTerms.length);
  let customWaveform = audioCtx.createPeriodicWave(cosineTerms, sineTerms);

  for (let i=0; i<9; i++) {
    oscList[i] = {};
  }
}

window.playTone = () => {
  let osc = audioCtx.createOscillator();
  osc.connect(window.masterGainNode);
  let type = 'sine';
  osc.frequency.value = '20.0';
  osc.start();
  return osc;

}

window.volume = (val) => {
  masterGainNode.gain.setValueAtTime(val, audioCtx.currentTime);
}
