import { appConstants as c } from './constants';

export const log = (message: unknown, context?: string) => {
  const showDebugInConsoleLog = (c.VALUES.showDebugInConsoleLog) ? true : false;
  if (showDebugInConsoleLog) {
    let outMessage = message;
    if (typeof message === 'object') {
      outMessage = JSON.stringify(outMessage, undefined, 2);
    }
    const outContext = context ? `[${context}] : ` : '';
    console.log(`${outContext}${outMessage}`);
  }
}
