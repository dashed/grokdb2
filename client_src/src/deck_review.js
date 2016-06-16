// TODO: this is fucking huge
// require('babel-polyfill');

const React = require('react');
const ReactDOM = require('react-dom');
const {Provider, connect} = require('react-redux');
const {createStore} = require('redux');
const classnames = require('classnames');
const TextareaAutosize = require('react-textarea-autosize').default;
// const _ = require('lodash');
// const {combineReducers} = require('redux');

// const combineReducers = require('./combineReducers');
// NOTE: above won't work; below is used to be in parity with redux upstream
// import combineReducers from './combineReducers';

const {/*serializeProps, deserializeProps,*/ genKey} = require('./helpers');
const {applyReducer, reducer: treeReducer} = require('./redux-tree');

/* react components */

// const styleA = {
//     marginTop: 0,
//     marginBottom: 0
// };

// TODO: move to its own file
// const CardTitleReviewComponent = React.createClass({
//     render: function() {
//         return (
//             <h5 style={styleA} className='text-break'>
//                 <small className='label'>{'Reviewing'}</small>
//                 {' '}
//                 {'What does the fox say?'}
//             </h5>
//         );
//     }
// });



// TODO: move to its own file for re-use
const __CardReviewTabsComponent = function(props) {

    const tabType = props[TAB];
    const {dispatch} = props;

    return (
        <div className='btn-group btn-group-block'>
            <a
                onClick={switchTab(dispatch, TAB_QUESTION)}
                href='#'
                className = {classnames('btn', {'btn-primary': tabType === TAB_QUESTION})}>
                {'Question'}
            </a>
            <a
                onClick={switchTab(dispatch, TAB_DESCRIPTION)}
                href='#'
                className = {classnames('btn', {'btn-primary': tabType === TAB_DESCRIPTION})}>
                {'Description'}
            </a>
        </div>
    );
}

const CardReviewTabsComponent = connect(
    // mapStateToProps
    (state) => {
        return{
            [TAB]: state[TAB]
        };
    }
)(__CardReviewTabsComponent);

const __RenderSourceComponent = function(props) {

    const {dispatch, switchTab} = props;
    const currentTab = props[MARKDOWN_VIEW];

    return (
        <ul className='tab'>
            <li className = {classnames('tab-item', {'active': currentTab === MARKDOWN_VIEW_RENDER})}>
                <a
                    href='#'
                    onClick={switchTab(dispatch, MARKDOWN_VIEW_RENDER)}>
                    {'Render'}
                </a>
            </li>
            <li className = {classnames('tab-item', {'active': currentTab === MARKDOWN_VIEW_SOURCE})}>
                <a
                    href='#'
                    onClick={switchTab(dispatch, MARKDOWN_VIEW_SOURCE)}>
                    {'Source'}
                </a>
            </li>
        </ul>
    );
}

const RenderSourceComponent = connect(

    // mapStateToProps
    (state) => {
        const currentCardTab = state[TAB];
        return{
            [MARKDOWN_VIEW]: state[currentCardTab][MARKDOWN_VIEW],
            switchTab: (dispatch, markdownView) => switchMarkdownView(dispatch, currentCardTab, markdownView)
        };
    },

    // mapDispatchToProps
    // (dispatch) => {
    //     return {
    //         switchTab:
    //     };
    // }
)(__RenderSourceComponent);

const ReviewScoreCommitComponent = function(/* props */) {
    return (
        <div className='columns'>
            <div className='column col-9'>
                <a href='#' className='btn btn-block'>
                    {'Reveal Answer'}
                </a>
            </div>
            <div className='column col-3'>
                <a href='#' className='btn btn-block'>
                    {'Skip Card'}
                </a>
            </div>
        </div>
    );
}

const __CardContentsComponent = function(props) {

    const contents = props[CONTENTS]
    const markdownView = props[MARKDOWN_VIEW];

    switch(markdownView) {
    case MARKDOWN_VIEW_RENDER:

        return (
            <div className='columns'>
                <div className='column'>
                    {contents}
                </div>
            </div>
        );

    case MARKDOWN_VIEW_SOURCE:
    default:


        return (
            <div className='columns'>
                <div className='column'>
                    <TextareaAutosize
                        key="textarea"
                        useCacheForDOMMeasurements
                        minRows={6}
                        maxRows={10}
                        className="form-input"
                        // id="deck_source"
                        // placeholder={placeholder}
                        // onChange={this.onSourceChange}
                        value={contents}
                        readOnly={true}
                    />
                </div>
            </div>
        );
    }

}

const CardContentsComponent = connect(

    // mapStateToProps
    (state) => {
        const currentCardTab = state[TAB];
        return {
            [MARKDOWN_VIEW]: state[currentCardTab][MARKDOWN_VIEW],
            [CONTENTS]: state[currentCardTab][CONTENTS]
        };
    }
)(__CardContentsComponent);

const DeckReviewContainer = function(/* props */) {
    return (
        <div>
            <div className='columns'>
                <div className='column'>
                    <CardReviewTabsComponent />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <RenderSourceComponent />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <CardContentsComponent />
                </div>
            </div>
            <div className='columns'>
                <div className='column'>
                    <ReviewScoreCommitComponent />
                </div>
            </div>
        </div>
    );
}

/* constants */

/*
enum Tab {
    Question,
    Answer,
    Description
}
 */
const TAB = genKey();
const TAB_QUESTION = genKey();
const TAB_ANSWER = genKey();
const TAB_DESCRIPTION = genKey();

/*
enum MarkdownView {
    Render,
    Source
}
 */
const MARKDOWN_VIEW = genKey();
const MARKDOWN_VIEW_RENDER = genKey();
const MARKDOWN_VIEW_SOURCE = genKey();

const CONTENTS = genKey();

/* redux action creators */
// NOTE: FSA compliant

const switchTab = function(dispatch, tabType) {
    return function(event) {
        event.preventDefault();
        dispatch(
            applyReducer(
                // path
                [TAB],
                // reducer
                tabReducer,
                // action
                {
                    type: tabType
                }
            )
        );
    }
}

const switchMarkdownView = function(dispatch, target, markdownView) {
    // target: Tab
    return function(event) {
        event.preventDefault();
        dispatch(
            applyReducer(
                // path
                [target, MARKDOWN_VIEW],
                // reducer
                markdownViewReducer,
                // action
                {
                    type: markdownView
                }
            )
        );
    }
}


/* redux reducers */

const tabReducer = function(state = TAB_QUESTION, action) {

    switch(action.type) {
    case TAB_QUESTION:
    case TAB_DESCRIPTION:
        state = action.type;
        break;

    default:
        state = TAB_QUESTION;
    }

    return state;
}

const markdownViewReducer = function(state = MARKDOWN_VIEW_RENDER, action) {

    switch(action.type) {
    case MARKDOWN_VIEW_RENDER:
    case MARKDOWN_VIEW_SOURCE:
        state = action.type;
        break;

    default:
        state = MARKDOWN_VIEW_RENDER;
    }

    return state;
}

/* redux store */

const initialState = {
    [TAB]: TAB_QUESTION,

    [TAB_QUESTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CONTENTS]: 'question'
    },

    [TAB_ANSWER]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CONTENTS]: 'answer'
    },

    [TAB_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CONTENTS]: 'description'
    }
};

const store = createStore(treeReducer, initialState);

/* start */

ReactDOM.render(
    <Provider store={store}>
        <DeckReviewContainer />
    </Provider>,
    document.getElementById('deck-review-container'));
