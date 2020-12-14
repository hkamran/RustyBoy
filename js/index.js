import("../pkg/index.js").catch(console.error);
import { memory } from "rust-webpack-template";

const screen_w = 160;
const scree_h = 144;
const BG_COLOR = "#FFFFFF";
const canvas = document.getElementById("screen");
const ctx = canvas.getContext('2d');
canvas.height = screen_h;
canvas.width = screen_w;
ctx.fillStyle = BG_COLOR;

