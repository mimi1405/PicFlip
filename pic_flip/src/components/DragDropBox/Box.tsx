import React, { useState } from 'react';
import { BiImageAdd } from "react-icons/bi";
import './Box.css'
import { fs } from '@tauri-apps/api';
import path from 'path';
import { event } from '@tauri-apps/api';
import { convertFileSrc } from '@tauri-apps/api/tauri';
import OutPutPicType from '../OutputPicType/OutputPicType';



interface BoxProps {
}

const Box: React.FC<BoxProps> = ({ }) => {
    const [dropped, setDropped] = useState(false);
    const [imagePath, setImagePath] = useState<string>("");

    event.listen('tauri://file-drop', event => {
        let file = event.payload as Array<string>;
        let path = file[0];
        setImagePath(convertFileSrc(path)); //prepare image for webviewloader --> otherwise can't display in UI
        setDropped(true);
    })

    const returnExtension = () => {
        const parts = imagePath.split('.');
        const fileExtension = `.${parts[parts.length - 1]}`;
        console.log(fileExtension);
        return fileExtension;
    }


    return (
        <>
            {
                dropped ? <><img className='current-img-display' alt='dropped image' src={imagePath} /> <OutPutPicType InputType={returnExtension()} /> </> : <div className="container">
                    <p className='container-text'>Drag and drop image here</p>
                    <BiImageAdd className='drop-icon' size={50} />
                </div>


            }


        </>
    );
};

export default Box;