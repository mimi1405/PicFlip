import { useEffect, useState } from 'react';
import React from 'react';
import './OutputPicType.css'
import { invoke } from '@tauri-apps/api';

interface OutPutPicTypeProps {
    InputType: string;
    Path: string;
}

const OutPutPicType: React.FC<OutPutPicTypeProps> = ({ InputType, Path }) => {
    const outputTypes: string[] = ['PNG', 'JPEG', 'SVG', 'BMP', 'WEBP', 'GIF', 'JPG'];
    const [extension, setExtension] = useState('PNG');
    const [path, setPath] = useState("");


    async function convert(): Promise<void> {
        setPath(Path);
        let actualChoice = extension.toLowerCase();
        console.log(actualChoice);
        await invoke('convert_image', { path: Path, choice: actualChoice})
            .then((response) => console.log(actualChoice)).catch((error) => {
                console.error(error);
            })
    }
 
     return (
        <>
            <select onChange={(e) => setExtension(e.target.value)} value={extension} placeholder='Drag and drop image first' className='select-box'>
                {
                    outputTypes.map((value) => {
                        return (
                            <>
                                <option disabled={InputType == '.' + value.toLowerCase() ? true : false}>{value}</option>
                            </>
                        )
                    })
                }
            </select>
            <button onClick={() => convert()} className='convert-btn'>Convert</button>
        </>
    );
};

export default OutPutPicType;