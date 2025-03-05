import CalendarData from '@/app/core/calendar';
import { EventDisplayManager, Event } from '@/app/core/eventManager';

import Modal from '../modal';
import EditEvent from '../modals/editEvent';

interface DayParams {
  dayKey: string;
  events: Array<Event>;
  day: number;
  active: boolean;
  today: boolean;
  showRightEdge: boolean;
  showBottomEdge: boolean;
  modal: string | null;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  setModal: any;
}

interface MonthViewParams {
  data: CalendarData;
  month: number;
  year: number;
  modal: string | null;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  setModal: any;
}

function Day({ dayKey, events, day, active, today, showRightEdge, showBottomEdge, modal, setModal }: DayParams) {
  const inactiveStyle = 'text-gray-500';
  const todayStyle = 'rounded-full text-white bg-black dark:text-black dark:bg-white';

  const style = today ? todayStyle : !active && inactiveStyle;

  return (
    <div
      className={`flex flex-col border-2 border-transparent ${showRightEdge && 'border-r-gray-800'} ${showBottomEdge && 'border-b-gray-800'}`}
    >
      <a className={`text-xl ${style} px-2 py-1 m-1`}>{day}</a>
      <div className="flex flex-col w-full overflow-auto">
        {events !== undefined &&
          events.map((event, index) => {
            return (
              <div
                key={`day-${event.start.month}-${event.start.day}`}
                onClick={() => {
                  setModal(`${dayKey}-${index}`);
                }}
                className="flex justify-between px-2 pb-1"
              >
                <a>{event.name}</a>
                <a>{`${event.start.hour}:${event.start.minute.toString().padStart(2, '0')}`}</a>
                {modal == `${dayKey}-${index}` && (
                  <Modal
                    title={event.name}
                    component={<EditEvent dayKey={dayKey} index={index} />}
                    setModal={setModal}
                  />
                )}
              </div>
            );
          })}
      </div>
    </div>
  );
}

export default function MonthView({ data, month, year, modal, setModal }: MonthViewParams) {
  const dayCompoents = [];
  const currentMonth = data.getYearData(year)[month];
  const previousMonth = month == 0 ? data.getYearData(year - 1)[11] : data.getYearData(year)[month - 1];

  const rows = currentMonth.weekIndex + currentMonth.daysInMonth > 7 * 5 ? 'grid-rows-6' : 'grid-rows-5';
  const totalDays = currentMonth.weekIndex + currentMonth.daysInMonth > 7 * 5 ? 7 * 6 : 7 * 5;

  const [todaysMonth, todaysYear, todaysDay] = data.getTodaysDate();

  for (let i = currentMonth.weekIndex; i > 0; i--) {
    const isThisDay = month - 1 == todaysMonth && year == todaysYear && previousMonth.daysInMonth - i + 1 == todaysDay;
    const dayKey = `${'default'}-${year}-${month}-${previousMonth.daysInMonth - i}`;

    dayCompoents.push(
      <Day
        dayKey={dayKey}
        key={`day-prev-${previousMonth.daysInMonth - i + 1}`}
        events={EventDisplayManager.getInstance().getEvents()[dayKey]}
        active={false}
        today={isThisDay}
        showRightEdge={true}
        showBottomEdge={true}
        day={previousMonth.daysInMonth - i + 1}
        modal={modal}
        setModal={setModal}
      />
    );
  }

  for (let i = 0; i < currentMonth.daysInMonth; i++) {
    const isThisDay = month == todaysMonth && year == todaysYear && i + 1 == todaysDay;
    const dayKey = `${'default'}-${year}-${month}-${i}`;

    const showRightEdge = (i + previousMonth.daysInMonth) % 7 !== 0;
    const showBottomEdge = i + currentMonth.weekIndex < totalDays - 7;

    dayCompoents.push(
      <Day
        dayKey={dayKey}
        events={EventDisplayManager.getInstance().getEvents()[dayKey]}
        key={`day-mon-${i}`}
        active={true}
        today={isThisDay}
        showRightEdge={showRightEdge}
        showBottomEdge={showBottomEdge}
        day={i + 1}
        modal={modal}
        setModal={setModal}
      />
    );
  }

  for (let i = 0; i < totalDays - currentMonth.weekIndex - currentMonth.daysInMonth; i++) {
    const isThisDay = month + 1 == todaysMonth && year == todaysYear && i + 1 == todaysDay;
    const dayKey = `${'default'}-${year}-${month}-${i}`;

    const showRightEdge = (i + previousMonth.daysInMonth + currentMonth.daysInMonth) % 7 !== 0;
    const showBottomEdge = i + currentMonth.weekIndex + currentMonth.daysInMonth < totalDays - 7;

    dayCompoents.push(
      <Day
        dayKey={dayKey}
        events={EventDisplayManager.getInstance().getEvents()[dayKey]}
        key={`day-next-${i}`}
        active={false}
        today={isThisDay}
        showRightEdge={showRightEdge}
        showBottomEdge={showBottomEdge}
        day={i + 1}
        modal={modal}
        setModal={setModal}
      />
    );
  }

  return (
    <div
      className={`grid grid-cols-7 ${rows} content-start w-full h-full border-2 border-black dark:border-transparent rounded-lg dark:bg-gray-900`}
    >
      {dayCompoents}
    </div>
  );
}
