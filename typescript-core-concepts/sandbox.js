// Type basics
var character = "Solomon";
var age = 10; // number types floating and integer
var isLayerOne = false;
// charcter = 20 is not a valid statement
character = "Solomon Web3";
// age = "Satoshi"
age = 40;
// isLayerOne = "yes";
isLayerOne = true;
// Functions param types
var circ = function (diameter) {
    // Math s an object in JS/TS
    return diameter * Math.PI;
};
console.log(circ(7.9));

// type cheking is done at compile time, not at runtime.
