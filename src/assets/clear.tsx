import React from 'react';

const ClearIcon: React.FC<React.SVGProps<SVGSVGElement>> = (props) => (
    <svg
        xmlns="http://www.w3.org/2000/svg"
        width="24"
        height="24"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="icon icon-tabler icon-tabler-clear-all"
        {...props}
    >
        <path stroke="none" d="M0 0h24v24H0z" fill="none" />
        <path d="M8 6h12" />
        <path d="M6 12h12" />
        <path d="M4 18h12" />
    </svg>
);

export default ClearIcon;
