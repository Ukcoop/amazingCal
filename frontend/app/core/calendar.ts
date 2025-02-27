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
  getYearData(year: number) {
    const yearData = [];
    for (let i = 0; i < 12; i++) {
      const day = new Date(year, i, 1);
      const month: MonthData = {
        weekIndex: day.getDay(),
        daysInMonth: getDaysInMonth(day)
      };

      yearData.push(month);
    }

    return yearData;
  }

  getTodaysDate() {
    const date = new Date();
    const month = date.getMonth();
    const year = date.getFullYear();
    const day = date.getDate();

    return [month, year, day];
  }
}
