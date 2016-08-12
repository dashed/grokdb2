/*global MathJax: true */
/*eslint new-cap: [2, {"capIsNewExceptions": ["MathJax.Hub.Queue", "Remove"]}]*/

const React = require('react');
const each = require('lodash/each');
const ReactDOM = require('react-dom');

// TODO: http://meta.math.stackexchange.com/questions/16946/using-block-displayed-equations-in-question-titles

const MathJaxLine = React.createClass({

    propTypes: {
        mathjaxify: React.PropTypes.bool.isRequired,
        content: React.PropTypes.string.isRequired,
        children: React.PropTypes.node.isRequired
    },

    componentDidUpdate() {

        if(!MathJax) {

            if(process.env.NODE_ENV !== 'production') {
                const invariant = require('invariant');
                invariant(MathJax, 'no MathJax loaded');
            }

            return;
        }

        if(!this.refs.mathjax_line) {
            return;
        }

        MathJax.Hub.Queue(['Typeset', MathJax.Hub, ReactDOM.findDOMNode(this.refs.mathjax_line)]);
    },

    componentDidMount() {

        if(!MathJax) {

            if(process.env.NODE_ENV !== 'production') {
                const invariant = require('invariant');
                invariant(MathJax, 'no MathJax loaded');
            }

            return;
        }

        if(!this.refs.mathjax_line) {
            return;
        }

        MathJax.Hub.Queue(['Typeset', MathJax.Hub, ReactDOM.findDOMNode(this.refs.mathjax_line)]);
    },

    componentWillUnmount() {

        if(!MathJax) {

            if(process.env.NODE_ENV !== 'production') {
                const invariant = require('invariant');
                invariant(MathJax, 'no MathJax loaded');
            }

            return;
        }

        if(!this.refs.mathjax_line) {
            return;
        }

        each(MathJax.Hub.getAllJax(ReactDOM.findDOMNode(this.refs.mathjax_line)), function(jax) {
            jax.Remove();
        });
    },

    render() {

        let sourceStyle = {};
        let renderStyle = {};

        if (this.props.mathjaxify) {
            sourceStyle.display = 'none';
        } else {
            renderStyle.display = 'none';
        }

        return (
            <div>
                <div style={renderStyle} ref='mathjax_line'>
                    {this.props.content}
                </div>
                <div style={sourceStyle}>
                    {this.props.children}
                </div>
            </div>
        );
    }
});

module.exports = MathJaxLine;
