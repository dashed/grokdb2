// reducer for rehydration
//
// terminology: http://stackoverflow.com/questions/29824908/what-does-dehydrate-and-rehydrate-stand-for-in-fluxible

const merge = require('lodash/merge');

// const REHYDRATE = Symbol('REHYDRATE');
//
// This is cheaper alternative to Symbol.
const REHYDRATE = ['REHYDRATE'];
const HOT_PATH = ['HOT_PATH'];

const IDENTITY = function(x) {
    return x;
};

// reducer factory
module.exports = (fallbackReducer = IDENTITY) => {

    let isHotPath = false;

    return (state, action) => {

        if (process.env.NODE_ENV !== 'production') {
            if(!action.type) {
                console.error(`Action not FSA. Given ${action}`);
            }
        }

        if(isHotPath) {
            return fallbackReducer(state, action);
        }

        switch(action.type) {

        case REHYDRATE:

            // rationale: action.payload may be staler than state

            return merge({}, action.payload, state);

        case HOT_PATH:
            isHotPath = true;
            return state;

        default:
            return fallbackReducer(state, action);
        }

        // unreachable!();
    };
};

// action creators

module.exports.hydrate = (state) => {
    return {
        type: REHYDRATE,
        payload: state
    };
};

module.exports.hotpath = () => {
    return {
        type: HOT_PATH
    };
};
