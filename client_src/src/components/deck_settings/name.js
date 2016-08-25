require('global/normalize');

const React = require('react');

const assign = require('lodash/assign');

const {Provider, connect} = require('react-redux');
const {reduxForm, reducer: reduxformReducer} = require('redux-form');
const classnames = require('classnames');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});


const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,
    MARKDOWN_CONTENTS,

    DECK_NAME,

    POST_TO,

} = require('global/constants');

const {reduceIn} = require('lib/redux-tree');

/* react components */

const MathJaxLine = require('components/dumb/mathjax_line');

const RenderSourceNameComponent = connect(
    // mapStateToProps
    (state) => {
        return {
            [MARKDOWN_VIEW]: state[DECK_NAME][MARKDOWN_VIEW]
        };
    },
    // mapDispatchToProps
    (dispatch) => {
        return {
            // markdownView := MARKDOWN_VIEW_RENDER | MARKDOWN_VIEW_SOURCE
            switchTab: (markdownView) => {
                return switchMarkdownView(dispatch, [DECK_NAME, MARKDOWN_VIEW], markdownView);
            }
        };
    }
)(require('components/dumb/render_source'));

const __DeckSettingsNameContainer = function(props) {

    const {
        mathjaxifyDeckName,
        fields: { name },
        submitting,
        handleSubmit,
        postURL,
        dispatch,
        originalName
    } = props;

    return (
        <div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceNameComponent
                        extraClasses='is-small'
                        reverse
                    />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <MathJaxLine
                        content={name.value}
                        mathjaxify={mathjaxifyDeckName}
                        notice={'No deck name rendered.  Click on "Source" tab and enter a name.'}
                    >
                        <p className='control'>
                            <input
                                id='input-deck-name'
                                className='input'
                                type='text'
                                placeholder='Enter a deck name'
                                autoFocus
                                {...assign({}, name)}
                            />
                        </p>
                    </MathJaxLine>
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <a
                        className={classnames('button is-success', {
                            'is-disabled':
                                submitting ||
                                String(name.value).trim().length <= 0 ||
                                String(originalName).trim() == String(name.value).trim(),
                            'is-loading': submitting
                        })}
                        onClick={handleSubmit(saveName.bind(null, dispatch, postURL))}>
                        {'Rename'}
                    </a>
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __DeckSettingsNameContainer.propTypes = {
        fields: React.PropTypes.object.isRequired,
        handleSubmit: React.PropTypes.func.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        postURL: React.PropTypes.string.isRequired,
        resetForm: React.PropTypes.func.isRequired,
        dispatch: React.PropTypes.func.isRequired,
        showNoContentMessage: React.PropTypes.bool.isRequired,
        mathjaxifyDeckName: React.PropTypes.bool.isRequired,
        originalName: React.PropTypes.string.isRequired,
    };
}

const deckSettingsNameContainerFactory = function(preRenderState) {

    return reduxForm(

        // config
        {
            form: 'deck_settings_name',
            fields: ['name'],
            initialValues: {
                name: preRenderState[DECK_NAME][MARKDOWN_CONTENTS]
            }
        },

        // mapStateToProps
        (state) => {
            return {
                postURL: state[POST_TO],
                initialValues: {
                    // NOTE: this commits initial value after save
                    name: state[DECK_NAME][MARKDOWN_CONTENTS],
                },
                showNoContentMessage: state[DECK_NAME].showNoContentMessage,
                mathjaxifyDeckName: state[DECK_NAME][MARKDOWN_VIEW] === MARKDOWN_VIEW_RENDER,
                originalName: state[DECK_NAME][MARKDOWN_CONTENTS],
            };
        }

    )(__DeckSettingsNameContainer);

};

/* redux action dispatchers */
// NOTE: FSA compliant

const saveName = function(dispatch, postURL, formData) {

    return new Promise((resolve, reject) => {

        const finalName = String(formData.name).trim();

        fetch(postURL, {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                name: finalName
            })
        })
        .then(function(response) {

            return Promise.all([response.status]);
        }, function(err) {

            // TODO: handle on network failure, etc

            console.log('err:', err);

            reject({
                _error: {
                    message: 'Unable to update deck name.'
                }
            });
        })
        .then(function([statusCode]) {

            switch(statusCode) {
            case 400: // Bad Request
            case 500: // Internal Server Error

                // response.userMessage

                // TODO: error fix
                //
                // http://redux-form.com/5.2.5/#/api/props
                // how to detect errors
                reject({
                    _error: {
                        message: 'Unable to update deck name.'
                    }
                });

                return;
                break;

            case 200: // Ok

                setTimeout(updateDeckNameExternally.bind(null, finalName), 0);

                // update deck name
                dispatch(
                    reduceIn(
                        // reducer
                        markdownContentsReducer,
                        // path
                        [DECK_NAME, MARKDOWN_CONTENTS],
                        // action
                        {
                            type: MARKDOWN_CONTENTS,
                            payload: finalName
                        }
                    )
                );

                // switch out of edit mode
                dispatch(
                    reduceIn(
                        // reducer
                        markdownViewReducer,
                        // path
                        [DECK_NAME, MARKDOWN_VIEW],
                        // action
                        {
                            type: MARKDOWN_VIEW_RENDER
                        }
                    )
                );

                resolve();

                break;

            default: // Unexpected http status code
                reject({
                    _error: {
                        message: 'Unable to update deck name.'
                    }
                });
            }

        }, function(err) {

            // TODO: handle on json parsing fail
            console.log('err:', err);

            reject({
                _error: {
                    message: 'Unable to update deck name.'
                }
            });
        })
        .catch(function(err) {

            // TODO: handle
            console.log('err:', err);

            reject({
                _error: {
                    message: 'Unable to update deck name.'
                }
            });
        });

    });

};

const switchMarkdownView = function(dispatch, path, markdownView) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                path,
                // action
                {
                    type: markdownView
                }
            )
        );
    }
};

/* redux reducers */

const markdownViewReducer = require('reducers/markdown_view');
const markdownContentsReducer = require('reducers/markdown_contents');

/* default state */

const initialState = {

    // NOTE: populated by window.__PRE_RENDER_STATE__
    [POST_TO]: '',

    [DECK_NAME]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_SOURCE,
        [MARKDOWN_CONTENTS]: '',
        showNoContentMessage: false
        // NOTE: contents is stored and handled by redux-form
    },

    // redux-form. generate initial state.
    form: reduxformReducer()

};

/* helpers */

const forEach = require('lodash/forEach');
const ReactDOM = require('react-dom');

const MathJaxRenderInline = require('components/dumb/mathjax_inline');

const updateDeckNameExternally = function(newName) {

    forEach(document.getElementsByClassName('__deck_name'), function(elem) {

        setTimeout(function(){
            ReactDOM.render(
                <MathJaxRenderInline contents={newName} />,
                elem,
            );
        }, 0);

    });

};

/* exports */

const formReducer = require('helpers/form_reducer');
const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const __Component = deckSettingsNameContainerFactory(store.getState());

    const component = (
        <Provider store={store}>
            <__Component />
        </Provider>
    );

    return component;

}, formReducer);

module.exports.initialState = initialState;
