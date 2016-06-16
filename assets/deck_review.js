webpackJsonp([0],{

/***/ 0:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _defineProperty2 = __webpack_require__(1);
	
	var _defineProperty3 = _interopRequireDefault(_defineProperty2);
	
	var _TAB_QUESTION, _TAB_ANSWER, _TAB_DESCRIPTION, _initialState;
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	// TODO: this is fucking huge
	// require('babel-polyfill');
	
	var React = __webpack_require__(20);
	var ReactDOM = __webpack_require__(57);
	
	var _require = __webpack_require__(187);
	
	var Provider = _require.Provider;
	var connect = _require.connect;
	
	var _require2 = __webpack_require__(194);
	
	var createStore = _require2.createStore;
	
	var classnames = __webpack_require__(209);
	var TextareaAutosize = __webpack_require__(210).default;
	// const _ = require('lodash');
	// const {combineReducers} = require('redux');
	
	// const combineReducers = require('./combineReducers');
	// NOTE: above won't work; below is used to be in parity with redux upstream
	// import combineReducers from './combineReducers';
	
	var _require3 = __webpack_require__(212);
	
	var /*serializeProps, deserializeProps,*/genKey = _require3.genKey;
	
	var _require4 = __webpack_require__(213);
	
	var applyReducer = _require4.applyReducer;
	var treeReducer = _require4.reducer;
	
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
	
	var __CardReviewTabsComponent = function __CardReviewTabsComponent(props) {
	
	    var tabType = props[TAB];
	    var dispatch = props.dispatch;
	
	
	    return React.createElement(
	        'div',
	        { className: 'btn-group btn-group-block' },
	        React.createElement(
	            'a',
	            {
	                onClick: switchTab(dispatch, TAB_QUESTION),
	                href: '#',
	                className: classnames('btn', { 'btn-primary': tabType === TAB_QUESTION }) },
	            'Question'
	        ),
	        React.createElement(
	            'a',
	            {
	                onClick: switchTab(dispatch, TAB_DESCRIPTION),
	                href: '#',
	                className: classnames('btn', { 'btn-primary': tabType === TAB_DESCRIPTION }) },
	            'Description'
	        )
	    );
	};
	
	var CardReviewTabsComponent = connect(
	// mapStateToProps
	function (state) {
	    return (0, _defineProperty3.default)({}, TAB, state[TAB]);
	})(__CardReviewTabsComponent);
	
	var __RenderSourceComponent = function __RenderSourceComponent(props) {
	    var dispatch = props.dispatch;
	    var switchTab = props.switchTab;
	
	    var currentTab = props[MARKDOWN_VIEW];
	
	    return React.createElement(
	        'ul',
	        { className: 'tab' },
	        React.createElement(
	            'li',
	            { className: classnames('tab-item', { 'active': currentTab === MARKDOWN_VIEW_RENDER }) },
	            React.createElement(
	                'a',
	                {
	                    href: '#',
	                    onClick: switchTab(dispatch, MARKDOWN_VIEW_RENDER) },
	                'Render'
	            )
	        ),
	        React.createElement(
	            'li',
	            { className: classnames('tab-item', { 'active': currentTab === MARKDOWN_VIEW_SOURCE }) },
	            React.createElement(
	                'a',
	                {
	                    href: '#',
	                    onClick: switchTab(dispatch, MARKDOWN_VIEW_SOURCE) },
	                'Source'
	            )
	        )
	    );
	};
	
	var RenderSourceComponent = connect(
	
	// mapStateToProps
	function (state) {
	    var _ref2;
	
	    var currentCardTab = state[TAB];
	    return _ref2 = {}, (0, _defineProperty3.default)(_ref2, MARKDOWN_VIEW, state[currentCardTab][MARKDOWN_VIEW]), (0, _defineProperty3.default)(_ref2, 'switchTab', function switchTab(dispatch, markdownView) {
	        return switchMarkdownView(dispatch, currentCardTab, markdownView);
	    }), _ref2;
	})(
	
	// mapDispatchToProps
	// (dispatch) => {
	//     return {
	//         switchTab:
	//     };
	// }
	__RenderSourceComponent);
	
	var ReviewScoreCommitComponent = function ReviewScoreCommitComponent() /* props */{
	    return React.createElement(
	        'div',
	        { className: 'columns' },
	        React.createElement(
	            'div',
	            { className: 'column col-9' },
	            React.createElement(
	                'a',
	                { href: '#', className: 'btn btn-block' },
	                'Reveal Answer'
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'column col-3' },
	            React.createElement(
	                'a',
	                { href: '#', className: 'btn btn-block' },
	                'Skip Card'
	            )
	        )
	    );
	};
	
	var __CardContentsComponent = function __CardContentsComponent(props) {
	
	    var contents = props[CONTENTS];
	    var markdownView = props[MARKDOWN_VIEW];
	
	    switch (markdownView) {
	        case MARKDOWN_VIEW_RENDER:
	
	            return React.createElement(
	                'div',
	                { className: 'columns' },
	                React.createElement(
	                    'div',
	                    { className: 'column' },
	                    contents
	                )
	            );
	
	        case MARKDOWN_VIEW_SOURCE:
	        default:
	
	            return React.createElement(
	                'div',
	                { className: 'columns' },
	                React.createElement(
	                    'div',
	                    { className: 'column' },
	                    React.createElement(TextareaAutosize, {
	                        key: 'textarea',
	                        useCacheForDOMMeasurements: true,
	                        minRows: 6,
	                        maxRows: 10,
	                        className: 'form-input'
	                        // id="deck_source"
	                        // placeholder={placeholder}
	                        // onChange={this.onSourceChange}
	                        , value: contents,
	                        readOnly: true
	                    })
	                )
	            );
	    }
	};
	
	var CardContentsComponent = connect(
	
	// mapStateToProps
	function (state) {
	    var _ref3;
	
	    var currentCardTab = state[TAB];
	    return _ref3 = {}, (0, _defineProperty3.default)(_ref3, MARKDOWN_VIEW, state[currentCardTab][MARKDOWN_VIEW]), (0, _defineProperty3.default)(_ref3, CONTENTS, state[currentCardTab][CONTENTS]), _ref3;
	})(__CardContentsComponent);
	
	var DeckReviewContainer = function DeckReviewContainer() /* props */{
	    return React.createElement(
	        'div',
	        null,
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(CardReviewTabsComponent, null)
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(RenderSourceComponent, null)
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(CardContentsComponent, null)
	            )
	        ),
	        React.createElement(
	            'div',
	            { className: 'columns' },
	            React.createElement(
	                'div',
	                { className: 'column' },
	                React.createElement(ReviewScoreCommitComponent, null)
	            )
	        )
	    );
	};
	
	/* constants */
	
	/*
	enum Tab {
	    Question,
	    Answer,
	    Description
	}
	 */
	var TAB = genKey();
	var TAB_QUESTION = genKey();
	var TAB_ANSWER = genKey();
	var TAB_DESCRIPTION = genKey();
	
	/*
	enum MarkdownView {
	    Render,
	    Source
	}
	 */
	var MARKDOWN_VIEW = genKey();
	var MARKDOWN_VIEW_RENDER = genKey();
	var MARKDOWN_VIEW_SOURCE = genKey();
	
	var CONTENTS = genKey();
	
	/* redux action creators */
	// NOTE: FSA compliant
	
	var switchTab = function switchTab(dispatch, tabType) {
	    return function (event) {
	        event.preventDefault();
	        dispatch(applyReducer(
	        // path
	        [TAB],
	        // reducer
	        tabReducer,
	        // action
	        {
	            type: tabType
	        }));
	    };
	};
	
	var switchMarkdownView = function switchMarkdownView(dispatch, target, markdownView) {
	    // target: Tab
	    return function (event) {
	        event.preventDefault();
	        dispatch(applyReducer(
	        // path
	        [target, MARKDOWN_VIEW],
	        // reducer
	        markdownViewReducer,
	        // action
	        {
	            type: markdownView
	        }));
	    };
	};
	
	/* redux reducers */
	
	var tabReducer = function tabReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? TAB_QUESTION : arguments[0];
	    var action = arguments[1];
	
	
	    switch (action.type) {
	        case TAB_QUESTION:
	        case TAB_DESCRIPTION:
	            state = action.type;
	            break;
	
	        default:
	            state = TAB_QUESTION;
	    }
	
	    return state;
	};
	
	var markdownViewReducer = function markdownViewReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? MARKDOWN_VIEW_RENDER : arguments[0];
	    var action = arguments[1];
	
	
	    switch (action.type) {
	        case MARKDOWN_VIEW_RENDER:
	        case MARKDOWN_VIEW_SOURCE:
	            state = action.type;
	            break;
	
	        default:
	            state = MARKDOWN_VIEW_RENDER;
	    }
	
	    return state;
	};
	
	/* redux store */
	
	var initialState = (_initialState = {}, (0, _defineProperty3.default)(_initialState, TAB, TAB_QUESTION), (0, _defineProperty3.default)(_initialState, TAB_QUESTION, (_TAB_QUESTION = {}, (0, _defineProperty3.default)(_TAB_QUESTION, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER), (0, _defineProperty3.default)(_TAB_QUESTION, CONTENTS, 'question'), _TAB_QUESTION)), (0, _defineProperty3.default)(_initialState, TAB_ANSWER, (_TAB_ANSWER = {}, (0, _defineProperty3.default)(_TAB_ANSWER, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER), (0, _defineProperty3.default)(_TAB_ANSWER, CONTENTS, 'answer'), _TAB_ANSWER)), (0, _defineProperty3.default)(_initialState, TAB_DESCRIPTION, (_TAB_DESCRIPTION = {}, (0, _defineProperty3.default)(_TAB_DESCRIPTION, MARKDOWN_VIEW, MARKDOWN_VIEW_RENDER), (0, _defineProperty3.default)(_TAB_DESCRIPTION, CONTENTS, 'description'), _TAB_DESCRIPTION)), _initialState);
	
	var store = createStore(treeReducer, initialState);
	
	/* start */
	
	ReactDOM.render(React.createElement(
	    Provider,
	    { store: store },
	    React.createElement(DeckReviewContainer, null)
	), document.getElementById('deck-review-container'));

