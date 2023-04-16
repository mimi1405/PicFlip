import { useEffect, useState } from 'react';
import React from 'react';
import './OutputPicType.css'
import { invoke } from '@tauri-apps/api';
import { PuffLoader } from 'react-spinners';

interface OutPutPicTypeProps {
    InputType: string;
    Path: string;
}

const OutPutPicType: React.FC<OutPutPicTypeProps> = ({ InputType, Path }) => {
    const outputTypes: string[] = ['PNG', 'JPEG', 'SVG', 'BMP', 'WEBP', 'GIF', 'JPG', 'TIFF', 'ICO', 'RAW', 'PGM'];
    const [extension, setExtension] = useState('PNG');
    const [path, setPath] = useState("");
    const [loading, setLoading] = useState(false);
    const [converted, setConverted] = useState(false);
  


    async function convert(): Promise<void> {
        setLoading(true);
        setPath(Path);
        let actualChoice = extension.toLowerCase();
        await invoke('convert_image', { path: Path, choice: actualChoice})
            .then((response) => {setLoading(false); setConverted(true);}).catch((error) => {
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
            {
                converted ? <><strong><p>Conversion finished successfully!</p></strong></> : <>{loading ? <PuffLoader /> : <button onClick={() => convert()} className='convert-btn'>Convert</button>}</>
            }
            
            
        </>
    );
};

export default OutPutPicType;