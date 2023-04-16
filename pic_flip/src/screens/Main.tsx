import React from 'react';
import Box from '../components/DragDropBox/Box';
import {FcRefresh} from 'react-icons/fc';
import {ImExit} from 'react-icons/im';
import { relaunch, exit } from '@tauri-apps/api/process';
import './Main.css'

const Main = () => {

    

    const restart = async () => {
        await relaunch();
    }

    return (
        <>
            <div className='main-container'>
                <h1>PicFlip!</h1>
                <Box/>
                <div className='desc-box'><p className='desc'>PicFlip! - Free MacOS image converting tool</p></div>
                <div className='system'>
                <FcRefresh onClick={() => restart()} className='refresh' />
                <ImExit onClick={() => exit(1)} className='exit' />
                </div>
            </div>
        </>
    );
};

export default Main;