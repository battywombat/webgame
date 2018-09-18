import React, { Component } from 'react';
import styles from './TileListComponent.css';

class TileComponent extends Component {

    render() {
        return <img width={this.props.width}
                    height={this.props.height}
                    style="background-image: {this.props.img}"></img>
    }
}

export default class TileListComponent extends Component {

    render() {
        const tiles = this.props.tiles.map((tileInfo) => <TileComponent width={this.props.width}
                                                                        height={this.props.height}
                                                                        src={tileInfo.src} />);
        return <div className={styles.TileListComponent}>
            {tiles}
        </div>
    }
}