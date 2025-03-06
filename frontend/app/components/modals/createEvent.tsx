'use client';

import { useState } from "react";

//import { EventManager } from "@/app/core/eventManager";
import CalendarData from "@/app/core/calendar";

import DropDown from "../dropDown";

interface CreateEventParams {
  month: number;
  year: number;
}

function getMonthName(monthNumber: number) {
  const date = new Date();
  date.setMonth(monthNumber);
  return date.toLocaleString('en-US', { month: 'long' });
}

function getOrdinal(n: number) {
  return n + ['th', 'st', 'nd', 'rd'][n % 10 > 3 ? 0 : n % 10];
}

const months: Array<string> = [];
const hours: Array<number> = [];
const minutes: Array<string> = [];

for(let i = 0; i < 12; i++) {
  months.push(getMonthName(i));
  hours.push(i + 1);
}

for(let i = 0; i < 60; i += 5) {
  minutes.push(i.toString().padStart(2, '0'));
}

const calendarData = new CalendarData();

export default function CreateEvent({ month, year }: CreateEventParams) {
  // eslint-disable-next-line react-hooks/rules-of-hooks
  const [open, setOpen] = useState('None');
  // eslint-disable-next-line react-hooks/rules-of-hooks
  const [selectedDay, setDay] = useState(0);
  // eslint-disable-next-line react-hooks/rules-of-hooks
  const [selectedMonth, setMonth] = useState(month);
  // eslint-disable-next-line react-hooks/rules-of-hooks
  const [selectedYear, setYear] = useState(year);
  //EventManager.getInstance().postEvent();

  const yearData = calendarData.getYearData(year);

  const years: Array<number> = [];
  const days: Array<string> = [];

  for(let i = 0; i < 10; i++) {
    years.push(i + year);
  }

  for(let i = 0; i < yearData[month].daysInMonth; i++) {
    days.push(getOrdinal(i + 1));
  }

  const handleDay = (i: number) => {
    setDay(i);
    setOpen('None');
  }

  const handleMonth = (i: number) => {
    setMonth(i);
    setOpen('None');
  }

  const handleYear = (i: number) => {
    setYear(i + year);
    setOpen('None');
  }

  return (
    <div>
      <div className="flex items-center">
        <DropDown open={open} id="Day" minimal={true} setOpen={setOpen} element={getOrdinal(selectedDay + 1)} options={days} returnIndex={handleDay} />
        <a>Of</a>
        <DropDown open={open} id="Month" minimal={true} setOpen={setOpen} element={getMonthName(selectedMonth)} options={months} returnIndex={handleMonth} />
        <a>,</a>
        <DropDown open={open} id="Year" minimal={true} setOpen={setOpen} element={selectedYear} options={years} returnIndex={handleYear} />
        <a>@</a>
        <DropDown open={open} id="Hour" minimal={true} setOpen={setOpen} element={'8'} options={hours} returnIndex={() => {}} />
        <a>:</a>
        <DropDown open={open} id="Minute" minimal={true} setOpen={setOpen} element={'00'} options={minutes} returnIndex={() => {}} />
        <DropDown open={open} id="Period" minimal={true} setOpen={setOpen} element={'AM'} options={['AM', 'PM']} returnIndex={() => {}} />
      </div>
    </div>
  );
}
