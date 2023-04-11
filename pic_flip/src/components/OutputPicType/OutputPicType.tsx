import { useEffect, useState } from 'react';
import React from 'react';
import './OutputPicType.css'

interface OutPutPicTypeProps {
    InputType: string;
}

const OutPutPicType: React.FC<OutPutPicTypeProps> = ({ InputType }) => {
    const outputTypes: string[] = ['PNG', 'JPEG', 'SVG', 'BMP', 'WEBP', 'GIF', 'JPG'];
    const [extension, setExtension] = useState('');
 
     return (
        <>
            <select placeholder='Drag and drop image first' className='select-box'>
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
        </>
    );
};

export default OutPutPicType;
