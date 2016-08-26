/*global MathJax: true */
/*eslint new-cap: [2, {"capIsNewExceptions": ["MathJax.Hub.Queue", "Remove"]}]*/

const React = require('react');
const ReactDOM = require('react-dom');
const each = require('lodash/each');

const linkAttributes = require('markdown-it-link-attributes')

const markdownParser = require('markdown-it')({
    // TODO: disable this for saas app
    html: true,
    linkify: true

    // TODO: https://github.com/markdown-it/markdown-it#syntax-highlighting
})

// custom plugin to mark mathjax markup to not be escaped by markdown-it
// and related plugins
// .use(require('helpers/mathjaxinline'))

// NOTE: alternative to markdown-it-for-inline
.use(linkAttributes, {
    target: '_blank',
    // NOTE: See: https://dev.to/ben/the-targetblank-vulnerability-by-example
    rel: 'nofollow me noopener noreferrer'
});

// load with plugins (officially supported by markdown-it org)
// ...


const MarkdownRenderInline = React.createClass({

    propTypes: {
        contents: React.PropTypes.string.isRequired
    },

    // TODO: remove
    // getDefaultProps() {
    //     return {
    //         noContentMessage: 'No content was rendered. Click on "Source" tab and enter some text.',
    //         showNoContentMessage: true
    //     };
    // },

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

        const content = markdownParser.render(this.props.contents || '').trim();

        if(content.length > 0) {
            return (
                <span
                    className='content'
                    ref='markdown_render'
                    dangerouslySetInnerHTML={{__html: content}}
                />
            );
        }

        return(
            <span
                className='content'
                ref='markdown_render'>
            </span>
        );

    }
});

module.exports = MarkdownRenderInline;
