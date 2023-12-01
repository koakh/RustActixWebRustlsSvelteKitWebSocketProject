import { log } from '$lib';
import { MessageToClientType } from '$lib/types';
import { writable } from 'svelte/store';
import { type IMessageEvent, type IStringified, w3cwebsocket as W3CWebSocket } from 'websocket';
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
      webSocketVersion: 13,
      tlsOptions: {
        agent: true,
        rejectUnauthorized: false,
        // servername: 'localhost',
//         cert: `-----BEGIN CERTIFICATE-----
// MIIEJzCCAo+gAwIBAgIQKu5MWHrdyO4HsnfIu8alTDANBgkqhkiG9w0BAQsFADBt
// MR4wHAYDVQQKExVta2NlcnQgZGV2ZWxvcG1lbnQgQ0ExITAfBgNVBAsMGHJvYkBz
// b21icmEueDUyLmRldiAoUm9iKTEoMCYGA1UEAwwfbWtjZXJ0IHJvYkBzb21icmEu
// eDUyLmRldiAoUm9iKTAeFw0yMTEwMDYyMTMxMzNaFw0yNDAxMDYyMjMxMzNaMEwx
// JzAlBgNVBAoTHm1rY2VydCBkZXZlbG9wbWVudCBjZXJ0aWZpY2F0ZTEhMB8GA1UE
// CwwYcm9iQHNvbWJyYS54NTIuZGV2IChSb2IpMIIBIjANBgkqhkiG9w0BAQEFAAOC
// AQ8AMIIBCgKCAQEAoI9BHaflPrNfnGKO6WmaEwhXfKKBH9sWlo4NKdP9ECZTC2Ef
// ubvQzhjcJsPWIwYj1NDiAa11WfD6ayKG7YleoNynsDKnsOEBfXtFHU2IPWaESX4Q
// rO8OaTXx001qdjwE3j/+K0AD43umXdnCeks3JYYlyG4/XxKa62pmpwu6KMgKbygA
// MS3dIMe7WcYbKX+qPNl4xoF5xkeqlp2urO3SWPkgIYB+cDNsWRHb5vsMWw9s7Zos
// W4mWAPZz0bLKw6w6imfo0rq0j5aoPJLNAyuH3/qhZIZC13tUCAxymIq0+pCeO+lZ
// f0OC05dB/Hw1zSLxAxHgDzpOsaq9/NXSkIwEzwIDAQABo2QwYjAOBgNVHQ8BAf8E
// BAMCBaAwEwYDVR0lBAwwCgYIKwYBBQUHAwEwHwYDVR0jBBgwFoAUto/ox0MqZShm
// QpViV/gjfJKrMDkwGgYDVR0RBBMwEYIJbG9jYWxob3N0hwR/AAABMA0GCSqGSIb3
// DQEBCwUAA4IBgQCxMiND9F3xsyGlIyqIgHc+fp+wzFI5Yz9qD/02RP558qXHAj2o
// 6zGECzc4PeiBLh7Y7wHjcu4TTLIXnRtrdVFBUT/s58l/uoK8NGVjky74Rc4A+djt
// zwcHS0snuj+FJ859Y+uS3rGKAmBAKWD22wmhB96UNRiZjG1QdJ/Or6hMZ3PVbELs
// Hgv69UG1jJiL8y7cn4foBXC6Wgb10tPXNoz7TpD3B14+Pd82yergAHswCp3nj9Ip
// D+9Ohko26OItO1dJYeDZWi0CurWdjP7xnEsZo2OaLIlSMiUbSyJOCMk/xWJCjuLW
// BEc1VzaFwhkGZJUa1F6TOIc70geLC4wQWOaqZoLbsQfihYgRoUMZJOmjcDXJrNZz
// wZofnBI+0tDsZfKjwXFyA4bzUD1I3lFY5Zy3wgQprUrZCm69uo8G4RtMWP9DmXCc
// SEw6CxBVPu/l/ljYoxdqCyJTLvdQ97OlGgLv3b0DDcWqi7e0zB8NqT0aCTPm7J/M
// OBWicNgMJ+1qL8M=
// -----END CERTIFICATE-----`,
//         key: `-----BEGIN PRIVATE KEY-----
// MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCgj0Edp+U+s1+c
// Yo7paZoTCFd8ooEf2xaWjg0p0/0QJlMLYR+5u9DOGNwmw9YjBiPU0OIBrXVZ8Ppr
// IobtiV6g3KewMqew4QF9e0UdTYg9ZoRJfhCs7w5pNfHTTWp2PATeP/4rQAPje6Zd
// 2cJ6SzclhiXIbj9fEprramanC7ooyApvKAAxLd0gx7tZxhspf6o82XjGgXnGR6qW
// na6s7dJY+SAhgH5wM2xZEdvm+wxbD2ztmixbiZYA9nPRssrDrDqKZ+jSurSPlqg8
// ks0DK4ff+qFkhkLXe1QIDHKYirT6kJ476Vl/Q4LTl0H8fDXNIvEDEeAPOk6xqr38
// 1dKQjATPAgMBAAECggEAVBfTvgmSuw1NtWW1fjDuHqvOzpt6T8n7Aa2y3UaHk67O
// 7fXXnPruuRMyMyd8/2kW2T7yMHi+LvZU4kn6K204X75SIanWRIEEu8kVgOx7v9Ty
// 0l8xsrGedaJoXwh8CyMSValkoRhtMPcxQpRsFItSfdfN8DU2AcCH3WckDrfIr9SJ
// qvag8VsYeg/PH3rP3bNAh4xousaJzcvr8ifuNcN7NmoUDMoTXk3Pxhxeryj+sACS
// cFxt777edShuYqL2BAziY/cTl0zcvCarX27NUS+q9exF7VYvMCuqiWHYcYkLlkH1
// UfrwPXQmdX5/CUBqt36xBsKyub5j74KoEk7shzOmkQKBgQDKTr0vc+53QNUR1mUD
// 7a8Pw+oWW1ddcd9SYtvzEJeNqb7s2aZsEzTRk4Pxdx3wrm8PAaPqjzJWwx1SmazU
// iLt55SRFu3sPw8gTwNQj01fy2roae/ZzMP4MJRzw6vFtNPPcevLQK9JN9uKBQep+
// NU3xHYNYnT2I+X7QVJi6AsMwxwKBgQDLLA6iOwN+3aQmLlW1A4reRpIkFQ75RD92
// BtCnYQwXCqOtU4uUz3fIlmcuCI5jhqAYWG0m9IL+rxQD2SdFu9UaG1pEsMkapjUh
// +mPLAm3UcoqnhKygGiiQ8iPL9zMFai3dfbBYrmBMsYgFxT7wkPuAgjWM0bvfyUqA
// lwKrkykTuQKBgHdSZacdW6MerA0vRLlCcSR9Sw4QpcDJrwwqnswIFztIyQFthgjs
// cxTBSusadKBGYd6Z+xIXj3s47YyQcy2Pz/OfQPuYDodH1DRCYV0YBCGK/IUuZDeg
// x9Zl9WHrUKY2uzZpldlOX2X4nbPbKvFxgx0ZaSTU6Txm23MI0mOzyWh1AoGBAJYu
// jvKkpMTWmUwP3BLd93yutcAuQM9I/5ADIaFYP1OY7bxlkTwC0AxaARMqB/bRwO2+
// D5FIFLymNilSD5GgcrnFlkhIVZ95VLU1HScnOIBd2thRXjlKnMnn80YGCJTsE9Mx
// 4XTsEQsf/+gkEY5J3V704RiiwDl/1a6P8c1aDnchAoGALEDzByXeADMiYjKi6M19
// 1WK3+TDD9Sy8fu4x2qmTho9Z9nk5bw6ZPHbXDTaQ+jxnOD4Io6iZIQLEYMwzbXnO
// 951+ck9E5mwWo/IyNROOMo0aNT9yqLANu5Hp1CliQ5Yqmb1R1Qhuk4SZTWmUGjo/
// 3I+uWHi2Foc2FU8LSAb4hLk=
// -----END PRIVATE KEY-----`,
      }
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
            // log(`ws: Echo: [${JSON.stringify(data, undefined, 0)}]`);
            log(`ws: Echo: [${JSON.stringify(data, undefined, 0)}]`);
            break;
          case MessageToClientType.CardInserted:
            log('ws: CardInserted');
            // dispatch({ type: ActionType.CARD_INSERTED, payload: null });
            break;
          case MessageToClientType.CardInsertedData:
            log('ws: CardInsertedData');
            log(`ws: CardInsertedData : [${JSON.stringify({ ...data, base64Image: undefined }, undefined, 2)}]`);
            // setCard(data);
            // dispatch({ type: ActionType.CARD_INSERTED_DATA, payload: data });
            break;
          case MessageToClientType.CardRemoved:
            log('ws: CardRemoved');
            // setCard(null);
            // dispatch({ type: ActionType.CARD_REMOVED, payload: null });
            // setShowModal(false);
            break;
          case MessageToClientType.CitizenCreated:
            log('ws: CitizenCreated');
            // dispatch({ type: ActionType.CITIZEN_CREATED, payload: data });
            break;
          case MessageToClientType.CitizenUpdated:
            log('ws: CitizenUpdated');
            // dispatch({ type: ActionType.CITIZEN_UPDATED, payload: data });
            break;
          case MessageToClientType.CitizenForgotten:
            log('ws: CitizenForgotten');
            // dispatch({ type: ActionType.CITIZEN_FORGOTTEN, payload: data });
            break;
          case MessageToClientType.CitizenResetPassword:
            log('ws: CitizenResetPassword');
            /// dispatch({ type: ActionType.CITIZEN_RESET_PASSWORD, payload: data });
            break;
          case MessageToClientType.HealthCheckIdentityServerApiResponse:
            log(`ws: HealthCheckIdentityServerApiResponse: [${JSON.stringify(data, undefined, 0)}]`);
            // dispatch({ type: ActionType.CHANGE_CONNECTED_STATUS, payload: data });
            break;
          case MessageToClientType.FailConnectToIdentityServerApi:
            log(`ws: FailConnectToIdentityServerApi : [${JSON.stringify(data, undefined, 2)}]`);
            // dispatch({ type: ActionType.CARD_INSERTED_DATA_TRY_AGAIN, payload: data });
            break;
          default:
            // reach here in heartBeat
            log(`ws: unknown msgType: '${msgType}'`);
            break;
        }
      }
      // debug block
      // if (typeof message.data === 'string') {
      //   log(`ws: Received string: ${message.data}`);
      // } else if (typeof message.data === 'object') {
      //   log(`ws: Received object: ${JSON.stringify(message.data, undefined, 2)}`);
      // } else {
      //    log('ws: Received other:', message.data);
      // };
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