wasm_bindgen()

document.addEventListener("keydown", function (e) {
    wasm_bindgen.eval_key(e.key);
})
