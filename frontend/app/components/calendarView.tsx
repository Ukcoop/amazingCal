import MonthView from './views/monthView';
import WeekView from './views/weekView';

import CalendarData from '../core/calendar';
const data = new CalendarData();

type Views = 'Month' | 'Week';

interface CalendarViewParams {
  view: Views;
  month: number;
  year: number;
}

export default function CalendarView({ view, month, year }: CalendarViewParams) {
  if (view == 'Month') {
    return <MonthView data={data} month={month} year={year} />;
  } else if (view == 'Week') {
    return <WeekView />;
  } else {
    return (
      <div className="flex w-full h-full items-center justify-center rounded-lg dark:bg-gray-900">Calendar view.</div>
    );
  }
}
