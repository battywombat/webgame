import React, {Component} from 'react';

const fs = require('fs');

export default class App extends Component {
    render() {
        return (<div>
            <h1>Hello, Node!</h1>
            <button onClick={this.handleClick}>Click Me Please!</button>
            </div>
        )
    }

    handleClick() {
        fs.exists('fakePath', (exists) => {
            console.log("Console existence is: ");
            console.log(exists);
        })
    }
}
