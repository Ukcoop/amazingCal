import { redirect } from 'next/navigation';

import { createClient } from '@/utils/supabase/server';

export default async function PrivatePage() {
  const supabase = await createClient();

  const { data, error } = await supabase.auth.getUser();
  if (error || !data?.user) {
    redirect('/login');
  }

  return (
    <div className="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
      <a className="text-2xl">Hello {data.user.email}</a>
    </div>
  );
}
