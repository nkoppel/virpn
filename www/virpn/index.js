let term = document.getElementById("terminal");

var width  = 80;
var height = 40;

async function init() {
    await wasm_bindgen();
    dynResize();
}

init();

document.addEventListener("keydown", function (e) {
    wasm_bindgen.eval_key(e.key);
    term.innerHTML = wasm_bindgen.render_html(width, height);
})

function dynResize() {
    var text = document.getElementById("textSize");

    var w = text.clientWidth;
    var h = text.clientHeight;

    width  = Math.trunc(window.innerWidth  / w - 2);
    height = Math.trunc(window.innerHeight / h - 1);

    term.innerHTML = wasm_bindgen.render_html(width, height);
}

let doResize = true;

window.onresize = function () {
    if (doResize) {
        dynResize();

        doResize = false;
        setTimeout(() => {doResize = true}, 15);
    }
}
