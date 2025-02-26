import CalendarData from "@/app/core/calendar";

interface DayParams {
  day: number,
  active: boolean
}

interface MonthViewParams {
  data: CalendarData,
  month: number
}

function Day({ day, active }: DayParams) {
  const inactiveStyle = 'text-gray-500';

  return (
    <div className="border-2 border-transparent border-r-gray-800 border-b-gray-800">
      <a className={`text-xl ${!active && inactiveStyle} ml-1 mt-2`}>{day}</a>
    </div>
  );
}

export default function MonthView({ data, month }: MonthViewParams) {
  const dayCompoents = [];
  const currentMonth = data.yearData[month];
  const previousMonth = data.yearData[month -1];

  const rows = (currentMonth.weekIndex + currentMonth.daysInMonth > 7 * 5) ? 'grid-rows-6' : 'grid-rows-5';
  const totalDays = (currentMonth.weekIndex + currentMonth.daysInMonth > 7 * 5) ? 7 * 6 : 7 * 5;

  // these checks are temporary
  if(previousMonth !== undefined) {
    for(let i = previousMonth.weekIndex; i >= 0; i--) {
      dayCompoents.push(<Day key={`day-prev-${previousMonth.daysInMonth - i}`} active={false} day={previousMonth.daysInMonth - i}/>);
    }
  }

  for (let i = 0; i < currentMonth.daysInMonth + 1; i++) {
    dayCompoents.push(<Day key={`day-mon-${i}`} active={true} day={i}/>);
  }

  if(previousMonth !== undefined) {
    for(let i = 0; i < totalDays - previousMonth.weekIndex - currentMonth.daysInMonth - 2; i++) {
      dayCompoents.push(<Day key={`day-next-${i}`} active={false} day={i}/>);
    }
  }

  return (
    <div className={`grid grid-cols-7 ${rows} content-start w-full h-full rounded-lg dark:bg-gray-900`}>
      {dayCompoents}
    </div>
  );
}
