import("../pkg/index_bg.js")
    .catch(console.error)
    .then(wasm => {
        window.wasm = wasm;

        let gameboy = wasm.Console.new();
        console.log(gameboy);
    });

