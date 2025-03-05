import MonthView from './views/monthView';
import WeekView from './views/weekView';

import CalendarData from '../core/calendar';

const data = new CalendarData();

type Views = 'Month' | 'Week';

interface CalendarViewParams {
  view: Views;
  month: number;
  year: number;
  modal: string | null;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  setModal: any;
}

export default function CalendarView({ view, month, year, modal, setModal }: CalendarViewParams) {
  if (view == 'Month') {
    return <MonthView data={data} month={month} year={year} modal={modal} setModal={setModal} />;
  } else if (view == 'Week') {
    return <WeekView />;
  } else {
    return (
      <div className="flex w-full h-full items-center justify-center rounded-lg dark:bg-gray-900">Calendar view.</div>
    );
  }
}
