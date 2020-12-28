import("./rust_webpack_template.js")
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
        };
        fileReader.readAsArrayBuffer(input);
}


