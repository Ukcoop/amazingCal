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
import Modal from '../components/modal';

import CreateEvent from '../components/modals/createEvent';
import CreateCalendar from '../components/modals/createCalendar';

import CalendarView from '../components/calendarView';

import MenuIcon from '@mui/icons-material/Menu';
// @ts-expect-error no decloration file
import AddIcon from '@mui/icons-material/Add';

// @ts-expect-error no decloration file
import KeyboardArrowLeftIcon from '@mui/icons-material/KeyboardArrowLeft';
// @ts-expect-error no decloration file
import KeyboardArrowRightIcon from '@mui/icons-material/KeyboardArrowRight';

import CalendarData from '../core/calendar';
import { EventDisplayManager, Event } from '../core/eventManager';

interface Calendar {
  name: string;
  events: Array<Event>;
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
  const [calendars, setCalendars]: [Array<Calendar>, any] = useState([]);

  const [menu, setMenu] = useState(false);
  const [open, setOpen] = useState('None');
  const [view, setView] = useState<'Month' | 'Week'>('Month');
  const [modal, setModal] = useState<string | null>(null);

  const [todaysMonth, todaysYear] = new CalendarData().getTodaysDate();
  const [month, setMonth] = useState(todaysMonth);
  const [year, setYear] = useState(todaysYear);

  const supabase = createClient();

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

  const handleSignOut = async () => {
    await supabase.auth.signOut();
    // Optionally, redirect the user to a different page
    window.location.href = '/login';
  };

  const handleViewMenu = (index: number) => {
    if(index == 0) {
      setView("Month");
    } else {
      setView("Week");
    }
    setOpen("None");
  }

  const handleAccountMenu = (index: number) => {
    if(index == 0) {
      handleSignOut();
    }
    setOpen("None");
  }

  useEffect(() => {
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
  }, [router, supabase.auth]);

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

        const calendars = response.data.calendars;
        EventDisplayManager.getInstance().clearEvents();

        for (let i = 0; i < calendars.length; i++) {
          for (let j = 0; j < calendars[i].events.length; j++) {
            EventDisplayManager.getInstance().addEvent(calendars[i].name, calendars[i].events[j]);
          }
        }

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

  const clickableElementClass = 'p-1 rounded-md hover:bg-gray-200 hover:dark:bg-gray-900';

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
          <DropDown
            id="viewSelector"
            minimal={false}
            open={open}
            setOpen={setOpen}
            element={<p>{view}</p>}
            options={["Month", "Week"]}
            returnIndex={handleViewMenu}
          />
          <DropDown
            id="account"
            minimal={false}
            open={open}
            setOpen={setOpen}
            element={<Jdenticon size="40" value={user?.email} />}
            options={["Sign out"]}
            returnIndex={handleAccountMenu}
          />
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
              setModal('CreateEvent');
            }}
          />
          {menu && (
            <div>
              <div className="flex justify-between items-center my-2">
                <p>Calendars</p>
                <div className={`flex items-center ${clickableElementClass}`}>
                  <AddIcon
                    onClick={() => {
                      setModal('CreateCaledar');
                    }}
                  />
                </div>
              </div>
              {calendars.map((calendar, index) => {
                return <p key={`calendar-m${index}`}>{calendar.name}</p>;
              })}
            </div>
          )}
        </div>
        <CalendarView view={view} month={month} year={year} modal={modal} setModal={setModal} />
      </div>
      {modal == 'CreateEvent' && <Modal title="Create event" component={<CreateEvent month={2} year={2025}/>} setModal={setModal} />}
      {modal == 'CreateCaledar' && <Modal title="Create calendar" component={<CreateCalendar />} setModal={setModal} />}
    </div>
  );
}
