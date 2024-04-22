// Type basics
let character = "Solomon";
let age = 10; // number types floating and integer
let isLayerOne = false;

// charcter = 20 is not a valid statement
character = "Solomon Web3";

// age = "Satoshi"
age = 40;

// isLayerOne = "yes";
isLayerOne = true;

// Functions param types
const circ = (diameter: number) => {
    // Math s an object in JS/TS
    return diameter*Math.PI;
}

console.log(circ(7.9));