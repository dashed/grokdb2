const has = require('lodash/has');

const typeReducer = function(state, action) {

    if(action && has(action, 'type')) {
        state = action.type;
    }
    return state;
};

module.exports = typeReducer;
