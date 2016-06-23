// helper for client entry files

require('babel-polyfill');

const ReactDOM = require('react-dom');

const rehydrate = require('helpers/hydrate');


module.exports = (maker, initialState, mountTarget) => {

    const { component, store } = maker();

    let firstRender = false;
    const afterRender = () => {

        if(firstRender) {
            return;
        }
        firstRender = true;

        store.dispatch(rehydrate.hydrate(initialState));

        console.log('finished render lol');
    };

    ReactDOM.render(
        component,
        mountTarget,
        afterRender
    );
};
