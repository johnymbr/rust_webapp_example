pub struct ApiErrorCode(pub &'static str, pub &'static str);

// Generic errors.
pub const ERR_DB_CONNECTION_ERROR: ApiErrorCode =
    ApiErrorCode("G9000", "Error when connecting to database.");
pub const ERR_DB_TRANSACTION_ERROR: ApiErrorCode =
    ApiErrorCode("G9001", "Error when trying an operation into database.");
pub const ERR_HYPER_ERROR: ApiErrorCode = ApiErrorCode("G0000", "Error when forwarding a request.");
pub const ERR_INVALID_REQUEST: ApiErrorCode = ApiErrorCode("G0001", "Invalid request");
pub const ERR_INVALID_EMAIL_REGEX: ApiErrorCode = ApiErrorCode("G0002", "Email regex is invalid.");
pub const ERR_TEMPLATE_ERROR: ApiErrorCode = ApiErrorCode("G0003", "Error to retrieve a template.");

// Pagination errors.
pub const PG_ERR_PAGE_REQUIRED: ApiErrorCode = ApiErrorCode("PG0001", "Param page is required.");
pub const PG_ERR_PAGE_SIZE_REQUIRED: ApiErrorCode =
    ApiErrorCode("PG0002", "Param pageSize is required.");

// Field errors.
pub const ERR_REQUIRED_FIELD: ApiErrorCode = ApiErrorCode("F0001", "This field is required.");
pub const ERR_MIN_SIZE: ApiErrorCode = ApiErrorCode(
    "F0002",
    "This field must has a minimum amount of characters.",
);
pub const ERR_INVALID_EMAIL: ApiErrorCode = ApiErrorCode("F0003", "Email isn't valid.");
pub const ERR_INVALID_PASSWORD: ApiErrorCode = ApiErrorCode("F0004", "Password isn't valid.");
pub const ERR_CONFIRM_PASSWORD_DIFF_PASSWORD: ApiErrorCode =
    ApiErrorCode("F0005", "Confirm password needs to be equal to password.");
pub const ERR_PASSWORD_HASH: ApiErrorCode = ApiErrorCode("F0006", "Error when encrypt password.");

// User errors.
pub const USR_ERR_FIND_ALL_PAGINATED: ApiErrorCode =
    ApiErrorCode("USR0001", "Error when search users with pagination");
pub const USR_ERR_FIND_BY_ID: ApiErrorCode =
    ApiErrorCode("USR0002", "Error when search an user by id.");
pub const USR_ERR_INSERTING: ApiErrorCode = ApiErrorCode("USR0003", "Error when insert an user.");
pub const USR_ERR_UPDATING: ApiErrorCode = ApiErrorCode("USR0004", "Error when update an user.");
pub const USR_ERR_DELETE: ApiErrorCode = ApiErrorCode("USR0005", "Error when delete an user.");
pub const USR_ERR_NOT_FOUND: ApiErrorCode = ApiErrorCode("USR0006", "User not found.");
pub const USR_ERR_FIND_BY_EMAIL: ApiErrorCode =
    ApiErrorCode("USR0007", "Error when search an user by email.");
pub const USR_ERR_VALIDATE_EMAIL: ApiErrorCode =
    ApiErrorCode("USR0008", "Error when validate an user email.");

// Token errors.
pub const TKN_ERR_FIND_BY_ID: ApiErrorCode =
    ApiErrorCode("TKN0001", "Error when search a token by id.");
pub const TKN_ERR_FIND_BY_TOKEN: ApiErrorCode =
    ApiErrorCode("TKN0002", "Error when search a token by token.");
pub const TKN_ERR_INSERTING: ApiErrorCode = ApiErrorCode("TKN0003", "Error when insert a token.");
pub const TKN_ERR_NOT_FOUND: ApiErrorCode = ApiErrorCode("TKN0004", "Token not found.");
pub const TKN_ERR_VALIDATING: ApiErrorCode =
    ApiErrorCode("TKN0005", "Error when validate a token.");
pub const TKN_ERR_ALREADY_VALIDATED: ApiErrorCode =
    ApiErrorCode("TKN0006", "Token already validated.");
pub const TKN_ERR_ALREADY_EXPIRED: ApiErrorCode = ApiErrorCode("TKN0007", "Token already expired.");

// Auth errors.
pub const AUTH_EMAIL_REQUIRED: ApiErrorCode = ApiErrorCode("AUTH0001", "Email is required.");
pub const AUTH_TOKEN_REQUIRED: ApiErrorCode = ApiErrorCode("AUTH0002", "Token is required.");
pub const AUTH_EMAIL_AND_PASSWORD_REQUIRED: ApiErrorCode =
    ApiErrorCode("AUTH0003", "Email and password are required.");
pub const AUTH_INVALID_LOGIN: ApiErrorCode = ApiErrorCode("AUTH0004", "Invalid email or password.");
pub const AUTH_INACTIVE_USER: ApiErrorCode = ApiErrorCode("AUTH0005", "Inactive user.");
pub const AUTH_REFRESH_TOKEN_REQUIRED: ApiErrorCode =
    ApiErrorCode("AUTH0006", "Refresh token is required.");
pub const AUTH_GRANT_TYPE_REQUIRED: ApiErrorCode =
    ApiErrorCode("AUTH0007", "Grant type is required.");
pub const AUTH_GRANT_TYPE_INVALID: ApiErrorCode = ApiErrorCode("AUTH0008", "Grant type is wrong.");
pub const AUTH_REFRESH_TOKEN_REVOKED: ApiErrorCode =
    ApiErrorCode("AUTH0009", "Refresh token already revoked.");

// Jwt errors.
pub const JWT_ERR_GENERATING_ACCESS_TOKEN: ApiErrorCode =
    ApiErrorCode("JWT0001", "Error when generate an access token.");
pub const JWT_ERR_GENERATING_REFRESH_TOKEN: ApiErrorCode =
    ApiErrorCode("JWT0002", "Error when generate a refresh token.");
pub const JWT_ERR_VALIDATION_REFRESH_TOKEN: ApiErrorCode =
    ApiErrorCode("JWT0003", "Error when validating a refresh token.");
pub const JWT_ERR_EXPIRED_REFRESH_TOKEN: ApiErrorCode =
    ApiErrorCode("JWT0004", "Refresh token already expired.");
pub const JWT_ERR_INVALID_EXPIRATION_TIMESTAMP: ApiErrorCode =
    ApiErrorCode("JWT0005", "Invalid expiration timestamp.");
pub const JWT_ACCESS_TOKEN_REQUIRED: ApiErrorCode =
    ApiErrorCode("JWT0006", "Access token required.");
pub const JWT_ERR_VALIDATION_ACCESS_TOKEN: ApiErrorCode =
    ApiErrorCode("JWT0007", "Invalid access token.");
pub const JWT_ERR_EXPIRED_ACCESS_TOKEN: ApiErrorCode =
    ApiErrorCode("JWT0008", "Access token already expired.");

// Refresh token errors.
pub const RTKN_ERR_INSERTING: ApiErrorCode =
    ApiErrorCode("RTKN0001", "Error when insert a refresh token.");
pub const RTKN_TOKEN_UUID_REQUIRED: ApiErrorCode =
    ApiErrorCode("RTKN0002", "Token uuid is required.");
pub const RTKN_ERR_FINDING_REFRESH_TOKEN: ApiErrorCode =
    ApiErrorCode("RTKN0003", "Error when finding a refresh token.");
