// rehydrate
//
// terminology: http://stackoverflow.com/questions/29824908/what-does-dehydrate-and-rehydrate-stand-for-in-fluxible

const REHYDRATE = Symbol('REHYDRATE');

// reducer
module.exports = (state, action) => {

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
