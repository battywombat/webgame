import React, { Component } from 'react';
import styles from './CanvasRendererComponent.css';


export default class CanvasRendererComponent extends Component {

    componentWillMount() {
        this.mouseMoveHandler = this.mouseMove.bind(this);
        this.sx = 0;
        this.sy = 0;
        this.sWidth = 0;
        this.sHeight = 0;
        this.dWidth = 0;
        this.dHeight = 0;
        this.dx = 0;
        this.dy = 0;
    }

    componentDidUpdate() {
        if (this.props.srcImage === undefined) {
            return;
        }
        if (this.props.srcImage.constructor === HTMLImageElement) {
            onImageLoaded(this.props.srcImage, () => {
                this.sx = this.sy = 0;
                const canvas = this.refs.tileEditorCanvas;
                canvas.width = canvas.offsetWidth;
                canvas.height = canvas.offsetHeight;
                this.sWidth = this.dWidth =  this.props.srcImage.naturalWidth;
                this.sHeight = this.dHeight = this.props.srcImage.naturalHeight;
                this.redrawImage();
            });
        }
    }

    render() {
        return <canvas onMouseDown={this.mouseDown.bind(this)}
                       onMouseUp={this.mouseUp.bind(this)}
                       onMouseLeave={this.mouseLeave.bind(this)}
                       className={styles.CanvasRendererComponent}
                       ref="tileEditorCanvas"></canvas>
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
        if (this.props.srcImage !== undefined) {
            const scrollScale = this.props.scrollScale || 1;
            const canvas = this.refs.tileEditorCanvas;
            this.sx = Math.max(Math.min(this.sx - e.movementX*scrollScale, this.props.srcImage.width - canvas.width), 0);
            this.sy = Math.max(Math.min(this.sy - e.movementY*scrollScale, this.props.srcImage.height - canvas.height), 0);
            this.redrawImage();
        }
    }

    redrawImage() {
        if (this.props.srcImage !== undefined) {
            const canvas = this.refs.tileEditorCanvas;
            const ctx = canvas.getContext('2d');
            ctx.clearRect(0, 0, canvas.width, canvas.height);
            ctx.drawImage(this.props.srcImage,
                          this.sx, this.sy,
                          this.sWidth, this.sHeight,
                          this.dx, this.dy,
                          this.dWidth, this.dHeight);
        }
    }
}

function onImageLoaded(img, cb) {
    // Create a temporary image, so that if the image is alread loaded, it will be triggered anyway.
    let testImage = new Image();
    testImage.src = img.src;
    testImage.onload = cb;
}