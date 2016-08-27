const React = require('react');

const ErrorComponent = function(props) {

    if(!props.error || String(props.error).trim().length <= 0) {
        return null;
    }

    return (
        <div className='columns'>
            <div className='column'>
                <div className='message is-danger'>
                    <div className='message-body'>
                        {props.error}
                    </div>
                </div>
            </div>
        </div>
    );

};

if(process.env.NODE_ENV !== 'production') {
    ErrorComponent.propTypes = {
        error: React.PropTypes.string
    };
}

module.exports = ErrorComponent;
