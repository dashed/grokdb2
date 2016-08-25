/*global MathJax: true */
/*eslint new-cap: [2, {"capIsNewExceptions": ["MathJax.Hub.Queue", "Remove"]}]*/

const React = require('react');
const ReactDOM = require('react-dom');
const each = require('lodash/each');

const MathJaxRenderInline = React.createClass({

    propTypes: {
        contents: React.PropTypes.string.isRequired
    },

    componentDidUpdate() {

        if(!MathJax) {

            if(process.env.NODE_ENV !== 'production') {
                const invariant = require('invariant');
                invariant(MathJax, 'no MathJax loaded');
            }

            return;
        }

        if(!this.refs.markdown_render) {
            return;
        }

        MathJax.Hub.Queue(['Typeset', MathJax.Hub, ReactDOM.findDOMNode(this.refs.markdown_render)]);
    },

    componentDidMount() {

        if(!MathJax) {

            if(process.env.NODE_ENV !== 'production') {
                const invariant = require('invariant');
                invariant(MathJax, 'no MathJax loaded');
            }

            return;
        }

        if(!this.refs.markdown_render) {
            return;
        }

        MathJax.Hub.Queue(['Typeset', MathJax.Hub, ReactDOM.findDOMNode(this.refs.markdown_render)]);
    },

    componentWillUnmount() {

        if(!MathJax) {

            if(process.env.NODE_ENV !== 'production') {
                const invariant = require('invariant');
                invariant(MathJax, 'no MathJax loaded');
            }

            return;
        }

        if(!this.refs.markdown_render) {
            return;
        }

        each(MathJax.Hub.getAllJax(ReactDOM.findDOMNode(this.refs.markdown_render)), function(jax) {
            jax.Remove();
        });
    },

    render() {

        const content = String(this.props.contents).trim();

        if(content.length > 0) {
            return (
                <span
                    className='mathjax_inline content'
                    ref='markdown_render'
                >
                {content}
                </span>
            );
        }

        return(
            <span
                className='mathjax_inline content'
                ref='markdown_render'
            >
            </span>
        );

    }
});

module.exports = MathJaxRenderInline;
