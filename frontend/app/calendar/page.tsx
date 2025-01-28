import Calendar from './calendar';

export default function Main() {
  const baseUrl = process.env.MODE == 'production' ? '/api/' : 'http://127.0.0.1:3080/api/';

  return <Calendar baseUrl={baseUrl} />;
}
