import React from 'react';
import styles from './TileFile.css';

const electron = require('electron');

class AddTileFileButton extends React.Component {

    render() {
        return <button onClick={this.buttonClicked.bind(this)}>Add Tile File</button>
    }

    buttonClicked() {
        electron.remote.dialog.showOpenDialog({
            filters: [{ name: 'Image', extensions: ['png', 'jpeg', 'jpg']}]
        }, (fps) => {
            if (typeof fps === 'undefined') {
                return;
            }
            this.props.callback(fps[0]);
        });
    }
}

class TileFile extends React.Component {
    render() {
        return <img className={styles.TileFile} src={this.props.src} />
    }
}

class TileFileList extends React.Component {

    render() {
        let files = this.props.files.map((file) => <TileFile key={file.key} src={file.src} />);
        return <div>{files}</div>
    } 
}

export default class TileFilesComponent extends React.Component {

    render() {
        return <div>
            <TileFileList files={this.props.files} />
            <AddTileFileButton callback={this.props.callbacks.onTileFileAdded}/>
        </div>
    }
}