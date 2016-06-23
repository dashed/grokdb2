// Render component to string at default initial state.

const args = process.argv.slice(2);
const ReactDOMServer = require('react-dom/server');

const componentPath = args[0];

// TODO: not used
// const destPath = args[1];

require("babel-register");

const maker = require(componentPath);
const { component, store } = maker();

const result = ReactDOMServer.renderToString(
    component
);

// NOTE: this adds trailing newline
// console.log(result);

process.stdout.write(result);
