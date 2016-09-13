const typeReducer = function(state, action) {

    state = action && action.type || state;

    return state;
};

module.exports = typeReducer;
