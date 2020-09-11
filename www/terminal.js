var width  = 80;
var height = 40;

var screen = {lines: [], cursor_x: 0, cursor_y: 0};
var term = document.getElementById("terminal");

var tmp;
for (var y = 0; y < height; y++) {
    tmp = [];

    for (var x = 0; x < width; x++) {
        tmp.push(' ');
    }

    screen.lines.push(tmp);
}

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

async function get_keydown() {
    return new Promise(function (resolve,reject) {
            document.addEventListener("keydown", function (e) {
                resolve(e.key)
            })
        }
    );
}

async function getch() {
    while (true) {
        let key = await get_keydown();
        switch (key) {
            case "Shift": case "Control": case "Meta": case "Alt": continue;
            default:
                return key;
        }
    }
}

function get_max_x() {
    return width;
}

function get_max_y() {
    return width;
}

function get_cur_yx() {
    return [screen.cursor_y, screen.cursor_x];
}

function mv(x, y) {
    x -= 1;
    y -= 1;

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
        screen.lines[screen.cursor_y][screen.cursor_x + x] = str.substr(x, 1);
    }
}
