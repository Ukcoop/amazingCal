'use client';

import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
// import ArrowDropUpIcon from '@mui/icons-material/ArrowDropUp';

interface DropDownParams {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  element: any;
}

export default function DropDown({ element }: DropDownParams) {
  return (
    <div className="flex items-center p-1 rounded-md hover:bg-gray-900">
      <ArrowDropDownIcon fontSize="large" />
      {element}
    </div>
  );
}
