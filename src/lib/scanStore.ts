// Shared scan state store — used by both layout and page to show live scan status
import { writable } from 'svelte/store';
import type { ServiceScanSummary } from './api';

export interface ScanState {
	lastScanSummary: ServiceScanSummary | null;
	lastScanTime: string | null;
	isScanning: boolean;
}

export const scanStore = writable<ScanState>({
	lastScanSummary: null,
	lastScanTime: null,
	isScanning: false
});
