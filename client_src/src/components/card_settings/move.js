require('global/normalize');

const React = require('react');
const ReactDOM = require('react-dom');

const {Provider, connect} = require('react-redux');
const classnames = require('classnames');
const get = require('lodash/get');

const {reduceIn} = require('lib/redux-tree');

const fetch = require('fetch-ponyfill')({
    Promise: require('bluebird')
});

const jsonDecode = require('helpers/json_decode');

const {
    DECK_ID,

    ERROR,
    ERROR_MESSAGE,

} = require('global/constants');

const CONFIRM_MOVE = 'CONFIRM_MOVE';
const MOVE_TO = 'MOVE_TO';
const SUBMITTING = 'SUBMITTING';

/* react components */

const ErrorComponent = require('components/dumb/error');

const __MoveButton = function(props) {

    const {dispatch, submitting} = props;

    if(props.shouldConfirmMove) {
        return (
            <a
                className={classnames('button is-bold is-outlined', {
                    'is-disabled': submitting,
                    'is-loading': submitting
                })}
                onClick={confirmMove(dispatch, false)}
            >
                {'Cancel'}
            </a>
        );
    }

    return (
        <a
            className={classnames('button is-primary is-bold is-outlined', {
                'is-disabled': submitting,
                'is-loading': submitting
            })}
            onClick={confirmMove(dispatch, true)}
        >
            {'Move to this deck'}
        </a>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __MoveButton.propTypes = {
        shouldConfirmMove: React.PropTypes.bool.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
    };
}

const MoveButton = connect(
    // mapStateToProps
    (state) => {
        return {
            shouldConfirmMove: state[CONFIRM_MOVE],
            submitting: state[SUBMITTING]
        };
    }
)(__MoveButton);

const __LeftSideMoveButton = function(props) {

    if(!props.shouldConfirmMove) {
        return null;
    }

    const {submitting} = props;

    const onClick = (event) => {
        event.preventDefault();
        moveCard(props.dispatch, props.moveURL, props.deckID, submitting);
    };

    return (
        <a
            className={classnames('button is-bold is-danger', {
                'is-disabled': submitting,
                'is-loading': submitting
            })}
            onClick={onClick}
        >
            {'Move'}
        </a>
    );

};

if(process.env.NODE_ENV !== 'production') {
    __LeftSideMoveButton.propTypes = {
        shouldConfirmMove: React.PropTypes.bool.isRequired,
        submitting: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
        moveURL: React.PropTypes.string.isRequired,
        deckID: React.PropTypes.number.isRequired,
    };
}

const LeftSideMoveButton = connect(
    // mapStateToProps
    (state) => {
        return {
            shouldConfirmMove: state[CONFIRM_MOVE],
            submitting: state[SUBMITTING],
            moveURL: state[MOVE_TO],
            deckID: state[DECK_ID]
        };
    }
)(__LeftSideMoveButton);

const __ErrorMoveButton = function(props) {

    const {dispatch, error} = props;

    return (<ErrorComponent error={error && error.message || ''} onConfirm={confirmError(dispatch)} />);
};

if(process.env.NODE_ENV !== 'production') {
    __ErrorMoveButton.propTypes = {
        dispatch: React.PropTypes.func.isRequired,
        error: React.PropTypes.object.isRequired,
    };
}

const ErrorMoveButton = connect(
    // mapStateToProps
    (state) => {
        return {
            error: state[ERROR]
        };
    }
)(__ErrorMoveButton);

/* redux action dispatchers */
// NOTE: FSA compliant

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

const defaultRESTError = 'Unable to move card. Please try again.';
const moveCard = function(dispatch, moveURL, deckID, submitting = true) {

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

    const moveRequest = {
        deck_id: deckID
    };

    fetch(moveURL, {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(moveRequest)
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

            window.location.href = jsonResponse.payload.redirect_to;

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

const confirmMove = function(dispatch, confirm = false) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                boolReducer,
                // path
                [CONFIRM_MOVE],
                // action
                {
                    type: confirm
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

    [DECK_ID]: 0,
    [MOVE_TO]: '',

    [CONFIRM_MOVE]: false,

    [SUBMITTING]: false,

    [ERROR]: errorReducer(),
};

/* exports */

const createStore = require('helpers/create_store');

module.exports = function(elem, deckID, moveTo) {

    const cloneDeep = require('lodash/cloneDeep');

    const __initialState = cloneDeep(initialState);

    __initialState[DECK_ID] = Number(deckID);
    __initialState[MOVE_TO] = String(moveTo);

    const store = createStore(__initialState);

    ReactDOM.render(
        <Provider store={store}>
            <MoveButton />
        </Provider>,
        elem,
    );

    ReactDOM.render(
        <Provider store={store}>
            <LeftSideMoveButton />
        </Provider>,
        document.getElementById('' + deckID + '-confirm'),
    );

    ReactDOM.render(
        <Provider store={store}>
            <ErrorMoveButton />
        </Provider>,
        document.getElementById('' + deckID + '-error'),
    );

};
