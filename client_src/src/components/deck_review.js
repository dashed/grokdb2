// TODO: this is fucking huge
// require('babel-polyfill');

const React = require('react');
// const ReactDOM = require('react-dom');
const {Provider, connect} = require('react-redux');
const {createStore} = require('redux');
const classnames = require('classnames');
const TextareaAutosize = require('react-textarea-autosize').default;


const {reduceIn, makeReducer} = require('redux-tree');

/* constants */

/*
enum Tab {
    Question,
    Answer,
    Description
}
 */
const TAB = 'TAB';
const TAB_QUESTION = 'TAB_QUESTION';
const TAB_ANSWER = 'TAB_ANSWER';
const TAB_DESCRIPTION = 'TAB_DESCRIPTION';

/*
enum MarkdownView {
    Render,
    Source
}
 */
const MARKDOWN_VIEW = 'MARKDOWN_VIEW';
const MARKDOWN_VIEW_RENDER = 'MARKDOWN_VIEW_RENDER';
const MARKDOWN_VIEW_SOURCE = 'MARKDOWN_VIEW_SOURCE';

const CARD_CONTENTS = 'CARD_CONTENTS';

// TODO: clean up below
/*
enum SkipCard {
    Yes,
    No // default
}

enum PerformanceChoice {
    NoChoice,
    Fail,
    Success,
    Reset
}

enum CustomScoreOperation {
    Append,
    Set
}

// UI state machine
enum CardPerformanceControl {
    Initial,
    DefaultChoices,
    CustomScore,
}
 */

const CARD_PERF_CONTROL_VIEW = 'CARD_PERF_CONTROL_VIEW';
const CARD_PERF_CONTROL__INITIAL = 'CARD_PERF_CONTROL__INITIAL'; // empty UI
const CARD_PERF_CONTROL__DEFAULT_CHOICES = 'CARD_PERF_CONTROL__DEFAULT_CHOICES';

/*
enum SkipCardView {
    Initial,
    Confirm
}
 */

