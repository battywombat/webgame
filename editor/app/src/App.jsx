import React, {Component} from 'react';
import update from 'immutability-helper';

import TileFilesComponent from './TileFilesComponent';
import TileEditorComponent from './TileEditorComponent';

import './global.css';
import styles from './App.css';

export default class App extends Component {
    constructor(props, context) {
        super(props, context);
        this.state = {
            tileWidth: 16,
            tileHeight: 16,
            tiles: [],
            tileFiles: [],
            tileEditorSelected: -1,
            tileEditorScrollScale: 0.5
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
        return (<div className={styles.App}>
            <TileFilesComponent files={this.state.tileFiles} callbacks={tileFileCallbacks}/>
            <TileEditorComponent height={editingTileFile.height}
                                 width={editingTileFile.width}
                                 src={editingTileFile.src}
                                 callbacks={tileEditorCallbacks}
                                 scrollScale={this.state.tileEditorScrollScale} />
            </div>
        )
    }

    onTileFileAdded(newFile) {
        let newFileKey = this.state.tileFiles.length;
        let img = new Image();
        img.src = newFile;
        img.onload = () => {
            let nrows = img.height/this.state.tileHeight;
            let ncols = img.width/this.state.tileWidth;
            let ntiles = ncols*nrows;
            let newTiles = [];
            for (let i = 0; i < ntiles; i++) {
                newTiles.push({
                    src: newFile,
                    xOffset: (i % ncols)*this.state.tileWidth,
                    yOffset: Math.floor(i/nrows)*this.state.tileHeight,
                });
            }
            this.setState(update(this.state, {
                tileFiles: {$push: [{key: newFileKey, src: newFile}]},
                tiles: {$push: newTiles}
            }));
        };
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
