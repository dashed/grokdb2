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

/* react components */

const __MoveButton = function(props) {

    const {dispatch} = props;

    if(props.shouldConfirmMove) {
        return (
            <a
                className='button is-bold is-outlined'
                onClick={confirmMove(dispatch, false)}
            >
                {'Cancel'}
            </a>
        );
    }

    return (
        <a
            className='button is-primary is-bold is-outlined'
            onClick={confirmMove(dispatch, true)}
        >
            {'Move to this deck'}
        </a>
    );
};

if(process.env.NODE_ENV !== 'production') {
    __MoveButton.propTypes = {
        shouldConfirmMove: React.PropTypes.bool.isRequired,
        dispatch: React.PropTypes.func.isRequired,
    };
}

const MoveButton = connect(
    // mapStateToProps
    (state) => {
        return {
            shouldConfirmMove: state[CONFIRM_MOVE]
        };
    }
)(__MoveButton);

const __LeftSideMoveButton = function(props) {

    if(!props.shouldConfirmMove) {
        return null;
    }

    return (
        <a
            className='button is-bold is-danger'
        >
            {'Move'}
        </a>
    );

};

const LeftSideMoveButton = connect(
    // mapStateToProps
    (state) => {
        return {
            shouldConfirmMove: state[CONFIRM_MOVE]
        };
    }
)(__LeftSideMoveButton);

/* redux action dispatchers */
// NOTE: FSA compliant

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

    [CONFIRM_MOVE]: false,

    [ERROR]: errorReducer(),
};

/* exports */

const createStore = require('helpers/create_store');

module.exports = function(elem, deckID) {

    initialState[DECK_ID] = deckID;

    const store = createStore(initialState);

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

};