/***/ },

/***/ 1:
/***/ function(module, exports, __webpack_require__) {

	"use strict";
	
	exports.__esModule = true;
	
	var _defineProperty = __webpack_require__(2);
	
	var _defineProperty2 = _interopRequireDefault(_defineProperty);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	exports.default = function (obj, key, value) {
	  if (key in obj) {
	    (0, _defineProperty2.default)(obj, key, {
	      value: value,
	      enumerable: true,
	      configurable: true,
	      writable: true
	    });
	  } else {
	    obj[key] = value;
	  }
	
	  return obj;
	};

/***/ },

/***/ 2:
/***/ function(module, exports, __webpack_require__) {

	module.exports = { "default": __webpack_require__(3), __esModule: true };

/***/ },

/***/ 3:
/***/ function(module, exports, __webpack_require__) {

	__webpack_require__(4);
	var $Object = __webpack_require__(7).Object;
	module.exports = function defineProperty(it, key, desc){
	  return $Object.defineProperty(it, key, desc);
	};

/***/ },

/***/ 210:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	Object.defineProperty(exports, "__esModule", {
	  value: true
	});
	exports.default = undefined;
	
	var _typeof = typeof Symbol === "function" && typeof Symbol.iterator === "symbol" ? function (obj) { return typeof obj; } : function (obj) { return obj && typeof Symbol === "function" && obj.constructor === Symbol ? "symbol" : typeof obj; };
	
	var _extends = Object.assign || function (target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i]; for (var key in source) { if (Object.prototype.hasOwnProperty.call(source, key)) { target[key] = source[key]; } } } return target; };
	
	var _createClass = function () { function defineProperties(target, props) { for (var i = 0; i < props.length; i++) { var descriptor = props[i]; descriptor.enumerable = descriptor.enumerable || false; descriptor.configurable = true; if ("value" in descriptor) descriptor.writable = true; Object.defineProperty(target, descriptor.key, descriptor); } } return function (Constructor, protoProps, staticProps) { if (protoProps) defineProperties(Constructor.prototype, protoProps); if (staticProps) defineProperties(Constructor, staticProps); return Constructor; }; }();
	
	var _class, _temp; /**
	                    * <TextareaAutosize />
	                    */
	
	var _react = __webpack_require__(20);
	
	var _react2 = _interopRequireDefault(_react);
	
	var _calculateNodeHeight = __webpack_require__(211);
	
	var _calculateNodeHeight2 = _interopRequireDefault(_calculateNodeHeight);
	
	function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }
	
	function _objectWithoutProperties(obj, keys) { var target = {}; for (var i in obj) { if (keys.indexOf(i) >= 0) continue; if (!Object.prototype.hasOwnProperty.call(obj, i)) continue; target[i] = obj[i]; } return target; }
	
	function _classCallCheck(instance, Constructor) { if (!(instance instanceof Constructor)) { throw new TypeError("Cannot call a class as a function"); } }
	
	function _possibleConstructorReturn(self, call) { if (!self) { throw new ReferenceError("this hasn't been initialised - super() hasn't been called"); } return call && (typeof call === "object" || typeof call === "function") ? call : self; }
	
	function _inherits(subClass, superClass) { if (typeof superClass !== "function" && superClass !== null) { throw new TypeError("Super expression must either be null or a function, not " + typeof superClass); } subClass.prototype = Object.create(superClass && superClass.prototype, { constructor: { value: subClass, enumerable: false, writable: true, configurable: true } }); if (superClass) Object.setPrototypeOf ? Object.setPrototypeOf(subClass, superClass) : subClass.__proto__ = superClass; }
	
	var emptyFunction = function emptyFunction() {};
	
	var TextareaAutosize = (_temp = _class = function (_React$Component) {
	  _inherits(TextareaAutosize, _React$Component);
	
	  function TextareaAutosize(props) {
	    _classCallCheck(this, TextareaAutosize);
	
	    var _this = _possibleConstructorReturn(this, Object.getPrototypeOf(TextareaAutosize).call(this, props));
	
	    _this.state = {
	      height: null,
	      minHeight: -Infinity,
	      maxHeight: Infinity
	    };
	    _this._onNextFrameActionId = null;
	    _this._rootDOMNode = null;
	    _this._onChange = _this._onChange.bind(_this);
	    _this._resizeComponent = _this._resizeComponent.bind(_this);
	    _this._onRootDOMNode = _this._onRootDOMNode.bind(_this);
	    return _this;
	  }
	
	  _createClass(TextareaAutosize, [{
	    key: 'render',
	    value: function render() {
	      var _props = this.props;
	      var valueLink = _props.valueLink;
	
	      var props = _objectWithoutProperties(_props, ['valueLink']);
	
	      props = _extends({}, props);
	      if ((typeof valueLink === 'undefined' ? 'undefined' : _typeof(valueLink)) === 'object') {
	        props.value = this.props.valueLink.value;
	      }
	      props.style = _extends({}, props.style, {
	        height: this.state.height || 0
	      });
	      var maxHeight = Math.max(props.style.maxHeight ? props.style.maxHeight : Infinity, this.state.maxHeight);
	      if (maxHeight < this.state.height) {
	        props.style.overflow = 'hidden';
	      }
	      return _react2.default.createElement('textarea', _extends({}, props, {
	        onChange: this._onChange,
	        ref: this._onRootDOMNode
	      }));
	    }
	  }, {
	    key: 'componentDidMount',
	    value: function componentDidMount() {
	      this._resizeComponent();
	      window.addEventListener('resize', this._resizeComponent);
	    }
	  }, {
	    key: 'componentWillReceiveProps',
	    value: function componentWillReceiveProps() {
	      // Re-render with the new content then recalculate the height as required.
	      this._clearNextFrame();
	      this._onNextFrameActionId = onNextFrame(this._resizeComponent);
	    }
	  }, {
	    key: 'componentDidUpdate',
	    value: function componentDidUpdate(prevProps, prevState) {
	      // Invoke callback when old height does not equal to new one.
	      if (this.state.height !== prevState.height) {
	        this.props.onHeightChange(this.state.height);
	      }
	    }
	  }, {
	    key: 'componentWillUnmount',
	    value: function componentWillUnmount() {
	      // Remove any scheduled events to prevent manipulating the node after it's
	      // been unmounted.
	      this._clearNextFrame();
	      window.removeEventListener('resize', this._resizeComponent);
	    }
	  }, {
	    key: '_clearNextFrame',
	    value: function _clearNextFrame() {
	      if (this._onNextFrameActionId) {
	        clearNextFrameAction(this._onNextFrameActionId);
	      }
	    }
	  }, {
	    key: '_onRootDOMNode',
	    value: function _onRootDOMNode(node) {
	      this._rootDOMNode = node;
	    }
	  }, {
	    key: '_onChange',
	    value: function _onChange(e) {
	      this._resizeComponent();
	      var _props2 = this.props;
	      var valueLink = _props2.valueLink;
	      var onChange = _props2.onChange;
	
	      if (valueLink) {
	        valueLink.requestChange(e.target.value);
	      } else {
	        onChange(e);
	      }
	    }
	  }, {
	    key: '_resizeComponent',
	    value: function _resizeComponent() {
	      var useCacheForDOMMeasurements = this.props.useCacheForDOMMeasurements;
	
	      this.setState((0, _calculateNodeHeight2.default)(this._rootDOMNode, useCacheForDOMMeasurements, this.props.rows || this.props.minRows, this.props.maxRows));
	    }
	
	    /**
	     * Read the current value of <textarea /> from DOM.
	     */
	
	  }, {
	    key: 'focus',
	
	
	    /**
	     * Put focus on a <textarea /> DOM element.
	     */
	    value: function focus() {
	      this._rootDOMNode.focus();
	    }
	
	    /**
	     * Shifts focus away from a <textarea /> DOM element.
	     */
	
	  }, {
	    key: 'blur',
	    value: function blur() {
	      this._rootDOMNode.blur();
	    }
	  }, {
	    key: 'value',
	    get: function get() {
	      return this._rootDOMNode.value;
	    }
	
	    /**
	     * Set the current value of <textarea /> DOM node.
	     */
	    ,
	    set: function set(val) {
	      this._rootDOMNode.value = val;
	    }
	
	    /**
	     * Read the current selectionStart of <textarea /> from DOM.
	     */
	
	  }, {
	    key: 'selectionStart',
	    get: function get() {
	      return this._rootDOMNode.selectionStart;
	    }
	
	    /**
	     * Set the current selectionStart of <textarea /> DOM node.
	     */
	    ,
	    set: function set(selectionStart) {
	      this._rootDOMNode.selectionStart = selectionStart;
	    }
	
	    /**
	     * Read the current selectionEnd of <textarea /> from DOM.
	     */
	
	  }, {
	    key: 'selectionEnd',
	    get: function get() {
	      return this._rootDOMNode.selectionEnd;
	    }
	
	    /**
	     * Set the current selectionEnd of <textarea /> DOM node.
	     */
	    ,
	    set: function set(selectionEnd) {
	      this._rootDOMNode.selectionEnd = selectionEnd;
	    }
	  }]);
	
	  return TextareaAutosize;
	}(_react2.default.Component), _class.propTypes = {
	  /**
	   * Current textarea value.
	   */
	  value: _react2.default.PropTypes.string,
	
	  /**
	   * Callback on value change.
	   */
	  onChange: _react2.default.PropTypes.func,
	
	  /**
	   * Callback on height changes.
	   */
	  onHeightChange: _react2.default.PropTypes.func,
	
	  /**
	   * Try to cache DOM measurements performed by component so that we don't
	   * touch DOM when it's not needed.
	   *
	   * This optimization doesn't work if we dynamically style <textarea />
	   * component.
	   */
	  useCacheForDOMMeasurements: _react2.default.PropTypes.bool,
	
	  /**
	   * Minimal numbder of rows to show.
	   */
	  rows: _react2.default.PropTypes.number,
	
	  /**
	   * Alias for `rows`.
	   */
	  minRows: _react2.default.PropTypes.number,
	
	  /**
	   * Maximum number of rows to show.
	   */
	  maxRows: _react2.default.PropTypes.number
	}, _class.defaultProps = {
	  onChange: emptyFunction,
	  onHeightChange: emptyFunction,
	  useCacheForDOMMeasurements: false
	}, _temp);
	exports.default = TextareaAutosize;
	
	
	function onNextFrame(cb) {
	  if (window.requestAnimationFrame) {
	    return window.requestAnimationFrame(cb);
	  }
	  return window.setTimeout(cb, 1);
	}
	
	function clearNextFrameAction(nextFrameId) {
	  if (window.cancelAnimationFrame) {
	    window.cancelAnimationFrame(nextFrameId);
	  } else {
	    window.clearTimeout(nextFrameId);
	  }
	}


