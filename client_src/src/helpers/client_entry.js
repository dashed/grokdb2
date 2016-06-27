// helper for client entry files

// TODO: deferred to cdn: https://cdnjs.com/libraries/babel-polyfill
// require('babel-polyfill');

const ReactDOM = require('react-dom');

const rehydrate = require('helpers/hydrate');


module.exports = (maker, preRenderState, postRenderState, mountTarget) => {

    const { component, store } = maker(preRenderState);

    if(process.env.NODE_ENV !== 'production') {
        if(!postRenderState) {
            console.warn('postRenderState not given');
            postRenderState = maker.initialState;
        }
    }

    let firstRender = false;
    const afterRender = () => {

        if(firstRender) {
            return;
        }
        firstRender = true;

        store.dispatch(rehydrate.hydrate(postRenderState));
        store.dispatch(rehydrate.hotpath());

        if(process.env.NODE_ENV !== 'production') {
            console.log('rehydration finished.');
        }

    };

    ReactDOM.render(
        component,
        mountTarget,
        afterRender
    );
};
