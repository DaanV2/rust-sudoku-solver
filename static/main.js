var table = document.getElementById("sudoku");

for (var i = 0; i < 9; i++) {
    var row = table.insertRow(i);
    row.setAttribute("class", "row row_" + i);
    row.setAttribute("id", "row_" + i);

    for (var j = 0; j < 9; j++) {
        var cell = row.insertCell(j);
        cell.setAttribute("class", "col col_" + j);
        cell.setAttribute("id", "col_" + j);

        cell.innerHTML = `<input type='text' class='cell' id='cell_${i*9+j}' placeholder="" maxlength='1' size='1' onkeyup='checkInput(this)'>`;
    }
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
        .then(set_sudoku);
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
        .then(set_sudoku);
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
        .then(set_sudoku);
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
    document.getElementById("text").innerHTML = `<code>\n${JSON.stringify(annotations, null, 4)}\n</code>`;

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
}