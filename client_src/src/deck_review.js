// entry file

const ReactDOM = require('react-dom');

const rehydrate = require('helpers/hydrate');

const maker = require('./components/deck_review');

const { component, store } = maker();

const initialState = window.__INITIAL_STATE__
    // TODO: test
    || require('lodash').merge({}, maker.initialState);

// TODO: test
initialState.TAB_QUESTION.CARD_CONTENTS = 'question';

let firstRender = false;
const afterRender = () => {

    if(firstRender) {
        return;
    }
    firstRender = true;

    store.dispatch(rehydrate.hydrate(initialState));

    console.log('finished render');
};

ReactDOM.render(
    component,
    document.getElementById('deck-review-container'),
    afterRender
);
