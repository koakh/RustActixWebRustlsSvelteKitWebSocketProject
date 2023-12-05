import { log } from '$lib';
import { MessageToClientType } from '$lib/types';
import { writable } from 'svelte/store';
import { w3cwebsocket as W3CWebSocket, type IMessageEvent, type IStringified } from 'websocket';
import { appConstants as c } from '../constants';

const createWebSocket = () => {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { subscribe, set } = writable<W3CWebSocket | null>(null);

  // eslint-disable-next-line prefer-const
  let client: W3CWebSocket;

  // internal function
  const wsConnect = () => {
    // prepare wsUri
    client = new W3CWebSocket(c.VALUES.apiUrlWs as string, undefined, undefined, undefined, {}, {
      // Disable certificate validation
      // webSocketVersion: 13,
      // tlsOptions: {
      //   agent: true,
      //   rejectUnauthorized: false,
      // }
    });
    log(`ws: wsUri: ${c.VALUES.apiUrlWs}`);

    client.onopen = () => {
      log('ws: WebSocket Client Connected');
      const sendNumber = () => {
        if (client.readyState === client.OPEN) {
          const number = Math.round(Math.random() * 0xFFFFFF);
          client.send(number.toString());
          setTimeout(sendNumber, 1000);
        }
      }
      sendNumber();
    };
    client.onclose = (error) => {
      log(`ws: Socket is closed. Reconnect will be attempted in ${c.VALUES.wsReconnectTimeout} ${c.I18N.milliSeconds}.`, error.reason);
      setTimeout(() => {
        // call itSelf
        wsConnect();
      }, c.VALUES.wsReconnectTimeout as number);
    };
    client.onerror = (error) => {
      log('ws: Connection Error', error.message);
    };
    client.onmessage = (message: IMessageEvent) => {
      const { msg_type: msgType, data }: { msg_type: string, data: unknown } = JSON.parse(message.data.toString());
      if (data) {
        // log(`ws: msgType: [${msgType}]`);
        switch (msgType) {
          case MessageToClientType.Echo:
            log(`ws: Echo: [${JSON.stringify(data, undefined, 0)}]`);
            break;
          default:
            // reach here in heartBeat
            log(`ws: unknown msgType: '${msgType}'`);
            break;
        }
      }
    };
  }

  return {
    subscribe,
    connect: () => {
      wsConnect();
    },
    send: (message: ArrayBufferView | ArrayBuffer | Buffer | IStringified) => {
      if (client && client.OPEN) {
        client.send(message);
      } else {
        console.warn('WebSocket connection not open.');
      }
    },
    close: () => {
      if (client) {
        client.close();
      }
    },
  };
};

export const websocket = createWebSocket();