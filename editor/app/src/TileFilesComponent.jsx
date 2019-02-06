import React from 'react';
import styles from './TileFilesComponent.css';

import CanvasRendererComponent from './CanvasRendererComponent';

const electron = require('electron');

class AddTileFileButton extends React.Component {

    render() {
        return <div className={styles.TileFileAddButton}>
                    <button className={styles.TileFileAddButton}
                            onClick={this.buttonClicked.bind(this)}>Add Tile File</button>
        </div>
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
        const srcImage = new Image();
        if (this.props.src !== undefined) {
            srcImage.src = this.props.src;
        }
        return <CanvasRendererComponent srcImage={srcImage}
                                        onDoubleClick={this.doubleClicked.bind(this)} />
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