var character = "Solomon";
console.log(character);
var inputs = document.querySelectorAll('input');
console.log(inputs);
inputs.forEach(function (input) {
    console.log(input);
});
// tsc sandbox.ts generates a sandbox.js file so need to manually create a sandbox.js file
// tsc sandbox.ts sandbox.js compiles to sandbox.js if the sandbox.js file is pre-exsitent
// tsc sandbox.ts -w stands for watch. this command allows us to automatically 
// transpile ts into js when changes are made.
