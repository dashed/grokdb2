/*global MathJax: true */
/*eslint new-cap: [2, {"capIsNewExceptions": ["MathJax.Hub.Queue", "Remove"]}]*/

const React = require('react');
const each = require('lodash/each');
const ReactDOM = require('react-dom');

const assign = require('lodash/assign');

// TODO: http://meta.math.stackexchange.com/questions/16946/using-block-displayed-equations-in-question-titles


const Content = function(props) {

    const content = props.content.trim();

    if(content.length <= 0) {
        return (
            <div className='message is-info'>
                <div className='message-body'>
                    {props.notice}
                </div>
            </div>
        );
    }

    return (
        <h1 className='title'>{content}</h1>
    );
};

if(process.env.NODE_ENV !== 'production') {
    Content.propTypes = {
        content: React.PropTypes.string.isRequired,
        notice: React.PropTypes.string.isRequired,
    };
}


const CardTitle = React.createClass({

    propTypes: {
        mathjaxify: React.PropTypes.bool.isRequired,
        content: React.PropTypes.string.isRequired,
        notice: React.PropTypes.string.isRequired,
        isEditing: React.PropTypes.bool.isRequired,
        assignField: React.PropTypes.object.isRequired
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
                    <Content
                        content={this.props.content || ''}
                        notice={this.props.notice}
                    />
                </div>
                <div style={sourceStyle}>
                    <p className='control'>
                        <input
                            id='input-card-title'
                            className='input'
                            type='text'
                            placeholder='Card Title'
                            autoFocus
                            disabled={!this.props.isEditing}
                            {...assign({}, this.props.assignField)}
                        />
                    </p>
                </div>
            </div>
        );
    }
});

module.exports = CardTitle;
