/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */

/**
 * These codes provide more granular error information beyond just the HTTP
 * status code of the response.
 */
export enum AptosErrorCode {
    UNSUPPORTED_ACCEPT_TYPE = 'UnsupportedAcceptType',
    READ_FROM_STORAGE_ERROR = 'ReadFromStorageError',
    INVALID_BCS_IN_STORAGE_ERROR = 'InvalidBcsInStorageError',
    BCS_SERIALIZATION_ERROR = 'BcsSerializationError',
    INVALID_START_PARAM = 'InvalidStartParam',
    INVALID_LIMIT_PARAM = 'InvalidLimitParam',
}
