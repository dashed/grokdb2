// reducer for rehydration
//
// terminology: http://stackoverflow.com/questions/29824908/what-does-dehydrate-and-rehydrate-stand-for-in-fluxible

// const REHYDRATE = Symbol('REHYDRATE');
//
// This is cheaper alternative to Symbol.
const REHYDRATE = ['REHYDRATE'];

// reducer
module.exports = (state, action) => {

    if (process.env.NODE_ENV !== 'production') {
        if(!action.type) {
            console.error(`Action not FSA. Given ${action}`);
        }
    }

    switch(action.type) {

    case REHYDRATE:
        return action.payload;

    default:
        return state;
    }

    // unreachable!();
};

// action creator
module.exports.hydrate = (state) => {
    return {
        type: REHYDRATE,
        payload: state
    };
};
