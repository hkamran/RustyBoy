
export function initializeKeyboard() {
    window.document.onkeydown = onKeyDown.bind(this);
    window.document.onkeyup = onKeyDown.bind(this);
}


export const key_mapping = {
    RIGHT: "D",
    LEFT: "A",
    DOWN: "S",
    UP: "W",
    START: "B",
    SELECT: "N",
    A: "H",
    B: "J",
}

function onKeyDown(event) {
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

function onKeyUp(event) {
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
