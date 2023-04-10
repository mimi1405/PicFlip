import React from 'react';
import Box from '../components/DragDropBox/Box';
import OutPutPicType from '../components/OutputPicType/OutputPicType';
import './Main.css'

const Main = () => {
    return (
        <>
            <div className='main-container'>
                <h1>PicFlip!</h1>
                <Box Image='lol' />
                <OutPutPicType InputType='t' />
                <button className='convert-btn'>Convert</button>
            </div>
        </>
    );
};

export default Main;
