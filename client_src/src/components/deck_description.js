require('babel-polyfill');
global.Promise = require('bluebird');

const React = require('react');

const {Provider, connect} = require('react-redux');
const {reduxForm, reducer: reduxformReducer} = require('redux-form');
const classnames = require('classnames');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});


const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

    DECK_DESCRIPTION,

    POST_TO
} = require('global/constants');

const {reduceIn, makeReducer} = require('lib/redux-tree');

/* react components */

const DeckDescriptionContainer = function() {
    return (
        <div>lol</div>
    );
};

/* default state */

const initialState = {

    [POST_TO]: '',

    [DECK_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        // NOTE: contents is stored and handled by redux-form
    },

    // redux-form. generate initial state.
    form: reduxformReducer()

};

module.exports = function(preRenderState) {

    if(preRenderState) {
        preRenderState = merge({}, initialState, preRenderState);
    } else {
        preRenderState = initialState;
    }

    const rehydrateFactory = require('helpers/hydrate');
    const {createStore, applyMiddleware} = require('redux');

    // TODO: refactor to module
    const middleware = () => {

        if(process.env.NODE_ENV !== 'production') {

            const createLogger = require('redux-logger');
            const logger = createLogger();

            return applyMiddleware(logger);
        }

        return applyMiddleware();
    };

    const store = createStore(makeReducer({
        reducer: rehydrateFactory(formReducer)
    }), preRenderState, middleware());

    const component = (
        <Provider store={store}>
            <DeckDescriptionContainer />
        </Provider>
    );

    return {
        store,
        component
    };

};

module.exports.initialState = initialState;
