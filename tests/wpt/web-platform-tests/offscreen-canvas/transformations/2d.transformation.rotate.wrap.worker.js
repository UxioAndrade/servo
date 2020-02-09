// DO NOT EDIT! This test has been generated by tools/gentest.py.
// OffscreenCanvas test in a worker:2d.transformation.rotate.wrap
// Description:rotate() wraps large positive values correctly
// Note:

importScripts("/resources/testharness.js");
importScripts("/2dcontext/resources/canvas-tests.js");

var t = async_test("rotate() wraps large positive values correctly");
var t_pass = t.done.bind(t);
var t_fail = t.step_func(function(reason) {
    throw reason;
});
t.step(function() {

var offscreenCanvas = new OffscreenCanvas(100, 50);
var ctx = offscreenCanvas.getContext('2d');

ctx.fillStyle = '#f00';
ctx.fillRect(0, 0, 100, 50);
ctx.rotate(Math.PI * (1 + 4096)); // == pi (mod 2*pi)
// We need about pi +/- 0.001 in order to get correct-looking results
// 32-bit floats can store pi*4097 with precision 2^-10, so that should
// be safe enough on reasonable implementations
ctx.fillStyle = '#0f0';
ctx.fillRect(-100, -50, 100, 50);
_assertPixel(offscreenCanvas, 50,25, 0,255,0,255, "50,25", "0,255,0,255");
_assertPixel(offscreenCanvas, 98,2, 0,255,0,255, "98,2", "0,255,0,255");
_assertPixel(offscreenCanvas, 98,47, 0,255,0,255, "98,47", "0,255,0,255");
t.done();

});
done();
