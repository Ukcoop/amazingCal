export default function MonthView() {
  const dayCompoents = [];

  for (let i = 0; i < 32; i++) {
    dayCompoents.push(
      <div key={`day-${i}`} className="border-2 border-transparent border-r-gray-800 border-b-gray-800">
        {i}
      </div>
    );
  }

  return (
    <div className="grid grid-cols-7 grid-rows-6 content-start w-full h-full rounded-lg dark:bg-gray-900">
      {dayCompoents}
    </div>
  );
}
