// draw.js - mdbook-draw canvas initializer
// This script runs after the page loads and wires up every canvas
// that the Rust preprocessor injected.

// "DOMContentLoaded" fires when HTML is fully parsed.
// Think of it as "wait until the page is ready, then run our code".
document.addEventListener("DOMContentLoaded", function () {

  // Find every toolbar the preprocessor emitted.
  // querySelectorAll returns a list of matching elements.
  var toolbars = document.querySelectorAll(".mdbook-draw-toolbar");

  toolbars.forEach(function (toolbar) {
    // Each toolbar has a data-canvas-id attribute telling us which
    // canvas it belongs to (matches the id= from the draw block).
    var canvasId = toolbar.getAttribute("data-canvas-id");
    var canvas   = document.getElementById(canvasId);

    if (!canvas) return; // safety: skip if canvas not found

    var ctx      = canvas.getContext("2d"); // the 2D drawing API
    var drawing  = false;                   // are we currently drawing?
    var tool     = "pencil";                // active tool
    var color    = "#000000";               // active color
    var brushSize = 4;                      // stroke width in pixels

    // --- Toolbar buttons ---

    // Find all buttons inside this toolbar
    var buttons = toolbar.querySelectorAll("button[data-tool]");
    buttons.forEach(function (btn) {
      btn.addEventListener("click", function () {
        var t = btn.getAttribute("data-tool");
        if (t === "clear") {
          // Clear the entire canvas and re-fill with background color
          ctx.clearRect(0, 0, canvas.width, canvas.height);
          ctx.fillStyle = canvas.getAttribute("data-background") || "#ffffff";
          ctx.fillRect(0, 0, canvas.width, canvas.height);
        } else {
          tool = t; // switch to pencil or eraser
        }
      });
    });

    // Color picker
    var colorInput = toolbar.querySelector("input[data-role='color']");
    if (colorInput) {
      colorInput.addEventListener("input", function () {
        color = colorInput.value;
      });
    }

    // Brush size slider
    var sizeInput = toolbar.querySelector("input[data-role='size']");
    if (sizeInput) {
      sizeInput.addEventListener("input", function () {
        brushSize = parseInt(sizeInput.value, 10);
      });
    }

    // --- Drawing logic ---
    // We track mouse position relative to the canvas, not the page.

    function getPos(e) {
      // getBoundingClientRect gives canvas position on screen
      var rect = canvas.getBoundingClientRect();
      return {
        x: e.clientX - rect.left,
        y: e.clientY - rect.top
      };
    }

    canvas.addEventListener("mousedown", function (e) {
      drawing = true;
      var pos = getPos(e);
      ctx.beginPath();           // start a new line path
      ctx.moveTo(pos.x, pos.y);  // "pick up the pen" at this point
    });

    canvas.addEventListener("mousemove", function (e) {
      if (!drawing) return;      // only draw while button is held

      var pos = getPos(e);

      if (tool === "eraser") {
        // Eraser: paint with the background color
        ctx.strokeStyle = canvas.getAttribute("data-background") || "#ffffff";
        ctx.lineWidth   = brushSize * 3; // eraser is wider than pencil
      } else {
        // Pencil: use the chosen color
        ctx.strokeStyle = color;
        ctx.lineWidth   = brushSize;
      }

      ctx.lineCap  = "round";    // rounded stroke ends look nicer
      ctx.lineJoin = "round";    // smooth corners when changing direction
      ctx.lineTo(pos.x, pos.y);  // draw line to current mouse position
      ctx.stroke();              // actually paint it on the canvas
    });

    canvas.addEventListener("mouseup",    function () { drawing = false; });
    canvas.addEventListener("mouseleave", function () { drawing = false; });
  });
});
