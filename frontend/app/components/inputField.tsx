interface InputFieldParams {
  testId: string;
  type: string;
  value: string | number;
  setValue: (input: string) => unknown;
  min?: string;
  onKeyPress?: () => unknown;
}

export default function InputField({
  testId,
  type,
  value,
  setValue,
  min = '',
  onKeyPress = () => {}
}: InputFieldParams) {
  return (
    <div className="w-full mb-2">
      <input
        data-cy={testId}
        className="w-full h-10 pl-2 text-2xl dark:text-white dark:bg-gray-950 border-2 border-gray-500 rounded-md outline-none focus:border-black focus:dark:border-white"
        type={type}
        value={value}
        onChange={event => setValue(event.target.value)}
        min={min}
        onKeyPress={onKeyPress}
      />
    </div>
  );
}
