import type { SvelteComponent } from "svelte";

export interface IRoute {
    path: string;
    name: string;
    component: any;
    props?: object | undefined;
    isProtected?: boolean;
};