const VALUES: { [key: string]: unknown } = {
  appName: 'sveltekit-websockets-poc',
  apiUrl: `https://localhost:8080/api`,
  apiUrlWs: `ws://localhost:8080/ws/`,
  apiHeaders: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer SECRET-CAN-BE-EXPOSED',
  },
  showDebugInConsoleLog: true,
  wsReconnectTimeout: 1500,
};

const I18N: { [key: string]: unknown } = {
  milliSeconds: 'milliSeconds'
};

export const appConstants = {
  VALUES,
  I18N,
};
