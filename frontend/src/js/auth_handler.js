let supabase;

export function init_supabase(supabase_url, anon_key) {
  if (supabase == undefined) supabase = window.supabase.createClient(supabase_url, anon_key);
}

export async function handle_login(email, password) {
  const { error } = await supabase.auth.signInWithPassword({
    email,
    password,
  })

  return `${error}`;
}

export async function handle_signup(email, password) {
  const { error } = await supabase.auth.signUp({
    email,
    password,
  })

  return `${error}`;
}

export async function get_session() {
  const { data, error } = await supabase.auth.getSession();

  if (error || !data.session?.user) {
    window.location.href = '/login';
  } else {
    return data.session.access_token;
  }
}

export async function get_email() {
  const { data, error } = await supabase.auth.getSession();

  if (error || !data.session?.user) {
    window.location.href = '/login';
  } else {
    return data.session.user.email;
  }
}

export async function handle_signout() {
  const { error } = await supabase.auth.signOut();

  if (error) {
    console.log(error);
  } else {
    window.location.href = '/login';
  }
}
