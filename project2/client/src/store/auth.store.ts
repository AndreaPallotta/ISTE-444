import { writable } from 'svelte/store';

export interface ISignIn {
    email: string;
    password: string;
}

export interface ISignUp {
    first_name: string;
    last_name: string;
    email: string;
    password: string;
}

export const signIn = writable<ISignIn>({
    email: '',
    password: ''
});

export const signUp = writable<ISignUp>({
    first_name: '',
    last_name: '',
    email: '',
    password: ''
});