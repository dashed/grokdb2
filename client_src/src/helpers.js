
// alternative to Symbol()
let _objKey = 1;
const genKey = function() {
    return '__genKey_' + String(_objKey++);
};

module.exports = {
    genKey
};
