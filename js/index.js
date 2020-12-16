import("../pkg/index.js").catch(console.error);
const screen_w = 160;
const scree_h = 144;
const BG_COLOR = "#FFFFFF";

var fileReader = new FileReader();
fileReader.onloadend = e => load_buffer(fileReader.result());

var fileInputElement = document.getElementById("cartridge-input");
fileInputElement.addEventListener("change",
  e => fileReader.readAsArrayBuffer(fileInputElement.files[0])
);
