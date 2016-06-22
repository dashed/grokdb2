// entry file

const ReactDOM = require('react-dom');


const initialState = window.__INITIAL_STATE__;

const deck_review = require('./components/deck_review');

// rehydrate
let firstRender = false;
const afterRender = () => {

    if(firstRender) {
        return;
    }
    firstRender = true;

    console.log('finished render');
};

ReactDOM.render(
    deck_review(initialState),
    document.getElementById('deck-review-container'),
    afterRender
);
