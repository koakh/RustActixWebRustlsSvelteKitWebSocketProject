export enum MessageToClientType {
  Echo = 'echo',
  CardInserted = 'card-inserted',
  CardInsertedData = 'card-inserted-data',
  CardRemoved = 'card-removed',
  CitizenCreated = 'citizen-created',
  CitizenUpdated = 'citizen-updated',
  CitizenForgotten = 'citizen-forget',
  CitizenResetPassword = 'citizen-reset-password',
  HealthCheckIdentityServerApiResponse = 'health-check-identity-server-api-response',
  FailConnectToIdentityServerApi = 'fail-connect-to-identity-server-api',
}

export enum StandByMode {
  INSERT_CARD = 'INSERT_CARD',
  READING_CARD = 'reading_card',
  RETRY_INSERT_CARD = 'RETRY_INSERT_CARD',
}
