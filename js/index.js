import("../pkg/index_bg.js").catch(console.error).then(wasm => {
  const screen_w = 160;
  const scree_h = 144;
  const BG_COLOR = "#FFFFFF";

  var fileReader = new FileReader();
  fileReader.onloadend = e => {
    console.log(fileReader.result);
    //wasm.load_buffer(new Uint8Array(fileReader.result))};
    wasm.load_buffer(Array.from(fileReader.result))};
  var fileInputElement = document.getElementById("cartridge-input");
  fileInputElement.addEventListener("change",
    e => fileReader.readAsArrayBuffer(fileInputElement.files[0])
  );
});
