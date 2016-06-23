/*
Same as render_source.js but tabs are swapped.

Design decision:
- swapping is not configurable to optimize generated code.

*/

const React = require('react');
const classnames = require('classnames');

const {

    MARKDOWN_VIEW,
    MARKDOWN_VIEW_RENDER,
    MARKDOWN_VIEW_SOURCE,

} = require('global/constants');

const BOLD = {
    fontWeight: 'bold'
};
const EMPTY_OBJ = {};

const boldTest = (conditionTest) => {
    return conditionTest ? BOLD : EMPTY_OBJ;
};

// API:
// type props[MARKDOWN_VIEW] = MARKDOWN_VIEW_RENDER | MARKDOWN_VIEW_SOURCE
// type props.switchTab = (dispatch: dispatch, next_view: MARKDOWN_VIEW) => (event) => void 0;
//
const RenderSourceComponent = function(props) {

    // NOTE: switchTab(dispatch: dispatch, next_view: MARKDOWN_VIEW) => (event) => void 0;

    const {dispatch, switchTab} = props;
    const currentTab = props[MARKDOWN_VIEW];

    return (
        <ul className='tab'>
            <li className = {classnames('tab-item', {'active': currentTab === MARKDOWN_VIEW_SOURCE})}>
                <a
                    href='#source'
                    style={boldTest(currentTab === MARKDOWN_VIEW_SOURCE)}
                    onClick={switchTab(dispatch, MARKDOWN_VIEW_SOURCE)}>
                    {'Source'}
                </a>
            </li>
            <li className = {classnames('tab-item', {'active': currentTab === MARKDOWN_VIEW_RENDER})}>
                <a
                    href='#render'
                    style={boldTest(currentTab === MARKDOWN_VIEW_RENDER)}
                    onClick={switchTab(dispatch, MARKDOWN_VIEW_RENDER)}>
                    {'Render'}
                </a>
            </li>
        </ul>
    );
};

if(process.env.NODE_ENV !== 'production') {
    RenderSourceComponent.propTypes = {
        dispatch: React.PropTypes.func,
        switchTab: React.PropTypes.func,
        [MARKDOWN_VIEW]: React.PropTypes.oneOf([MARKDOWN_VIEW_RENDER, MARKDOWN_VIEW_SOURCE])
    };
}

module.exports = RenderSourceComponent;
