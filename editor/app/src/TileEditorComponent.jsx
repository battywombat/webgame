import React, { Component } from 'react';

import styles from './TileEditorComponent.css';

export default class TileEditorComponent extends Component {

    componentDidUpdate() {
        let img = new Image();
        img.src = this.props.src;
        img.onload = () => {
            const canvas = this.refs.tileEditorCanvas;
            const ctx = canvas.getContext('2d');
            ctx.drawImage(img, 0, 0);
        }
    }

    render() {
        return <div className={styles.TileEditorComponent}>
            <canvas ref="tileEditorCanvas"></canvas>
        </div>
    }
}