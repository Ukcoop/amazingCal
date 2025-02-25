'use client';

import MonthView from './views/monthView';
import WeekView from './views/weekView';

type Views = 'Month' | 'Week';

interface MonthViewParams {
  view: Views;
}

export default function CalendarView({ view }: MonthViewParams) {
  if (view == 'Month') {
    return <MonthView />;
  } else if (view == 'Week') {
    return <WeekView />;
  } else {
    return (
      <div className="flex w-full h-full items-center justify-center rounded-lg dark:bg-gray-900">Calendar view.</div>
    );
  }
}
