import React, { useState } from 'react';
import { BiImageAdd } from "react-icons/bi";
import './Box.css'

interface BoxProps {
    Image: string;
}

const Box: React.FC<BoxProps> = ({ Image }) => {
    const [dropped, setDropped] = useState(false);
    return (
        <>
            {
                dropped ? <img src={Image}></img> : <div className="container">
                    <p className='container-text'>Drag and drop image here</p>
                    <BiImageAdd className='drop-icon' size={50} />
                </div>
            }
        </>
    );
};

export default Box;