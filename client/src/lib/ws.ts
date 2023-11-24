import { appConstants as c } from './constants';
import { type IMessageEvent, w3cwebsocket as W3CWebSocket } from 'websocket';
import { log } from './util';
import { MessageToClientType } from './types';

export const wsConnect = (
  // dispatch: any,
  // setShowModal: Function,
) => {
  // prepare wsUri
  const client = new W3CWebSocket(c.VALUES.apiUrlWs as string);
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
      wsConnect(
        // dispatch, setShowModal
      );
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
