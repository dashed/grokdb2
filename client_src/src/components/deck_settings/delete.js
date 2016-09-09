require('global/normalize');

const React = require('react');

const {Provider, connect} = require('react-redux');
const classnames = require('classnames');
const get = require('lodash/get');

const {reduceIn} = require('lib/redux-tree');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});

const jsonDecode = require('helpers/json_decode');

const {

    DELETE_TO,

    ERROR,
    ERROR_MESSAGE,

} = require('global/constants');

// TODO: move these constants
const CONFIRM_DELETE = 'CONFIRM_DELETE';
const SUBMITTING = 'SUBMITTING';

/* react components */

const ErrorComponent = require('components/dumb/error');

const __DeleteDeck = function(props) {

    const {submitting, confirmDelete, handleConfirm, error, dispatch} = props;

    if(confirmDelete) {

        const {deleteURL} = props;

        const handleDelete = function(event) {
            event.preventDefault();
            deleteDeck(dispatch, deleteURL, submitting);
        };

        return (
            <div>
                <ErrorComponent error={error && error.message || ''} onConfirm={confirmError(dispatch)} />
                <div className='columns'>
                    <div className='column'>
                        <strong>{'Are you sure you want to delete this deck?'}</strong>
                    </div>
                </div>
                <div className='columns'>
                    <div className='column'>
                        <div className='control is-grouped'>
                            <p className='control'>
                                <a
                                    className={classnames('button is-bold', {
                                        'is-disabled': submitting,
                                        'is-loading': submitting
                                    })}
                                    onClick={handleConfirm(false)}
                                    >
                                    {'Cancel'}
                                </a>
                            </p>
                            <p className='control'>
                                <a
                                    className={classnames('button is-danger is-bold', {
                                        'is-disabled': submitting,
                                        'is-loading': submitting
                                    })}
                                    onClick={handleDelete}
                                    >
                                    {'Delete'}
                                </a>
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        );
    }

    return (
        <div>
            <ErrorComponent error={error && error.message || ''} onConfirm={confirmError(dispatch)} />
            <div className='columns'>
                <div className='column'>
                    <a
                        className={classnames('button is-danger is-bold', {
                            'is-disabled': submitting,
                            'is-loading': submitting
                        })}
                        onClick={handleConfirm(true)}
                        >
                        {'Delete Deck'}
                    </a>
                </div>
            </div>
        </div>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __DeleteDeck.propTypes = {
        submitting: React.PropTypes.bool.isRequired,
        confirmDelete: React.PropTypes.bool.isRequired,
        handleConfirm: React.PropTypes.func.isRequired,
        dispatch: React.PropTypes.func.isRequired,
        deleteURL: React.PropTypes.string.isRequired,
        error: React.PropTypes.object,
    };
}

const DeleteDeck = connect(
    // mapStateToProps
    (state) => {
        return {
            error: state[ERROR],
            submitting: state[SUBMITTING],
            confirmDelete: state[CONFIRM_DELETE],
            deleteURL: state[DELETE_TO],
        };
    },
    // mapDispatchToProps
    (dispatch) => {
        return {
            dispatch,
            handleConfirm: (bool) => {
                return function(event) {
                    event.preventDefault();
                    dispatch(
                        reduceIn(
                            // reducer
                            boolReducer,
                            // path
                            [CONFIRM_DELETE],
                            // action
                            {
                                type: bool
                            }
                        )
                    );
                };
            }
        };
    }
)(__DeleteDeck);

/* redux action dispatchers */
// NOTE: FSA compliant

const defaultRESTError = 'Unable to delete deck. Please try again.';
const deleteDeck = function(dispatch, deleteURL, submitting = true) {

    if(submitting) {
        return;
    }

    dispatch(
        reduceIn(
            // reducer
            boolReducer,
            // path
            [SUBMITTING],
            // action
            {
                type: true
            }
        )
    );

    fetch(deleteURL, {
        method: 'DELETE',
    })
    .then(function(response) {
        return Promise.all([response.status, jsonDecode(response)]);
    })
    .then(function([statusCode, jsonResponse]) {

        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SUBMITTING],
                // action
                {
                    type: false
                }
            )
        );

        switch(statusCode) {
        case 500: // Internal Server Error
        case 400: // Bad Request

            dispatch(
                reduceIn(
                    // reducer
                    errorReducer,
                    // path
                    [ERROR],
                    // action
                    {
                        type: ERROR_MESSAGE,
                        payload: get(jsonResponse, ['error'], defaultRESTError)
                    }
                )
            );

            return;
            break;

        case 200: // Ok

            if(jsonResponse.payload) {

                dispatch(
                    reduceIn(
                        // reducer
                        errorReducer,
                        // path
                        [ERROR],
                        // action
                        {
                            type: ERROR_MESSAGE,
                            payload: get(jsonResponse, ['error'], '')
                        }
                    )
                );

                window.location.href = jsonResponse.payload.redirect_to;

            } else {

                dispatch(
                    reduceIn(
                        // reducer
                        errorReducer,
                        // path
                        [ERROR],
                        // action
                        {
                            type: ERROR_MESSAGE,
                            payload: get(jsonResponse, ['error'], 'Unable to delete this deck.')
                        }
                    )
                );

            }

            return;
            break;

        default: // Unexpected http status code
            dispatch(
                reduceIn(
                    // reducer
                    errorReducer,
                    // path
                    [ERROR],
                    // action
                    {
                        type: ERROR_MESSAGE,
                        payload: get(jsonResponse, ['error'], defaultRESTError)
                    }
                )
            );
        }

    })
    .catch(function(/*err*/) {

        // any other errors
        // console.log('err:', err);

        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [SUBMITTING],
                // action
                {
                    type: false
                }
            )
        );

        dispatch(
            reduceIn(
                // reducer
                errorReducer,
                // path
                [ERROR],
                // action
                {
                    type: ERROR_MESSAGE,
                    payload: defaultRESTError
                }
            )
        );

    });
};

const confirmError = function(dispatch) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                errorReducer,
                // path
                [ERROR],
                // action
                {
                    type: ERROR_MESSAGE,
                    payload: ''
                }
            )
        );
    };
};

/* redux reducers */

const boolReducer = require('reducers/bool');
const errorReducer = require('reducers/error_message');

/* default state */

const initialState = {

    // NOTE: populated by window.__PRE_RENDER_STATE__
    [DELETE_TO]: '',

    [CONFIRM_DELETE]: false,
    [SUBMITTING]: false,

    [ERROR]: errorReducer(),
};

/* exports */

const componentCreator = require('helpers/component_factory');

module.exports = componentCreator(initialState, function(store) {

    const component = (
        <Provider store={store}>
            <DeleteDeck />
        </Provider>
    );

    return component;

});

module.exports.initialState = initialState;
