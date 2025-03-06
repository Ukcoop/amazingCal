'use client';

import React, { useState, useRef, useEffect } from 'react';
// @ts-expect-error no declaration file
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
// @ts-expect-error no declaration file
import ArrowDropUpIcon from '@mui/icons-material/ArrowDropUp';

interface DropDownParams {
  open: string;
  id: string;
  minimal: boolean;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  setOpen: any;
  element: React.ReactNode;
  options: Array<React.ReactNode>;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  returnIndex: any;
}

export default function DropDown({ open, id, minimal, setOpen, element, options, returnIndex }: DropDownParams) {
  const [dropdownStyle, setDropdownStyle] = useState({ top: 0, left: 0 });

  const dropdownRef = useRef<HTMLDivElement>(null);
  const buttonRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (
        dropdownRef.current &&
        buttonRef.current &&
        !dropdownRef.current.contains(event.target as Node) &&
        !buttonRef.current.contains(event.target as Node)
      ) {
        setOpen('None');
      }
    }

    if (id === open) {
      document.addEventListener('mousedown', handleClickOutside);
    }
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [id, open, setOpen]);

  useEffect(() => {
    if (id === open && dropdownRef.current && buttonRef.current) {
      const dropdownRect = dropdownRef.current.getBoundingClientRect();
      const buttonRect = buttonRef.current.getBoundingClientRect();
      const viewportWidth = window.innerWidth;
      const viewportHeight = window.innerHeight;

      let newTop = buttonRect.bottom;
      let newLeft = buttonRect.left;

      if (buttonRect.bottom + dropdownRect.height > viewportHeight) {
        newTop = buttonRect.top - dropdownRect.height;
      }

      if (newLeft + dropdownRect.width > viewportWidth) {
        newLeft = viewportWidth - dropdownRect.width - 10;
      }

      if (newLeft < 10) {
        newLeft = 10;
      }

      setDropdownStyle({ top: newTop + 5, left: newLeft });
    }
  }, [id, open]);

  // Determine the styles for the options container
  const optionsContainerStyle: React.CSSProperties =
    options.length > 8
      ? {
          maxHeight: '200px', // adjust this value based on your option height
          overflowY: 'auto',
        }
      : {};

  return (
    <div className="relative flex flex-col items-center p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-900">
      <div
        ref={buttonRef}
        className="flex items-center cursor-pointer"
        onClick={
          open === id
            ? () => {
                setOpen('None');
              }
            : () => {
                setOpen(id);
              }
        }
      >
        {!minimal && (open === id ? <ArrowDropUpIcon fontSize="large" /> : <ArrowDropDownIcon fontSize="large" />)}
        {element}
      </div>
      {id === open && (
        <div
          ref={dropdownRef}
          className="absolute z-10 flex bg-white dark:bg-gray-800 border border-black dark:border-gray-700 text-white rounded-md shadow-lg"
          style={{
            position: 'fixed',
            top: dropdownStyle.top,
            left: dropdownStyle.left
          }}
        >
          <div className="flex min-w-20 flex-col items-center" style={optionsContainerStyle}>
            {options.map((option, index) => {
              const notLast = index < options.length - 1;
              return (
                <div
                  key={`option-${element}-${index}`}
                  onClick={() => {
                    returnIndex(index);
                  }}
                  className={`flex items-center justify-center w-full px-2 py-1 ${notLast ? 'border border-transparent border-b-black dark:border-b-gray-700' : ''}`}
                >
                  <div className="text-black dark:text-white">{option}</div>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </div>
  );
}
