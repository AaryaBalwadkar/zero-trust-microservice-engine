import { writable } from 'svelte/store';

export type NavigationSection =
	| 'dashboard'
	| 'services'
	| 'policies'
	| 'mesh'
	| 'attacks'
	| 'audit'
	| 'settings';

export const currentSection = writable<NavigationSection>('dashboard');
