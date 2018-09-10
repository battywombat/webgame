import React, { Component } from 'react';

import styles from './TileEditorComponent.css';

export default class TileEditorComponent extends Component {

    componentWillMount() {
        this.mouseMoveHandler = this.mouseMove.bind(this);
        this.img = new Image();
        this.sx = 0;
        this.sy = 0;
        this.sWidth = 0;
        this.sHeight = 0;
        this.dWidth = 0;
        this.dHeight = 0;
        this.dx = 0;
        this.dy = 0;
        this.lastX = 0;
        this.lastY = 0;
    }

    componentDidUpdate() {
        if (this.props.src !== undefined) {
            this.img.src = this.props.src;
            this.img.onload = () => {
                const canvas = this.refs.tileEditorCanvas;
                this.sWidth = this.dWidth =  Math.min(this.img.width, canvas.width);
                this.sHeight = this.dHeight = Math.min(this.img.height, canvas.height);
                this.redrawImage();
            };
        }
    }

    render() {
        return <div className={styles.TileEditorComponent}>
            <canvas onMouseDown={this.mouseDown.bind(this)}
                    onMouseUp={this.mouseUp.bind(this)}
                    onMouseLeave={this.mouseLeave.bind(this)}
                    className={styles.TileEditorCanvas}
                    ref="tileEditorCanvas"></canvas>
        </div>
    }

    mouseDown(e) {
        if (e.button === 2) {
            this.refs.tileEditorCanvas.addEventListener('mousemove', this.mouseMoveHandler);
        }
    }

    mouseUp(e) {
        if (e.button === 2) {
            this.refs.tileEditorCanvas.removeEventListener('mousemove', this.mouseMoveHandler);
        }
    }

    mouseLeave(e) {
        if (e.button === 2) {
            this.refs.tileEditorCanvas.removeEventListener('mousemove', this.mouseMoveHandler);
        }
    }

    mouseMove(e) {
        // console.log(this.img.src);
        if (this.img.src !== "") {
            this.sx -= e.movementX;
            this.sy -= e.movementY;
            // console.log(this.sx, this.sy);
            this.redrawImage();
        }
    }

    redrawImage() {
        if (this.img.src !== "") {
            const canvas = this.refs.tileEditorCanvas;
            const ctx = canvas.getContext('2d');
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.drawImage(this.img,
                          this.sx, this.sy,
                          this.sWidth, this.sHeight,
                          this.dx, this.dy,
                          this.dWidth, this.dHeight);
        }
    }
}