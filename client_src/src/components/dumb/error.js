const React = require('react');
const isFunction = require('lodash/isFunction');

const ErrorComponent = function(props) {

    if(!props.error || String(props.error).trim().length <= 0) {
        return null;
    }

    if(isFunction(props.onConfirm)) {

        return (
            <div className='columns'>
                <div className='column'>
                    <div className='notification is-danger'>
                        <button className='delete' onClick={props.onConfirm}></button>
                        {props.error}
                    </div>
                </div>
            </div>
        );

    }

    return (
        <div className='columns'>
            <div className='column'>
                <div className='notification is-danger'>
                    {props.error}
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    ErrorComponent.propTypes = {
        error: React.PropTypes.string,
        onConfirm: React.PropTypes.func
    };
}

module.exports = ErrorComponent;
