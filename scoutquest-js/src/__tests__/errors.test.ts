import { ScoutQuestError, isScoutQuestError } from '../errors';

describe('ScoutQuestError', () => {
  describe('constructor', () => {
    it('should create error with default values', () => {
      const error = new ScoutQuestError('Test message');

      expect(error.message).toBe('Test message');
      expect(error.code).toBe('UNKNOWN_ERROR');
      expect(error.name).toBe('ScoutQuestError');
      expect(error.statusCode).toBeUndefined();
      expect(error.details).toBeUndefined();
    });

    it('should create error with all parameters', () => {
      const details = { extra: 'info' };
      const error = new ScoutQuestError(
        'Test message',
        'TEST_ERROR',
        400,
        details
      );

      expect(error.message).toBe('Test message');
      expect(error.code).toBe('TEST_ERROR');
      expect(error.statusCode).toBe(400);
      expect(error.details).toBe(details);
    });
  });

  describe('static factory methods', () => {
    it('should create network error', () => {
      const error = ScoutQuestError.network('Network failed');

      expect(error.message).toBe('Network failed');
      expect(error.code).toBe('NETWORK_ERROR');
    });

    it('should create timeout error', () => {
      const error = ScoutQuestError.timeout('Request timeout');

      expect(error.message).toBe('Request timeout');
      expect(error.code).toBe('TIMEOUT_ERROR');
    });

    it('should create service not found error', () => {
      const error = ScoutQuestError.serviceNotFound('my-service');

      expect(error.message).toBe("Service 'my-service' not found");
      expect(error.code).toBe('SERVICE_NOT_FOUND');
      expect(error.statusCode).toBe(404);
    });

    it('should create instance not found error', () => {
      const error = ScoutQuestError.instanceNotFound('instance-123');

      expect(error.message).toBe("Instance 'instance-123' not found");
      expect(error.code).toBe('INSTANCE_NOT_FOUND');
      expect(error.statusCode).toBe(404);
    });

    it('should create registration failed error', () => {
      const error = ScoutQuestError.registrationFailed('Invalid data');

      expect(error.message).toBe('Service registration failed: Invalid data');
      expect(error.code).toBe('REGISTRATION_FAILED');
      expect(error.statusCode).toBe(400);
    });

    it('should create invalid config error', () => {
      const error = ScoutQuestError.invalidConfig('Missing URL');

      expect(error.message).toBe('Invalid configuration: Missing URL');
      expect(error.code).toBe('INVALID_CONFIG');
    });
  });

  describe('fromHttpResponse', () => {
    it('should create appropriate error for different HTTP status codes', () => {
      const testCases = [
        { status: 400, expectedCode: 'BAD_REQUEST' },
        { status: 401, expectedCode: 'UNAUTHORIZED' },
        { status: 404, expectedCode: 'NOT_FOUND' },
        { status: 500, expectedCode: 'INTERNAL_SERVER_ERROR' },
        { status: 418, expectedCode: 'HTTP_ERROR' }, // Unknown status
      ];

      testCases.forEach(({ status, expectedCode }) => {
        const error = ScoutQuestError.fromHttpResponse(status, 'Test message');
        expect(error.code).toBe(expectedCode);
        expect(error.statusCode).toBe(status);
      });
    });
  });

  describe('isScoutQuestError', () => {
    it('should return true for ScoutQuestError instances', () => {
      const error = new ScoutQuestError('Test');
      expect(isScoutQuestError(error)).toBe(true);
    });

    it('should return false for regular Error instances', () => {
      const error = new Error('Test');
      expect(isScoutQuestError(error)).toBe(false);
    });

    it('should return false for non-error objects', () => {
      expect(isScoutQuestError({})).toBe(false);
      expect(isScoutQuestError('string')).toBe(false);
      expect(isScoutQuestError(null)).toBe(false);
    });
  });
});
