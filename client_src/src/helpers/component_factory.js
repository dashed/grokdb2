const merge = require('lodash/merge');

const createStore = require('helpers/create_store');

const IDENTITY = function(x) {
    return x;
};

const componentCreator = function(initialState, getComponent, fallbackReducer = IDENTITY) {

    return function(preRenderState) {

        if(preRenderState) {
            preRenderState = merge({}, initialState, preRenderState);
        } else {
            preRenderState = initialState;
        }

        const store = createStore(preRenderState, fallbackReducer);

        const component = getComponent(store);

        return {
            store,
            component
        };

    };
};

module.exports = componentCreator;
