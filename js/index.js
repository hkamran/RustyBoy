import("../pkg/index_bg.js").catch(console.error).then(wasm => {
  var fileReader = new FileReader();
  fileReader.onloadend = e => {
    console.log(fileReader.result);
    //wasm.load_buffer(new Uint8Array(fileReader.result))};
    wasm.load_buffer(Array.from(new Uint8Array(fileReader.result)))};
  var fileInputElement = document.getElementById("cartridge-input");
  fileInputElement.addEventListener("change",
    e => fileReader.readAsArrayBuffer(fileInputElement.files[0])
  );
});
