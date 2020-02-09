// DO NOT EDIT! This test has been generated by tools/gentest.py.
// OffscreenCanvas test in a worker:2d.path.arc.selfintersect.1
// Description:arc() with lineWidth > 2*radius is drawn sensibly
// Note:

importScripts("/resources/testharness.js");
importScripts("/2dcontext/resources/canvas-tests.js");

var t = async_test("arc() with lineWidth > 2*radius is drawn sensibly");
var t_pass = t.done.bind(t);
var t_fail = t.step_func(function(reason) {
    throw reason;
});
t.step(function() {

var offscreenCanvas = new OffscreenCanvas(100, 50);
var ctx = offscreenCanvas.getContext('2d');

ctx.fillStyle = '#0f0';
ctx.fillRect(0, 0, 100, 50);
ctx.lineWidth = 200;
ctx.strokeStyle = '#f00';
ctx.beginPath();
ctx.arc(100, 50, 25, 0, -Math.PI/2, true);
ctx.stroke();
ctx.beginPath();
ctx.arc(0, 0, 25, 0, -Math.PI/2, true);
ctx.stroke();
_assertPixel(offscreenCanvas, 1,1, 0,255,0,255, "1,1", "0,255,0,255");
_assertPixel(offscreenCanvas, 50,25, 0,255,0,255, "50,25", "0,255,0,255");
t.done();

});
done();
