const styles = {
  primary:
    'flex items-center justify-center h-10 my-1 bg-black dark:bg-gray-300 hover:bg-gray-900 hover:dark:bg-white text-white dark:text-black rounded-md p-2 px-4',
  secondary:
    'flex items-center justify-center h-10 my-1 bg-transparent border-2 border-black dark:border-gray-500 hover:bg-gray-200 hover:dark:bg-gray-900 text-black dark:text-white rounded-md p-2 px-4'
};

interface ButtonParams {
  testId: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  text: any;
  style: keyof typeof styles;
  width: string;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  onClick: any;
}

export default function Button({ testId, text, style, width, onClick }: ButtonParams) {
  return (
    <div data-cy={testId} className={`${styles[style]} ${width}`} onClick={onClick}>
      <a>{text}</a>
    </div>
  );
}
