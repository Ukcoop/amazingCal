import { EventDisplayManager, Event } from '@/app/core/eventManager';

interface EditEventParams {
  dayKey: string;
  index: number;
}

function getMonthName(monthNumber: number) {
  const date = new Date();
  date.setMonth(monthNumber);
  return date.toLocaleString('en-US', { month: 'long' });
}

function getOrdinal(n: number) {
  return n + ['th', 'st', 'nd', 'rd'][n % 10 > 3 ? 0 : n % 10];
}

function formatTime(hours: number, minutes: number) {
  const amOrPm = hours >= 12 ? 'pm' : 'am';
  hours = hours % 12 || 12;
  const strMinutes = minutes < 10 ? '0' + minutes : minutes;
  return `${hours}:${strMinutes} ${amOrPm}`;
}

export default function EditEvent({ dayKey, index }: EditEventParams) {
  const event: Event = EventDisplayManager.getInstance().getEvents()[dayKey][index];

  const startDay = `${getOrdinal(event.start.day + 1)} of ${getMonthName(event.start.month)}`;
  const endDay = `${getOrdinal(event.end.day + 1)} of ${getMonthName(event.end.month)}`;

  const startTime = formatTime(event.start.hour, event.start.minute);
  const endTime = formatTime(event.end.hour, event.end.minute);

  return (
    <div className="w-96">
      <div className="flex justify-between mt-4">
        <a>Starts:</a>
        <div className="flex">{`${startDay}, ${startTime}`}</div>
      </div>
      <div className="flex justify-between">
        <a>Ends:</a>
        <div className="flex">{`${endDay}, ${endTime}`}</div>
      </div>
    </div>
  );
}
