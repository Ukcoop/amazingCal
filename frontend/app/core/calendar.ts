interface MonthData {
  weekIndex: number;
  daysInMonth: number;
}

function getDaysInMonth(date: Date) {
  const year = date.getFullYear();
  const month = date.getMonth();
  const nextMonth = new Date(year, month + 1, 0);
  return nextMonth.getDate();
}

export default class CalendarData {
  yearData: Array<MonthData>;

  constructor() {
    this.yearData = [];

    for (let i = 0; i < 12; i++) {
      const day = new Date(2025, i, 1);
      const month: MonthData = {
        weekIndex: day.getDay(),
        daysInMonth: getDaysInMonth(day)
      };

      this.yearData.push(month);
    }
  }
}
