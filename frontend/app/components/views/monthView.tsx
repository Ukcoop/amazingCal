import CalendarData from '@/app/core/calendar';
import { EventDisplayManager, Event } from '@/app/core/eventManager';

interface DayParams {
  events: Array<Event>;
  day: number;
  active: boolean;
  today: boolean;
}

interface MonthViewParams {
  data: CalendarData;
  month: number;
  year: number;
}

function Day({ events, day, active, today }: DayParams) {
  const inactiveStyle = 'text-gray-500';
  const todayStyle = 'rounded-full text-gray-900 bg-white';

  const style = today ? todayStyle : !active && inactiveStyle;

  return (
    <div className="flex flex-col border-2 border-transparent border-r-gray-800 border-b-gray-800">
      <a className={`text-xl ${style} px-2 py-1 m-1`}>{day}</a>
      <div className="flex flex-col w-full overflow-auto">
        {events !== undefined &&
          events.map(event => {
            return (
              <div key={`day-${event.start.month}-${event.start.day}`} className="flex justify-between px-2 pb-1">
                <a>{event.name}</a>
                <a>{`${event.start.hour}:${event.start.minute.toString().padStart(2, '0')}`}</a>
              </div>
            );
          })}
      </div>
    </div>
  );
}

export default function MonthView({ data, month, year }: MonthViewParams) {
  const dayCompoents = [];
  const currentMonth = data.getYearData(year)[month];
  const previousMonth = month == 0 ? data.getYearData(year - 1)[11] : data.getYearData(year)[month - 1];

  const rows = currentMonth.weekIndex + currentMonth.daysInMonth > 7 * 5 ? 'grid-rows-6' : 'grid-rows-5';
  const totalDays = currentMonth.weekIndex + currentMonth.daysInMonth > 7 * 5 ? 7 * 6 : 7 * 5;

  const [todaysMonth, todaysYear, todaysDay] = data.getTodaysDate();

  for (let i = currentMonth.weekIndex; i > 0; i--) {
    const isThisDay = month - 1 == todaysMonth && year == todaysYear && previousMonth.daysInMonth - i + 1 == todaysDay;
    const eventKey = `${'default'}-${year}-${month}-${previousMonth.daysInMonth - i}`;

    dayCompoents.push(
      <Day
        key={`day-prev-${previousMonth.daysInMonth - i + 1}`}
        events={EventDisplayManager.getInstance().getEvents()[eventKey]}
        active={false}
        today={isThisDay}
        day={previousMonth.daysInMonth - i + 1}
      />
    );
  }

  for (let i = 0; i < currentMonth.daysInMonth; i++) {
    const isThisDay = month == todaysMonth && year == todaysYear && i + 1 == todaysDay;
    const eventKey = `${'default'}-${year}-${month}-${i}`;

    dayCompoents.push(
      <Day
        events={EventDisplayManager.getInstance().getEvents()[eventKey]}
        key={`day-mon-${i}`}
        active={true}
        today={isThisDay}
        day={i + 1}
      />
    );
  }

  for (let i = 0; i < totalDays - currentMonth.weekIndex - currentMonth.daysInMonth; i++) {
    const isThisDay = month + 1 == todaysMonth && year == todaysYear && i + 1 == todaysDay;
    const eventKey = `${'default'}-${year}-${month}-${i}`;

    dayCompoents.push(
      <Day
        events={EventDisplayManager.getInstance().getEvents()[eventKey]}
        key={`day-next-${i}`}
        active={false}
        today={isThisDay}
        day={i + 1}
      />
    );
  }

  return (
    <div className={`grid grid-cols-7 ${rows} content-start w-full h-full rounded-lg dark:bg-gray-900`}>
      {dayCompoents}
    </div>
  );
}
