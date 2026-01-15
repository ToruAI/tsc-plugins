import { http, HttpResponse, delay } from 'msw';
import {
  mockTimers,
  mockAvailableTimers,
  mockSettings,
  mockHistoryByTimer,
  getMockExecutionDetails,
} from './data';
import type { TimerStatus, Settings } from '../types';

const BASE = '/api/plugins/route/systemd-timers';

// In-memory state for mock interactions
let state = {
  timers: [...mockTimers] as TimerStatus[],
  settings: { ...mockSettings } as Settings,
};

export const handlers = [
  // GET /timers - list watched timers with status
  http.get(`${BASE}/timers`, async () => {
    await delay(150);
    const watchedTimers = state.timers.filter(t =>
      state.settings.watched_timers.includes(t.name)
    );
    return HttpResponse.json(watchedTimers);
  }),

  // GET /timers/available - list all systemd timers
  http.get(`${BASE}/timers/available`, async () => {
    await delay(150);
    return HttpResponse.json(mockAvailableTimers);
  }),

  // GET /timers/settings - get current settings
  http.get(`${BASE}/timers/settings`, async () => {
    await delay(100);
    return HttpResponse.json(state.settings);
  }),

  // POST /timers/settings - save settings
  http.post(`${BASE}/timers/settings`, async ({ request }) => {
    await delay(200);
    const body = await request.json() as { watched_timers: string[] };
    state.settings.watched_timers = body.watched_timers;
    console.log('[MSW] Settings saved:', body.watched_timers);
    return HttpResponse.json({ success: true });
  }),

  // POST /timers/:name/run - run timer now
  http.post(`${BASE}/timers/:name/run`, async ({ params }) => {
    await delay(300);
    const name = params.name as string;
    console.log('[MSW] Running timer:', name);

    // Update timer to running state
    state.timers = state.timers.map(t =>
      t.name === name
        ? { ...t, last_result: 'running' as const, last_run: new Date().toISOString() }
        : t
    );

    // Simulate completion after 3 seconds
    setTimeout(() => {
      state.timers = state.timers.map(t =>
        t.name === name && t.last_result === 'running'
          ? { ...t, last_result: 'success' as const }
          : t
      );
    }, 3000);

    return HttpResponse.json({ success: true });
  }),

  // POST /timers/:name/test - test run timer
  http.post(`${BASE}/timers/:name/test`, async ({ params }) => {
    await delay(300);
    const name = params.name as string;
    console.log('[MSW] Testing timer:', name);

    state.timers = state.timers.map(t =>
      t.name === name
        ? { ...t, last_result: 'running' as const, last_run: new Date().toISOString() }
        : t
    );

    setTimeout(() => {
      state.timers = state.timers.map(t =>
        t.name === name && t.last_result === 'running'
          ? { ...t, last_result: 'success' as const }
          : t
      );
    }, 2000);

    return HttpResponse.json({ success: true });
  }),

  // POST /timers/:name/enable
  http.post(`${BASE}/timers/:name/enable`, async ({ params }) => {
    await delay(150);
    const name = params.name as string;
    state.timers = state.timers.map(t =>
      t.name === name ? { ...t, enabled: true } : t
    );
    console.log('[MSW] Enabled timer:', name);
    return HttpResponse.json({ success: true });
  }),

  // POST /timers/:name/disable
  http.post(`${BASE}/timers/:name/disable`, async ({ params }) => {
    await delay(150);
    const name = params.name as string;
    state.timers = state.timers.map(t =>
      t.name === name ? { ...t, enabled: false } : t
    );
    console.log('[MSW] Disabled timer:', name);
    return HttpResponse.json({ success: true });
  }),

  // GET /timers/:name/history
  http.get(`${BASE}/timers/:name/history`, async ({ params, request }) => {
    await delay(200);
    const name = params.name as string;
    const url = new URL(request.url);
    const limit = parseInt(url.searchParams.get('limit') || '20', 10);

    const history = mockHistoryByTimer[name] || [];
    return HttpResponse.json(history.slice(0, limit));
  }),

  // GET /timers/:name/history/:id
  http.get(`${BASE}/timers/:name/history/:id`, async ({ params }) => {
    await delay(150);
    const name = params.name as string;
    const id = params.id as string;

    const details = getMockExecutionDetails(name, id);
    if (!details) {
      return HttpResponse.json({ error: 'Not found' }, { status: 404 });
    }
    return HttpResponse.json(details);
  }),
];
