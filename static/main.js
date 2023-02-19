var table = document.getElementById("sudoku");
for (var i = 0; i < 9; i++) {
    var row = table.insertRow(i);
    for (var j = 0; j < 9; j++) {
        var cell = row.insertCell(j);
        cell.innerHTML = `<input type='text' class='cell' id='cell_${i*9+j}' maxlength='1' size='1' onkeyup='checkInput(this)'>`;
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
    document.getElementById("text").innerHTML = JSON.stringify(annotations);

    for (var i = 0; i < cells.length; i++) {
        //Get id from cell and look up value in data
        var id = cells[i].id;
        var c = data.cells[id];
        if (c == undefined) {
            continue;
        }

        var value = c.value;
        if (value == 0 || value == undefined) {
            value = '';
        }

        cells[i].value = value;
    }
}