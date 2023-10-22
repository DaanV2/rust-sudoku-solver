var table = document.getElementById("sudoku");

for (var i = 0; i < 9; i++) {
    var row = table.insertRow(i);
    row.setAttribute("class", "row row_" + i);
    row.setAttribute("id", "row_" + i);

    for (var j = 0; j < 9; j++) {
        var cell = row.insertCell(j);
        cell.setAttribute("class", "col col_" + j);
        cell.setAttribute("id", `c_${i}_${j}`);

        cell.innerHTML = `<input type='text' class='cell' id='cell_${i*9+j}' placeholder="" maxlength='1' size='1' onkeyup='checkInput(this)' onchange='cell_changed(this)'>`;
    }
}

function cell_changed(c) {
    to_query();
}

function setMessage(text, classes) {
    text = text || "";
    const t = document.getElementById("text");
    t.innerHTML = text;
    t.hidden = text.length == 0;

    if (classes) {
        t.setAttribute("class", classes);
    }
}

//Check if grid was enter by query string, which is a string of 81 characters representing the grid
var urlParams = new URLSearchParams(window.location.search);
var grid = urlParams.get('grid');
if (grid != null) {
    var data = { cells: [] };
    for (var i = 0; i < grid.length; i++) {
        var c = grid.charAt(i);
        c = c == '.' ? 0 : parseInt(c);
        data.cells.push({
            value: c,
            possible: {}
        });
    }
    set_sudoku(data);
}


function checkInput(input) {
    var value = input.value;
    if (value.length > 1) {
        input.value = value.charAt(0);
    }
    if (value.length == 1) {
        if (value < '1' || value > '9') {
            input.value = '';
        }
    }
}

var solve = document.getElementById("solve");
solve.onclick = function () {
    var data = get_sudoku();

    console.log("sending", data);
    var req = new Request('/api/v1/solve', {
        method: 'POST',
        body: JSON.stringify(data),
        headers: new Headers({
            'Content-Type': 'application/json'
        })
    });

    fetch(req)
        .then(function (response) {
            return response.json();
        })
        .then(set_sudoku)
        .catch(err => setMessage(err, "error"));
}

var solve_once = document.getElementById("solve_once");
solve_once.onclick = function () {
    var data = get_sudoku();

    console.log("sending", data);
    var req = new Request('/api/v1/solve/once', {
        method: 'POST',
        body: JSON.stringify(data),
        headers: new Headers({
            'Content-Type': 'application/json'
        })
    });

    fetch(req)
        .then(function (response) {
            return response.json();
        })
        .then(set_sudoku)
        .catch(err => setMessage(err, "error"));
}

var filled = document.getElementById("filled");
filled.onclick = function () {
    var req = new Request('/api/v1/filled', {
        method: 'GET',
    });

    fetch(req)
        .then(function (response) {
            return response.json();
        })
        .then(set_sudoku)
        .catch(err => setMessage(err, "error"));
}

function get_sudoku() {
    var cells = document.getElementsByClassName("cell");
    var data = [];
    for (var i = 0; i < cells.length; i++) {
        var value = cells[i].value;
        if (value == '') {
            value = 0;
        } else {
            value = parseInt(value);
        }

        data.push(value);
    }

    return { cells: data };
}

function set_sudoku(data) {
    console.log("received", data);
    var cells = document.getElementsByClassName("cell");

    var annotations = {
        iterations: data.iterations,
        result: data.result,
    };
    setMessage(`<code>\n${JSON.stringify(annotations, null, 4)}\n</code>`, "info");

    for (var i = 0; i < cells.length; i++) {
        //Get id from cell and look up value in data
        var id = cells[i].id;
        //Remove "cell_" from id
        id = id.substring(5);
        //Convert to int
        id = parseInt(id);
        var c = data.cells[id];
        if (c == undefined) {
            continue;
        }

        var value = c.value;
        if (value == 0 || value == undefined) {
            value = '';

            //Use placeholder to show possible values
            var placeholder = '';
            for (var j = 0; j < 10; j++) {
                if (c.possible["p"+j] == true) {
                    placeholder += j;
                }
            }
        }

        cells[i].setAttribute("placeholder", placeholder);
        cells[i].value = value;
    }

    to_query();
}

function to_query() {
    //Takes the current grid and converts it to a query string
    var cells = document.getElementsByClassName("cell");
    var query = '?grid=';
    for (var i = 0; i < cells.length; i++) {
        var c = cells[i].value;
        query += c == '' ? '.' : c;
    }

    //Update url
    window.history.pushState({}, '', query);
}