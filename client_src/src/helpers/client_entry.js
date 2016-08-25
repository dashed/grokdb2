// helper for client entry files

// TODO: deferred to cdn: https://cdnjs.com/libraries/babel-polyfill
// require('babel-polyfill');

const ReactDOM = require('react-dom');

const rehydrate = require('helpers/hydrate');


module.exports = (maker, preRenderState, postRenderState, mountTarget) => {

    // NOTES:
    // - preRenderState := inject state before initial render.
    //                     useful for injecting state that doesn't cause UI change
    //
    // - postRenderState := inject state after initial render. this is used for statically compiled react components

    const { component, store } = maker(preRenderState);

    if(!postRenderState) {
        if(process.env.NODE_ENV !== 'production') {
            const invariant = require('invariant');
            invariant(maker.initialState, 'maker.initialState not set');
        }
        postRenderState = maker.initialState;
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

        if(maker.afterRender) {
            maker.afterRender(store);
        }

    };

    if(maker.preRender) {
        maker.preRender();
    }

    ReactDOM.render(
        component,
        mountTarget,
        afterRender
    );
};
