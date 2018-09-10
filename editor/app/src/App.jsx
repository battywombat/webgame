import React, {Component} from 'react';
import update from 'immutability-helper';

import TileFilesComponent from './TileFilesComponent';
import TileEditorComponent from './TileEditorComponent';

import './global.css'

export default class App extends Component {
    constructor(props, context) {
        super(props, context);
        this.state = {
            tileFiles: [],
            tileEditorSelected: -1
        };
    }

    render() {
        const tileFileCallbacks = {
            onTileFileAdded: this.onTileFileAdded.bind(this),
            onDisplayTileFile: this.onDisplayTileFile.bind(this)
        };
        const tileEditorCallbacks = {
            onWidthChanged: this.onTileWidthChanged.bind(this),
            onHeightChanged: this.onTileHeightChanged.bind(this)
        };
        const editingTileFile = this.state.tileFiles.find((e) => e.key === this.state.tileEditorSelected) || {};
        return (<div>
            <TileFilesComponent files={this.state.tileFiles} callbacks={tileFileCallbacks}/>
            <TileEditorComponent height={editingTileFile.height}
                                 width={editingTileFile.width}
                                 src={editingTileFile.src}
                                 callbacks={tileEditorCallbacks} />
            </div>
        )
    }

    onTileFileAdded(newFile) {
        let newFileKey = this.state.tileFiles.length;
        this.setState(update(this.state, {
            tileFiles: {$push: [{key: newFileKey, src: newFile, width: 16, height: 16}]}
        }));
    }

    onDisplayTileFile(key) {

        this.setState(update(this.state, {
            tileEditorSelected: {$set: key}
        }));
    }

    onTileWidthChanged(newWidth) {

    }

    onTileHeightChanged(newHeight) {

    }
}