/***/ },

/***/ 211:
/***/ function(module, exports) {

	'use strict';
	
	Object.defineProperty(exports, "__esModule", {
	  value: true
	});
	exports.default = calculateNodeHeight;
	/**
	 * calculateNodeHeight(uiTextNode, useCache = false)
	 */
	
	var HIDDEN_TEXTAREA_STYLE = '\n  min-height:0 !important;\n  max-height:none !important;\n  height:0 !important;\n  visibility:hidden !important;\n  overflow:hidden !important;\n  position:absolute !important;\n  z-index:-1000 !important;\n  top:0 !important;\n  right:0 !important\n';
	
	var SIZING_STYLE = ['letter-spacing', 'line-height', 'padding-top', 'padding-bottom', 'font-family', 'font-weight', 'font-size', 'text-rendering', 'text-transform', 'width', 'text-indent', 'padding-left', 'padding-right', 'border-width', 'box-sizing'];
	
	var computedStyleCache = {};
	var hiddenTextarea = void 0;
	
	function calculateNodeHeight(uiTextNode) {
	  var useCache = arguments.length <= 1 || arguments[1] === undefined ? false : arguments[1];
	  var minRows = arguments.length <= 2 || arguments[2] === undefined ? null : arguments[2];
	  var maxRows = arguments.length <= 3 || arguments[3] === undefined ? null : arguments[3];
	
	  if (!hiddenTextarea) {
	    hiddenTextarea = document.createElement('textarea');
	    document.body.appendChild(hiddenTextarea);
	  }
	
	  // Copy all CSS properties that have an impact on the height of the content in
	  // the textbox
	
	  var _calculateNodeStyling = calculateNodeStyling(uiTextNode, useCache);
	
	  var paddingSize = _calculateNodeStyling.paddingSize;
	  var borderSize = _calculateNodeStyling.borderSize;
	  var boxSizing = _calculateNodeStyling.boxSizing;
	  var sizingStyle = _calculateNodeStyling.sizingStyle;
	
	  // Need to have the overflow attribute to hide the scrollbar otherwise
	  // text-lines will not calculated properly as the shadow will technically be
	  // narrower for content
	
	  hiddenTextarea.setAttribute('style', sizingStyle + ';' + HIDDEN_TEXTAREA_STYLE);
	  hiddenTextarea.value = uiTextNode.value || uiTextNode.placeholder || '';
	
	  var minHeight = -Infinity;
	  var maxHeight = Infinity;
	  var height = hiddenTextarea.scrollHeight;
	
	  if (boxSizing === 'border-box') {
	    // border-box: add border, since height = content + padding + border
	    height = height + borderSize;
	  } else if (boxSizing === 'content-box') {
	    // remove padding, since height = content
	    height = height - paddingSize;
	  }
	
	  if (minRows !== null || maxRows !== null) {
	    // measure height of a textarea with a single row
	    hiddenTextarea.value = '';
	    var singleRowHeight = hiddenTextarea.scrollHeight - paddingSize;
	    if (minRows !== null) {
	      minHeight = singleRowHeight * minRows;
	      if (boxSizing === 'border-box') {
	        minHeight = minHeight + paddingSize + borderSize;
	      }
	      height = Math.max(minHeight, height);
	    }
	    if (maxRows !== null) {
	      maxHeight = singleRowHeight * maxRows;
	      if (boxSizing === 'border-box') {
	        maxHeight = maxHeight + paddingSize + borderSize;
	      }
	      height = Math.min(maxHeight, height);
	    }
	  }
	  return { height: height, minHeight: minHeight, maxHeight: maxHeight };
	}
	
	function calculateNodeStyling(node) {
	  var useCache = arguments.length <= 1 || arguments[1] === undefined ? false : arguments[1];
	
	  var nodeRef = node.getAttribute('id') || node.getAttribute('data-reactid') || node.getAttribute('name');
	
	  if (useCache && computedStyleCache[nodeRef]) {
	    return computedStyleCache[nodeRef];
	  }
	
	  var style = window.getComputedStyle(node);
	
	  var boxSizing = style.getPropertyValue('box-sizing') || style.getPropertyValue('-moz-box-sizing') || style.getPropertyValue('-webkit-box-sizing');
	
	  var paddingSize = parseFloat(style.getPropertyValue('padding-bottom')) + parseFloat(style.getPropertyValue('padding-top'));
	
	  var borderSize = parseFloat(style.getPropertyValue('border-bottom-width')) + parseFloat(style.getPropertyValue('border-top-width'));
	
	  var sizingStyle = SIZING_STYLE.map(function (name) {
	    return name + ':' + style.getPropertyValue(name);
	  }).join(';');
	
	  var nodeInfo = {
	    sizingStyle: sizingStyle,
	    paddingSize: paddingSize,
	    borderSize: borderSize,
	    boxSizing: boxSizing
	  };
	
	  if (useCache && nodeRef) {
	    computedStyleCache[nodeRef] = nodeInfo;
	  }
	
	  return nodeInfo;
	}


/***/ },

