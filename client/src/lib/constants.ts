import { getUri as getApiUri, getWsUri } from './util';

const VALUES: { [key: string]: unknown } = {
  appName: 'ccard-identity-provider',
  apiUrl: getApiUri(),
  apiUrlWs: getWsUri(),
  apiHeaders: {
    'Content-Type': 'application/json',
    // 'Authorization': `Bearer ${process.env.REACT_APP_HTTP_SERVER_API_KEY}`,
    'Authorization': 'Bearer Uffpwzm5Ahx5zWVWi6H0LZnQnmYA4uelif2U54ATqDO0rORRMQRvnA1zuAnQkKU3bC6T4RI9EghIcwBaXKkenkT0t9jVdoaAMXQsKjFFGDn7oSvfTcSaU5YYKtY1ydwn',
  },
  // showDebugInConsoleLog: (process.env.REACT_APP_SHOW_DEBUG_IN_CONSOLE_LOG || false),
  showDebugInConsoleLog: true,
  wsReconnectTimeout: 1500,
  activationUrl: 'https://activation.kuartzo.com/activation?code={CODE}',
};

const I18N: { [key: string]: unknown } = {
  // keywords
  undefined: 'Undefined',
  error: 'Error',
  success: 'Success',
  milliSeconds: 'ms',
  buttonLabels: {
    register: 'register',
    update: 'update',
    forget: 'forget',
    recover: 'recover',
    resetPassword: 'reset password',
    close: 'close',
    showCredentialsQRCode: 'qrcode',
    showCredentials: 'credentials',
  },
  text: {
    standByModeInsertCard: 'Please insert a citizen card',
    standByModeReadingCard: 'Reading citizen card',
    standByRetryInsertCard: 'Fail connecting server, try again',
    footer: 'Kuartzo Identity Provider v0.3 : kuartzo.com'
  },
  textLabels: {
    username: 'username',
    password: 'password',
    activationCode: 'activationCode',
    activationUrl: 'activationUrl',
  },
  modalTitles: {
    // not used, use a lot of space
    // loginDetails: 'Activate login',
  },
  modalText: {
    scanQrCodeToGoToActivationPage: 'scan qrcode to go to activation page',
    useAboveDetailsToActivateLogin: 'use above details to activate login',
  }
};

export const appConstants = {
  VALUES,
  I18N,
};