const SKIPCARD_VIEW = 'SKIPCARD_VIEW';
const SKIPCARD_INITIAL = 'SKIPCARD_INITIAL';
const SKIPCARD_CONFIRM = 'SKIPCARD_CONFIRM';


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
                href='#question'
                className = {classnames('btn', {'btn-primary': tabType === TAB_QUESTION})}>
                {'Question'}
            </a>
            <a
                onClick={switchTab(dispatch, TAB_DESCRIPTION)}
                href='#description'
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

    // NOTE: switchTab(dispatch: dispatch, next_view: MARKDOWN_VIEW);

    const {dispatch, switchTab} = props;
    const currentTab = props[MARKDOWN_VIEW];

    return (
        <ul className='tab'>
            <li className = {classnames('tab-item', {'active': currentTab === MARKDOWN_VIEW_RENDER})}>
                <a
                    href='#render'
                    onClick={switchTab(dispatch, MARKDOWN_VIEW_RENDER)}>
                    {'Render'}
                </a>
            </li>
            <li className = {classnames('tab-item', {'active': currentTab === MARKDOWN_VIEW_SOURCE})}>
                <a
                    href='#source'
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

const __RevealCommitButtonComponent = function(props) {

    const {dispatch} = props;

    const perfControlView = props[CARD_PERF_CONTROL_VIEW];

    switch(perfControlView) {
    case CARD_PERF_CONTROL__INITIAL:


        return (
            <a
                href='#confirm-skip'
                className='btn btn-block'
                onClick={switchCardPerfControl(dispatch, CARD_PERF_CONTROL__DEFAULT_CHOICES)}>
                {'Reveal Answer'}
            </a>
        );

    case CARD_PERF_CONTROL__DEFAULT_CHOICES:
    default:

        return (
            <a
                href='#confirm-skip'
                className='btn btn-block'
                onClick={() => void 0}>
                {'Commit Score & Next Card'}
            </a>
        );

    }

}

const RevealCommitButtonComponent = connect(
    // mapStateToProps
    (state) => {
        return{
            [CARD_PERF_CONTROL_VIEW]: state[CARD_PERF_CONTROL_VIEW]
        };
    },

)(__RevealCommitButtonComponent);

const __CardPerformanceControlComponent = function(props) {

    const {shouldNotShow} = props;

    if(shouldNotShow) {
        return null;
    }

    // switch(perfControlView) {
    // case CARD_PERF_CONTROL__INITIAL:
    //     return;
    // }

    return (
        <div className='columns'>
            <div className='column col-4'>
                <a
                    href='#confirm-skip'
                    className='btn btn-block'
                    onClick={() => void 0}>
                    {'Fail'}
                </a>
            </div>
            <div className='column col-4'>
                <a
                    href='#confirm-skip'
                    className='btn btn-block'
                    onClick={() => void 0}>
                    {'Success'}
                </a>
            </div>
            <div className='column col-4'>
                <a
                    href='#confirm-skip'
                    className='btn btn-block'
                    onClick={() => void 0}>
                    {'Reset Score'}
                </a>
            </div>
        </div>
    );
}

const CardPerformanceControlComponent = connect(
    // mapStateToProps
    (state) => {
        const shouldNotShow = (state[SKIPCARD_VIEW] === SKIPCARD_CONFIRM) ||
            state[CARD_PERF_CONTROL_VIEW] === CARD_PERF_CONTROL__INITIAL;

        return{
            // [CARD_PERF_CONTROL_VIEW]: state[CARD_PERF_CONTROL_VIEW],
            // shouldShow: !shouldNotShow
            shouldNotShow
        };
    },

)(__CardPerformanceControlComponent);

const CustomScoreButtonComponent = function(props) {
    return (
        <div className='columns'>
            <div className='column'>
                <a
                    href='#custom-score'
                    className='btn btn-block'
                    onClick={() => void 0}>
                    {'Set/Append Custom Score'}
                </a>
            </div>
        </div>
    );
}

const __ReviewScoreCommitComponent = function(props) {

    const {dispatch, showConfirmSkipCard} = props;

    if(showConfirmSkipCard) {

        return (
            <div key='0'>
                <div className='columns'>
                    <div className='column col-6'>
                        <a
                            href='#confirm-skip'
                            className='btn btn-block'
                            onClick={() => void 0}>
                            {'Yes, skip'}
                        </a>
                    </div>
                    <div className='column col-6'>
                        <a
                            href='#cancel-skip'
                            className='btn btn-block btn-primary'
                            onClick={skipCard(dispatch, SKIPCARD_INITIAL)}>
                            {'No, do not skip'}
                        </a>
                    </div>
                </div>
            </div>
        );

    }

    return (
        <div key='1'>
            <CustomScoreButtonComponent />
            <CardPerformanceControlComponent />
            <div className='columns'>
                <div className='column col-9'>
                    <RevealCommitButtonComponent />
                </div>
                <div className='column col-3'>
                    <a
                        href='#skip'
                        className='btn btn-block'
                        onClick={skipCard(dispatch, SKIPCARD_CONFIRM)}>
                        {'Skip Card'}
                    </a>
                </div>
            </div>
        </div>
    );
}

const ReviewScoreCommitComponent = connect(

    // mapStateToProps
    (state) => {
        return{
            showConfirmSkipCard: state[SKIPCARD_VIEW] === SKIPCARD_CONFIRM
        };
    },


)(__ReviewScoreCommitComponent);

const CardSource = function(props) {
    return <TextareaAutosize style={props.style}
        key='textarea'
        useCacheForDOMMeasurements
        minRows={6}
        maxRows={10}
        className='form-input'
        // id="deck_source"
        // placeholder={placeholder}
        // onChange={this.onSourceChange}
        value={props.contents}
        readOnly={true}
    />;
}

const __DISPLAY_NONE = {display: 'none'};
const __CardContentsComponent = function(props) {

    const currentTab = props[TAB];
    const contents = props[currentTab]
    const markdownView = props[MARKDOWN_VIEW];


    let sourceStyles = {
        [TAB_QUESTION]: __DISPLAY_NONE,
        [TAB_ANSWER]: __DISPLAY_NONE,
        [TAB_DESCRIPTION]: __DISPLAY_NONE
    };
    let renderStyle = {};

    switch(markdownView) {
    case MARKDOWN_VIEW_RENDER:
        // no-op
        break;

    case MARKDOWN_VIEW_SOURCE:
    default:

        sourceStyles[currentTab] = {};
        renderStyle.display = 'none';
    }

    return (
        <div>
            <div style={renderStyle}>
                {contents}
            </div>
            <div>
                <CardSource contents={props[TAB_QUESTION]} style={sourceStyles[TAB_QUESTION]} />
                <CardSource contents={props[TAB_ANSWER]} style={sourceStyles[TAB_ANSWER]} />
                <CardSource contents={props[TAB_DESCRIPTION]} style={sourceStyles[TAB_DESCRIPTION]} />
            </div>
        </div>
    );

}

const CardContentsComponent = connect(

    // mapStateToProps
    (state) => {
        const currentCardTab = state[TAB];
        return {

            // current tab
            [TAB]: state[TAB],

            // markdown view of current tab
            [MARKDOWN_VIEW]: state[currentCardTab][MARKDOWN_VIEW],

            // card contents
            [TAB_QUESTION]: state[TAB_QUESTION][CARD_CONTENTS],
            [TAB_ANSWER]: state[TAB_ANSWER][CARD_CONTENTS],
            [TAB_DESCRIPTION]: state[TAB_DESCRIPTION][CARD_CONTENTS],

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


/* redux action dispatchers */
// NOTE: FSA compliant

const switchCardPerfControl = function(dispatch, nextCardPerfView) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                cardPerfReducer,
                // path
                [CARD_PERF_CONTROL_VIEW],
                // action
                {
                    type: nextCardPerfView
                }
            )
        );
    }
}

