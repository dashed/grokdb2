const transform = function(jsonResponse) {

    const error = (jsonResponse && jsonResponse.error) ? jsonResponse.error : null;
    const payload = (jsonResponse && jsonResponse.payload) ? jsonResponse.payload : null;

    return {
        error: error,
        payload: payload
    };
};

module.exports = function(response) {
    // NOTE: response generated from github's fetch (https://github.com/github/fetch)

    try {
        return transform(response.json());
    } catch(_) {
        // NOTE: error swallowed here
        return transform();
    }

    return transform();
}