/***/ 212:
/***/ function(module, exports) {

	'use strict';
	
	// alternative to Symbol()
	var _objKey = 1;
	var genKey = function genKey() {
	    return '__genKey_' + String(_objKey++);
	};
	
	module.exports = {
	    genKey: genKey
	};

/***/ },

/***/ 213:
/***/ function(module, exports, __webpack_require__) {

	'use strict';
	
	var _ = __webpack_require__(214);
	
	var APPLY_REDUCER = 'APPLY_REDUCER';
	var EMPTY_OBJ = {};
	
	var treeReducer = function treeReducer() {
	    var state = arguments.length <= 0 || arguments[0] === undefined ? EMPTY_OBJ : arguments[0];
	    var action = arguments[1];
	
	
	    var patch = EMPTY_OBJ;
	
	    switch (action.type) {
	
	        case APPLY_REDUCER:
	            var _action$payload = action.payload;
	            var path = _action$payload.path;
	            var reducer = _action$payload.reducer;
	
	            // TODO: path is array
	            // TODO: check reducer is function
	
	            var specificState = _.get(state, path);
	
	            // TODO: _.has(state, path);
	
	            var result = reducer(specificState, action.payload.action);
	            patch = _.set({}, path, result);
	            break;
	
	        default:
	            return state;
	    }
	
	    // console.log('patched');
	    return _.merge({}, state, patch);
	    // return {...state, ...patch};
	};
	
	var applyReducer = function applyReducer(path, reducer, action) {
	    return {
	        type: APPLY_REDUCER,
	        payload: {
	            path: path, // array
	            reducer: reducer, // redux compatible reducer
	            action: action // action to be applied to given reducer
	        }
	    };
	};
	
	module.exports = {
	    reducer: treeReducer,
	    applyReducer: applyReducer
	};

/***/ }

});
//# sourceMappingURL=deck_review.js.map