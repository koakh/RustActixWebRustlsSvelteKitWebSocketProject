export interface CardData {
  givenName: string,
  documentType: string,
  documentVersion: string,
  documentNumber: string,
  localOfRequest: string,
  issuingEntity: string,
  validityBeginDate: string,
  validityEndDate: string,
  documentPan: string,
  civilianIdNumber: string,
  taxNo: string,
  socialSecurityNumber: string,
  healthNumber: string,
  parents: string,
  givenNameFather: string,
  givenNameMother: string,
  accidentalIndications: string,
  nationality: string,
  country: string,
  dateOfBirth: string,
  height: string,
  gender: string,
  mRz1: string,
  mRz2: string,
  mRz3: string,
}

export interface Card {
  cardData: CardData,
  base64Image: string,
  // cardIdentityServerData
  id: string;
  exists: boolean,
  updated: boolean,
  forgotten: boolean,
}

export interface CardIdentityServerData {
  id: string;
  exists: boolean,
  updated: boolean,
  forgotten: boolean,
}