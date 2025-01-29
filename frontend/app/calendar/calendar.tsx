'use client';

import { useEffect, useState } from 'react';
import { useRouter } from 'next/navigation';
import { createClient } from '@/utils/supabase/client';
import type { User } from '@supabase/supabase-js';

import axios from 'axios';

export default function Calendar({ baseUrl }: { baseUrl: string }) {
  const router = useRouter();
  const [user, setUser] = useState<User | null>(null);
  const [token, setToken] = useState('');

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
        const response = await axios.get('http://localhost:3080/api/getUserData', {
          headers: {
            'Content-Type': 'application/json',
            Authorization: token
          }
        });

        console.log(response.data);
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

  return (
    <div className="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
      <a className="text-2xl">Hello {user?.email}</a>
    </div>
  );
}