const skipCard = function(dispatch, nextSkipCardView) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                skipCardReducer,
                // path
                [SKIPCARD_VIEW],
                // action
                {
                    type: nextSkipCardView
                }
            )
        );
    }
}

const switchTab = function(dispatch, tabType) {
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                tabReducer,
                // path
                [TAB],
                // action
                {
                    type: tabType
                }
            )
        );
    }
}

const switchMarkdownView = function(dispatch, target, markdownView) {
    // target: Tab such as question, question, or description
    return function(event) {
        event.preventDefault();
        dispatch(
            reduceIn(
                // reducer
                markdownViewReducer,
                // path
                [target, MARKDOWN_VIEW],
                // action
                {
                    type: markdownView
                }
            )
        );
    }
}


/* redux reducers */

const cardPerfReducer = function(state = CARD_PERF_CONTROL__INITIAL, action) {

    switch(action.type) {
    case CARD_PERF_CONTROL__INITIAL:
    case CARD_PERF_CONTROL__DEFAULT_CHOICES:
        state = action.type;
        break;

    default:
        state = CARD_PERF_CONTROL__INITIAL;
    }

    return state;
}

const skipCardReducer = function(state = SKIPCARD_INITIAL, action) {

    switch(action.type) {
    case SKIPCARD_INITIAL:
    case SKIPCARD_CONFIRM:
        state = action.type;
        break;

    default:
        state = SKIPCARD_INITIAL;
    }

    return state;
}

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

    // TODO: check window.location.hash.substr(1)
    [TAB]: TAB_QUESTION,

    [TAB_QUESTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CARD_CONTENTS]: 'question'
    },

    [TAB_ANSWER]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CARD_CONTENTS]: 'answer'
    },

    [TAB_DESCRIPTION]: {
        [MARKDOWN_VIEW]: MARKDOWN_VIEW_RENDER,
        [CARD_CONTENTS]: 'description'
    },

    [SKIPCARD_VIEW]: SKIPCARD_INITIAL,

    [CARD_PERF_CONTROL_VIEW]: CARD_PERF_CONTROL__INITIAL
};

// TODO: remove
// const store = createStore(treeReducer, initialState);

/* start */

// ReactDOM.render(
//     <Provider store={store}>
//         <DeckReviewContainer />
//     </Provider>,
//     document.getElementById('deck-review-container'));

module.exports = function(__initialState = initialState) {

    const store = createStore(makeReducer(), __initialState);

    return (
        <Provider store={store}>
            <DeckReviewContainer />
        </Provider>
    );
}
