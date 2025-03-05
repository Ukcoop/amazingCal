'use client';

import React, { useState, useRef, useEffect } from 'react';
// @ts-expect-error no decloration file
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
// import ArrowDropUpIcon from '@mui/icons-material/ArrowDropUp';

interface DropDownParams {
  open: string;
  id: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  setOpen: any;
  element: React.ReactNode;
  options: Array<React.ReactNode>;
}

export default function DropDown({ open, id, setOpen, element, options }: DropDownParams) {
  const [dropdownStyle, setDropdownStyle] = useState({ top: 0, left: 0, minWidth: '0px' });

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

    if (id == open) {
      document.addEventListener('mousedown', handleClickOutside);
    }
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, [id, open, setOpen]);

  // Adjust dropdown position and width
  useEffect(() => {
    if (id == open && dropdownRef.current && buttonRef.current) {
      const dropdownRect = dropdownRef.current.getBoundingClientRect();
      const buttonRect = buttonRef.current.getBoundingClientRect();
      const viewportWidth = window.innerWidth;
      const viewportHeight = window.innerHeight;

      let newTop = buttonRect.bottom; // Default position below button
      let newLeft = buttonRect.left; // Align left with button
      const minWidth = `${buttonRect.width}px`; // Set min-width to button width

      // Flip dropdown if it overflows the bottom of the screen
      if (buttonRect.bottom + dropdownRect.height > viewportHeight) {
        newTop = buttonRect.top - dropdownRect.height; // Move above button
      }

      // Prevent right overflow
      if (newLeft + dropdownRect.width > viewportWidth) {
        newLeft = viewportWidth - dropdownRect.width - 10; // Keep 10px margin
      }

      // Prevent left overflow
      if (newLeft < 10) {
        newLeft = 10; // Keep 10px margin
      }

      setDropdownStyle({ top: newTop + 5, left: newLeft, minWidth: minWidth });
    }
  }, [id, open]);

  return (
    <div className="relative flex flex-col items-center p-1 rounded-md hover:bg-gray-200 dark:hover:bg-gray-900">
      <div
        ref={buttonRef}
        className="flex items-center cursor-pointer"
        onClick={
          open == id
            ? () => {
                setOpen('None');
              }
            : () => {
                setOpen(id);
              }
        }
      >
        <ArrowDropDownIcon fontSize="large" />
        {element}
      </div>
      {id == open && (
        <div
          ref={dropdownRef}
          className="absolute z-10 bg-gray-700 text-white rounded-md shadow-lg p-2"
          style={{
            position: 'fixed',
            top: dropdownStyle.top,
            left: dropdownStyle.left,
            minWidth: dropdownStyle.minWidth // Apply minWidth dynamically
          }}
        >
          <div className="flex flex-col items-center">{options}</div>
        </div>
      )}
    </div>
  );
}
