import React from 'react';
import Box from '../components/DragDropBox/Box';
import './Main.css'

const Main = () => {
    return (
        <>
            <div className='main-container'>
                <h1>PicFlip!</h1>
                <Box/>
                <button className='convert-btn'>Convert</button>
            </div>
        </>
    );
};

export default Main;