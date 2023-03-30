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

export interface IUser {
    _key: string;
    _rev: string;
    _id: string;
    first_name: string;
    last_name: string;
    email: string;
    password: string;
}

const authStore = writable<IUser>(null);

export default authStore;