/**
 * Custom error class for ScoutQuest SDK errors.
 */
export class ScoutQuestError extends Error {
  public readonly code: string;
  public readonly statusCode?: number;
  public readonly details?: any;

  constructor(
    message: string,
    code: string = 'UNKNOWN_ERROR',
    statusCode?: number,
    details?: any
  ) {
    super(message);
    this.name = 'ScoutQuestError';
    this.code = code;
    this.statusCode = statusCode;
    this.details = details;

    // Maintains proper stack trace for where our error was thrown (only available on V8)
    if ((Error as any).captureStackTrace) {
      (Error as any).captureStackTrace(this, ScoutQuestError);
    }
  }

  /**
   * Creates a network-related error.
   */
  static network(message: string, details?: any): ScoutQuestError {
    return new ScoutQuestError(message, 'NETWORK_ERROR', undefined, details);
  }

  /**
   * Creates a timeout error.
   */
  static timeout(message: string): ScoutQuestError {
    return new ScoutQuestError(message, 'TIMEOUT_ERROR');
  }

  /**
   * Creates a service not found error.
   */
  static serviceNotFound(serviceName: string): ScoutQuestError {
    return new ScoutQuestError(
      `Service '${serviceName}' not found`,
      'SERVICE_NOT_FOUND',
      404
    );
  }

  /**
   * Creates an instance not found error.
   */
  static instanceNotFound(instanceId: string): ScoutQuestError {
    return new ScoutQuestError(
      `Instance '${instanceId}' not found`,
      'INSTANCE_NOT_FOUND',
      404
    );
  }

  /**
   * Creates a registration failed error.
   */
  static registrationFailed(message: string, details?: any): ScoutQuestError {
    return new ScoutQuestError(
      `Service registration failed: ${message}`,
      'REGISTRATION_FAILED',
      400,
      details
    );
  }

  /**
   * Creates an invalid configuration error.
   */
  static invalidConfig(message: string): ScoutQuestError {
    return new ScoutQuestError(
      `Invalid configuration: ${message}`,
      'INVALID_CONFIG'
    );
  }

  /**
   * Creates an HTTP error from response.
   */
  static fromHttpResponse(
    status: number,
    message: string,
    details?: any
  ): ScoutQuestError {
    let code = 'HTTP_ERROR';
    
    switch (status) {
      case 400:
        code = 'BAD_REQUEST';
        break;
      case 401:
        code = 'UNAUTHORIZED';
        break;
      case 403:
        code = 'FORBIDDEN';
        break;
      case 404:
        code = 'NOT_FOUND';
        break;
      case 409:
        code = 'CONFLICT';
        break;
      case 422:
        code = 'VALIDATION_ERROR';
        break;
      case 429:
        code = 'RATE_LIMITED';
        break;
      case 500:
        code = 'INTERNAL_SERVER_ERROR';
        break;
      case 502:
        code = 'BAD_GATEWAY';
        break;
      case 503:
        code = 'SERVICE_UNAVAILABLE';
        break;
      case 504:
        code = 'GATEWAY_TIMEOUT';
        break;
    }

    return new ScoutQuestError(message, code, status, details);
  }
}

/**
 * Type guard to check if an error is a ScoutQuestError.
 */
export function isScoutQuestError(error: any): error is ScoutQuestError {
  return error instanceof ScoutQuestError;
}
