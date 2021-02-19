var width  = 80;
var height = 40;

var screen = {lines: [], cursor_x: 0, cursor_y: 0};
var term = document.getElementById("terminal");

function init_lines() {
    screen.lines = [];
    var tmp;
    for (var y = 0; y < height; y++) {
        tmp = [];

        for (var x = 0; x < width; x++) {
            tmp.push(' ');
        }

        screen.lines.push(tmp);
    }
}

init_lines()
refresh()

function escapeHTML(str) {
    return new Option(str).innerHTML;
}

function refresh() {
    var out = '';

    for (var y = 0; y < screen.lines.length; y++) {
        for (var x = 0; x < screen.lines[y].length; x++) {
            if (x == screen.cursor_x && y == screen.cursor_y) {
                out += '<span class="cursor">' +
                       escapeHTML(screen.lines[y][x]) +
                       '</span>';
            } else {
                out += escapeHTML(screen.lines[y][x]);
            }
        }
        out += '\n'
    }

    term.innerHTML = out;
}

let doRefresh = false;

function refresh_export() {
    doRefresh = true;
    setTimeout(() => {doRefresh = false}, 15);
}

setInterval(() => {if (doRefresh) {refresh()}}, 15)

function get_max_x() {
    return width;
}

function get_max_y() {
    return height;
}

function get_cur_yx() {
    return [screen.cursor_y + 1, screen.cursor_x + 1];
}

function mv(y, x) {
    if (x >= 0 && x < width && y >= 0 && y < height) {
        screen.cursor_x = x;
        screen.cursor_y = y;
    }
}

function clrtoeol() {
    for (var x = screen.cursor_x; x < width; x++) {
        screen.lines[screen.cursor_y][x] = " ";
    }
}

function addstr(str) {
    if (screen.cursor_x + str.length > width) {
        str = str.substr(0, width - screen.cursor_x);
    }

    for (var x = 0; x < str.length; x++) {
        screen.lines[screen.cursor_y][screen.cursor_x + x] = str.substring(x, x + 1);
    }
}
