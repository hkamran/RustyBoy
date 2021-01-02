import * as Tone from "tone";

export function initializeSound() {
    window.synth = new Tone.Synth().toDestination();
    window.audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    window.oscList = [];
    window.masterGainNode = null;
}


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
