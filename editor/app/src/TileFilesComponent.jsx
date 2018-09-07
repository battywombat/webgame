import React from 'react';
import styles from './TileFilesComponent.css';

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
        return <img onDoubleClick={this.doubleClicked.bind(this)} className={styles.TileFile} src={this.props.src} />
    }

    doubleClicked() {
        this.props.callback(this.props.id);
    }
}

class TileFileList extends React.Component {

    render() {
        let files = this.props.files.map((file) => <TileFile id={file.key} callback={this.props.callback} key={file.key} src={file.src} />);
        return <div>{files}</div>
    }
}

export default class TileFilesComponent extends React.Component {

    render() {
        return <div className={styles.TileFilesComponent}>
            <TileFileList callback={this.props.callbacks.onDisplayTileFile} files={this.props.files} />
            <AddTileFileButton callback={this.props.callbacks.onTileFileAdded}/>
        </div>
    }
}