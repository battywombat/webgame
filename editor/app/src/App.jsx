import React, {Component} from 'react';
import update from 'immutability-helper';

import TileFilesComponent from './TileFilesComponent';

export default class App extends Component {
    constructor(props, context) {
        super(props, context);
        this.state = {
            tileFiles: []
        };
    }

    render() {
        const tileFileCallbacks = {
            onTileFileAdded: this.onTileFileAdded.bind(this)
        };
        return (<div>
            <TileFilesComponent files={this.state.tileFiles} callbacks={tileFileCallbacks}/>
            </div>
        )
    }

    handleClick() {
        fs.exists('fakePath', (exists) => {
            console.log("Console existence is: ");
            console.log(exists);
        });
    }

    onTileFileAdded(newFile) {
        let newFileKey = this.state.tileFiles.length;
        this.setState(update(this.state, {
            tileFiles: {$push: [{ key: newFileKey, src: newFile }]}
        }));
    }
}
