// Render component to string at default initial state.

const args = process.argv.slice(2);
const ReactDOMServer = require('react-dom/server');

const componentPath = args[0];

// TODO: not used
// const destPath = args[1];

require("babel-register");

// NOTE: type componentMaker = (initialState) => ReactComponent
const componentMaker = require(componentPath);

const result = ReactDOMServer.renderToString(
    componentMaker(void 0 /* initial state */)
);

// NOTE: this adds trailing newline
// console.log(result);

process.stdout.write(result);
