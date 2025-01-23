'use client';

import { useState } from 'react';
import { login, signup } from './actions';

import Button from '../components/button';
import InputField from '../components/inputField';
import Status from '../components/status';

function isPasswordValid(password: string): boolean {
  const hasLowercase = /[a-z]/.test(password);
  const hasUppercase = /[A-Z]/.test(password);
  const hasDigit = /\d/.test(password);
  const hasSymbol = /[!@#$%^&*(),.?":{}|<>_\-+=~`[\]\\;/]/.test(password);

  return password.length >= 8 && hasLowercase && hasUppercase && hasDigit && hasSymbol;
}

export default function LoginPage() {
  const [status, setStatus] = useState({ code: 'ok', data: '' });

  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');

  const isValidLogin = (): boolean => {
    if (email == '' || password == '') {
      setStatus({
        code: 'error',
        data:
          'the following fields are not filled out:\n' +
          (email == '' ? ' email\n' : '') +
          (password == '' ? ' password' : '')
      });

      return false;
    }

    if (!isPasswordValid(password)) {
      setStatus({
        code: 'error',
        data: 'your password must contain: \none uppercase letter\none lowercase letter\none digit\none special charecter'
      });

      return false;
    }

    setStatus({ code: 'ok', data: '' });
    return true;
  };

  const handleLogin = () => {
    if (!isValidLogin()) return;
    login({ email, password });
  };

  const handleSignup = () => {
    if (!isValidLogin()) return;
    signup({ email, password });
  };

  return (
    <div className="flex flex-col p-5 h-screen max-h-screen items-center justify-center bg-white dark:bg-gray-950">
      <div className="flex flex-col max-w-xs">
        <a className="text-2xl pb-1">Email</a>
        <InputField testId="cypress-email" type="email" value={email} setValue={setEmail} />
        <a className="text-2xl pb-1">Password</a>
        <InputField testId="cypress-password" type="password" value={password} setValue={setPassword} />
        {status.code !== 'ok' && <Status status={status} />}
        <div className="flex flex-col">
          <Button testId="cypress-login" text={'Log in'} style="primary" width="w-full" onClick={handleLogin} />
          <Button testId="cypress-signup" text={'Sign up'} style="primary" width="w-full" onClick={handleSignup} />
        </div>
      </div>
    </div>
  );
}
