import React, { Component } from 'react';

import styles from './TileEditorComponent.css';
import CanvasRendererComponent from './CanvasRendererComponent';

export default class TileEditorComponent extends Component {

    render() {
        const srcImage = new Image();
        if (this.props.src !== undefined) {
            srcImage.src = this.props.src;
        }
        return <div className={styles.TileEditorComponent}>
            <CanvasRendererComponent srcImage={srcImage} />
        </div>
    }

}