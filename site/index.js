import {default as init_wasm, run} from "./gbemu/gbemu.js";

let emu_wasm = null;

async function init() {
    let w = await init_wasm();
    console.log(w);
    set_wasm(w);
}

function set_wasm(w) {
    emu_wasm = w;
}

export { init, run };