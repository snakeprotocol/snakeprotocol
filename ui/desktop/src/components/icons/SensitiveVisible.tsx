import React from 'react';

export default function SensitiveVisible({ className = '' }) {
  return (
    <svg
      width="1.5rem"
      height="1.5rem"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
      viewBox="0 0 24 24"
      aria-hidden="true"
      className={className}
    >
      <path
        fillRule="evenodd"
        clipRule="evenodd"
        d="M6.97 14.23A16.635 16.635 0 0 1 4.343 12 16.633 16.633 0 0 1 6.97 9.77C8.42 8.79 10.167 8 12 8c1.833 0 3.58.789 5.03 1.77A16.631 16.631 0 0 1 19.657 12a16.633 16.633 0 0 1-2.627 2.23C15.58 15.21 13.834 16 12 16c-1.833 0-3.58-.789-5.03-1.77ZM12 6C9.608 6 7.469 7.018 5.849 8.114a18.644 18.644 0 0 0-3.333 2.916 1.45 1.45 0 0 0 0 1.94 18.644 18.644 0 0 0 3.333 2.916C7.469 16.982 9.609 18 12 18c2.392 0 4.531-1.018 6.151-2.114a18.644 18.644 0 0 0 3.333-2.916 1.45 1.45 0 0 0 0-1.94 18.644 18.644 0 0 0-3.333-2.916C16.531 7.018 14.392 6 12 6Zm0 8a2 2 0 1 0 0-4 2 2 0 0 0 0 4Z"
        fill="currentColor"
      ></path>
    </svg>
  );
}
