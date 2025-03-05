import React from 'react';
// @ts-expect-error no decloration file
import CloseIcon from '@mui/icons-material/Close';

interface ModalParams {
  title: string;
  component: React.ReactNode;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  setModal: any;
}

export default function Modal({ title, component, setModal }: ModalParams) {
  const closeFunction = (e: React.MouseEvent) => {
    e.stopPropagation();
    setModal(null);
  };

  return (
    <div
      onClick={e => e.stopPropagation()}
      className="flex items-center justify-center absolute top-0 left-0 w-full h-full bg-black/20 backdrop-blur-sm"
    >
      <div className="min-w-80 p-4 rounded-md bg-white dark:bg-gray-800">
        <div className="flex justify-between">
          <a className="text-xl">{title}</a>
          <CloseIcon onClick={closeFunction} />
        </div>
        {component}
      </div>
    </div>
  );
}
