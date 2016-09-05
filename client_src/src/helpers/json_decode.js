const Promise = require('bluebird');

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
    // NOTE: response.json() returns a Promise

    // TODO: simplify this
    return new Promise(function(resolve) {
        Promise.resolve(response.json())
            .then(function(jsonResponse) {
                resolve(transform(jsonResponse));
            })
            .catch(function() {
                resolve(transform());
            });
    });

};
