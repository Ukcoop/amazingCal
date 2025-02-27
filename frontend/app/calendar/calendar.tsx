'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { createClient } from '@/utils/supabase/client';
import type { User } from '@supabase/supabase-js';

import axios from 'axios';

// @ts-expect-error no decloration file
import Jdenticon from 'react-jdenticon';

import Button from '../components/button';
import DropDown from '../components/dropDown';
import CalendarView from '../components/calendarView';

import MenuIcon from '@mui/icons-material/Menu';
import AddIcon from '@mui/icons-material/Add';

import KeyboardArrowLeftIcon from '@mui/icons-material/KeyboardArrowLeft';
import KeyboardArrowRightIcon from '@mui/icons-material/KeyboardArrowRight';

import CalendarData from '../core/calendar';

interface calendar {
  // more of the calendar type will be added when they are needed
  name: string;
}

function getMonthName(monthNumber: number) {
  const date = new Date();
  date.setMonth(monthNumber);
  return date.toLocaleString('en-US', { month: 'long' });
}

export default function Calendar({ baseUrl }: { baseUrl: string }) {
  const router = useRouter();
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState('');
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const [calendars, setCalendars]: [Array<calendar>, any] = useState([]);

  const [menu, setMenu] = useState(false);

  const [todaysMonth, todaysYear] = new CalendarData().getTodaysDate();
  const [month, setMonth] = useState(todaysMonth);
  const [year, setYear] = useState(todaysYear);

  const toggleMenu = () => {
    setMenu(!menu);
  };

  const fowardOneMonth = () => {
    if (month == 11) {
      setMonth(0);
      setYear(year + 1);
    } else {
      setMonth(month + 1);
    }
  };

  const backwardOneMonth = () => {
    if (month == 0) {
      setMonth(11);
      setYear(year - 1);
    } else {
      setMonth(month - 1);
    }
  };

  useEffect(() => {
    const supabase = createClient();

    async function fetchSession() {
      const { data, error } = await supabase.auth.getSession();

      if (error || !data.session?.user) {
        router.replace('/login');
      } else {
        setUser(data.session.user);
        setToken(data.session.access_token);
      }
    }

    fetchSession();
  }, [router]);

  useEffect(() => {
    if (token == '') return;

    const getUserData = async () => {
      try {
        const response = await axios.get(baseUrl + 'get/userData', {
          headers: {
            'Content-Type': 'application/json',
            Authorization: token
          }
        });

        console.log(response.data);
        setCalendars(response.data.calendars);
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
      } catch (e: any) {
        if (e.status == 401) {
          router.push('/login');
        } else {
          console.error(e);
        }
      }
    };

    getUserData();
  }, [token, router, baseUrl]);

  const clickableElementClass = 'p-1 rounded-md hover:bg-gray-900';

  return (
    <div className="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
      <div className="w-full h-10 mb-2 flex items-center justify-between">
        <div className="flex items-center">
          <Button testId="cy-menu" text={<MenuIcon />} style="secondary" width="w-max" onClick={toggleMenu} />
          <a className="text-2xl pl-4 pr-2">amazingCal</a>
          <div className="flex px-2">
            <div className={clickableElementClass} onClick={backwardOneMonth}>
              <KeyboardArrowLeftIcon fontSize="large" />
            </div>
            <div className={clickableElementClass} onClick={fowardOneMonth}>
              <KeyboardArrowRightIcon fontSize="large" />
            </div>
          </div>
          <a className="text-2xl">{`${getMonthName(month)} ${year}`}</a>
        </div>
        <div className="flex items-center">
          <DropDown element={<p>Month</p>} />
          <DropDown element={<Jdenticon size="40" value={user?.email} />} />
        </div>
      </div>
      <div className="flex h-full">
        <div className={`flex flex-col ${menu ? 'w-60' : 'w-16'} h-full mr-2`}>
          <Button
            testId="cy-create-button"
            text={
              <div className="flex items-center text-lg">
                <AddIcon />
                {menu && <p className="pl-1">Event</p>}
              </div>
            }
            style="secondary"
            width="w-max"
            onClick={() => {
              console.log('New event');
            }}
          />
          {menu && (
            <div>
              <div className="flex justify-between my-2">
                <p>Calendars</p>
                <div className="flex items-center">
                  <AddIcon />
                </div>
              </div>
              {calendars.map((calendar, index) => {
                return <p key={`calendar-m${index}`}>{calendar.name}</p>;
              })}
            </div>
          )}
        </div>
        <CalendarView view="Month" month={month} year={year} />
      </div>
    </div>
  );
}
