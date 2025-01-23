import Link from './components/link';

export default function Home() {
  return (
    <div className="flex flex-col p-5 h-screen max-h-screen bg-white dark:bg-gray-950">
      <a className="text-2xl">Hello there, this is the homepage of amazingCal.</a>
      <div className="flex mt-2">
        <Link testId="cypress-signup" text="login" style="primary" href="/login" />
        <Link testId="cypress-calendar" text="calendar" style="primary" href="/calendar" />
      </div>
    </div>
  );
}
