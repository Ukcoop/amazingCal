const styles = {
  primary:
    'flex items-center justify-center w-max h-10 ml-2 bg-black dark:bg-gray-300 hover:bg-gray-900 hover:dark:bg-white text-white dark:text-black rounded-md p-2 px-4',
  secondary:
    'flex items-center justify-center w-max h-10 ml-2 bg-transparent border-2 border-black dark:border-gray-500 hover:bg-gray-200 hover:dark:bg-gray-900 text-black dark:text-white rounded-md p-2 px-4'
};

interface LinkParams {
  testId: string;
  text: string;
  style: keyof typeof styles;
  href: string;
}

export default function Link({ testId, text, style, href }: LinkParams) {
  return (
    <a data-cy={testId} className={styles[style]} href={href}>
      {text}
    </a>
  );
}
