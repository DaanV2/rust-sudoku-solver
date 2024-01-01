var wasm = import("./wasm/daanv2-sudoku.js");

var table = document.getElementById("sudoku");

// Setup table
for (var i = 0; i < 9; i++) {
  var row = table.insertRow(i);
  row.setAttribute("class", "row row_" + i);
  row.setAttribute("id", "row_" + i);

  for (var j = 0; j < 9; j++) {
    var cell = row.insertCell(j);
    cell.setAttribute("class", "col col_" + j);
    cell.setAttribute("id", `c_${i}_${j}`);

    const index = i * 9 + j;

    var input = document.createElement("input");
    cell.appendChild(input);

    input.type = "text";
    input.setAttribute("class", "cell");
    input.setAttribute("id", `cell_${index}`);
    input.setAttribute("placeholder", "");
    input.setAttribute("maxlength", "1");
    input.setAttribute("size", "1");
    input.setAttribute("onkeyup", "checkInput(this)");
    input.setAttribute("onchange", "cell_changed(this)");
    input.setAttribute("value", "");
    input.setAttribute("title", `Cell ${index}`);
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
var grid = urlParams.get("grid");
if (grid != null) {
  var data = { cells: [] };
  for (var i = 0; i < grid.length; i++) {
    var c = grid.charAt(i);
    c = c == "." ? 0 : parseInt(c);
    data.cells.push({
      value: c,
      possibilities: {},
    });
  }
  set_sudoku(data);
}

wasm.then(async (wasm_module) => {
  await wasm_module.default();

  var solve = document.getElementById("solve");
  solve.onclick = function () {
    const start = Date.now();
    var data = get_sudoku();
    var output = wasm_module.solve(data);

    annotate(output, Date.now() - start);

    set_sudoku({ cells: output });
  };

  var solve_once = document.getElementById("solve_once");
  solve_once.onclick = function () {
    const start = Date.now();
    var data = get_sudoku();
    var output = wasm_module.solve_once(data);

    annotate(output, Date.now() - start);

    set_sudoku({ cells: output });
  };

  var generate = document.getElementById("generate");
  generate.onclick = function () {
    const start = Date.now();
    var diff = document.getElementById("difficulty").value;
    var seed = Math.random() * Number.MAX_SAFE_INTEGER;
    seed = Math.floor(seed);

    console.log("seed", seed);
    var output = wasm_module.generate_with(diff, seed);
    output.difficulty = diff;
    output.seed = seed;

    annotate(output, Date.now() - start);

    set_sudoku({ cells: output });
  };
});

function annotate(data, time) {
  console.log("Annotation", data);
  const annotation = [];
  if (data.iterations) {
    annotation.push(`<a>Iterations: ${data.iterations}</a>`);
  }
  if (data.result) {
    annotation.push(`<a>Result: ${get_result(data.result)}</a>`);
  }
  if (data.difficulty) {
    annotation.push(`<a>Difficulty: ${data.difficulty}</a>`);
  }
  if (data.seed) {
    annotation.push(`<a>Seed: ${data.seed}</a>`);
  }
  annotation.push(`<a>Time: ${time} ms</a>`);

  setMessage(`<p>${annotation.join("<br>")}</p>`, "info");
  return data;
}

function get_result(result) {
  switch (result) {
    case 0:
      return "nothing";
    case 1:
      return "updated";
    case 2:
      return "solved";
    case 3:
      return "invalid";
  }

  return "unknown";
}

/**
 *
 * @param {{cells:Pick<wasm.Cell, "possibilities" | "value">[] }} data
 */
function set_sudoku(data) {
  console.log("received", data);
  var cells = document.getElementsByClassName("cell");
  var hints = document.getElementById("hints").checked;

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
    var placeholder = ""; //Use placeholder to show possible values
    if (value == 0 || value == undefined) {
      value = "";

      if (hints) {
        for (var j = 0; j < 10; j++) {
          if (c.possibilities[`p${j}`] === true) {
            placeholder += j;
          }
        }
      }
    }

    cells[i].setAttribute("placeholder", placeholder);
    cells[i].value = value;
  }

  to_query();
}

/**
 *
 * @returns {Int32Array}
 */
function get_sudoku() {
  console.log("getting sudoku");
  var cells = document.getElementsByClassName("cell");
  var data = [];
  for (var i = 0; i < cells.length; i++) {
    var value = cells[i].value;
    if (value == "") {
      value = 0;
    } else {
      value = parseInt(value);
    }

    data.push(value);
  }

  console.log("got grid", data);
  return new Int32Array(data);
}

function to_query() {
  //Takes the current grid and converts it to a query string
  var cells = document.getElementsByClassName("cell");
  var query = "?grid=";
  for (var i = 0; i < cells.length; i++) {
    var c = cells[i].value;
    query += c == "" ? "." : c;
  }

  //Update url
  window.history.pushState({}, "", query);
}

function checkInput(input) {
  var value = input.value;
  if (value.length > 1) {
    input.value = value.charAt(0);
  }
  if (value.length == 1) {
    if (value < "1" || value > "9") {
      input.value = "";
    }
  }
}
